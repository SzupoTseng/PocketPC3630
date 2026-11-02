//! The part of FMOD 3 (`fmodce.dll`) that EverQuest for the Pocket PC uses.
//!
//! EverQuest imports 14 `FSOUND_*` functions: 8-bit PCM `.wav` effects and one
//! `EQTheme.mp3` stream. Running the real DLL needs its mixer thread polling
//! `waveOutGetPosition`, which starves the cooperative scheduler. Decoding here
//! and handing the PCM to [`pocket_kernel::AudioEngine`] avoids that and works
//! wherever the engine does. Titles using other FMOD entry points, such as
//! Xtrakt's `Create_FmodSoundDevice`, keep the ignored-DLL stub.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use pocket_kernel::{DispatchOutcome, GuestFormat, KernelError, VoiceParams};
use rmp3::{DecoderOwned, Frame};

use crate::{CallCtx, WinCeDispatcher};

/// Voice group for effects; HSS owns groups 1 and 2.
const SFX_GROUP: u32 = 3;
/// Voice group for the music stream.
const STREAM_GROUP: u32 = 4;
/// `FSOUND_LOOP_NORMAL` in a sample or stream mode word.
const FSOUND_LOOP_NORMAL: u32 = 0x2;
/// `FSOUND_ALL`, the channel number meaning every channel.
const FSOUND_ALL: i32 = -3;
/// Opaque non-NULL handles; the guest only stores and returns them.
const FIRST_HANDLE: u32 = 0x0FD0_0010;

#[derive(Clone)]
struct Clip {
    format: GuestFormat,
    samples: Arc<Vec<i16>>,
    looped: bool,
}

#[derive(Default)]
struct FmodState {
    clips: HashMap<u32, Clip>,
    by_path: HashMap<String, (GuestFormat, Arc<Vec<i16>>)>,
    next_handle: u32,
    next_channel: u32,
    sfx_volume: Option<u32>,
}

static STATE: Lazy<Mutex<FmodState>> = Lazy::new(|| Mutex::new(FmodState::default()));

pub fn register(d: &mut WinCeDispatcher) {
    let dll = "fmodce.dll";
    d.register_handler(dll, "FSOUND_Sample_Load", sample_load);
    d.register_handler(dll, "FSOUND_Sample_Free", clip_free);
    d.register_handler(dll, "FSOUND_PlaySound", play_sound);
    d.register_handler(dll, "FSOUND_Stream_OpenFile", stream_open);
    d.register_handler(dll, "FSOUND_Stream_Play", stream_play);
    d.register_handler(dll, "FSOUND_Stream_Stop", stream_stop);
    d.register_handler(dll, "FSOUND_Stream_Close", clip_free);
    d.register_handler(dll, "FSOUND_SetSFXMasterVolume", set_sfx_volume);
    d.register_handler(dll, "FSOUND_SetPaused", set_paused);
    d.register_handler(dll, "FSOUND_Init", init);
    d.register_handler(dll, "FSOUND_Close", close);
    d.register_handler(dll, "FSOUND_SetMixer", returns_one);
    d.register_handler(dll, "FSOUND_SetBufferSize", returns_one);
    d.register_handler(dll, "FSOUND_GetError", returns_zero);
}

fn returns_one(_ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    Ok(DispatchOutcome::ReturnedR0(1))
}

fn returns_zero(_ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    Ok(DispatchOutcome::ReturnedR0(0))
}

fn init(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    ctx.kernel.audio.start();
    Ok(DispatchOutcome::ReturnedR0(1))
}

fn close(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    ctx.kernel.audio.stop_voice_group(SFX_GROUP);
    ctx.kernel.audio.stop_voice_group(STREAM_GROUP);
    Ok(DispatchOutcome::ReturnedR0(0))
}

/// Decode a clip: RIFF PCM first, then MPEG audio.
fn decode(bytes: &[u8]) -> Option<(GuestFormat, Vec<i16>)> {
    if let Some(decoded) = crate::coredll::decode_pcm_wave(bytes) {
        return Some(decoded);
    }
    decode_mp3(bytes)
}

fn decode_mp3(bytes: &[u8]) -> Option<(GuestFormat, Vec<i16>)> {
    let mut decoder = DecoderOwned::new(bytes);
    let mut samples = Vec::new();
    let mut format = None;
    while let Some(frame) = decoder.next() {
        if let Frame::Audio(audio) = frame {
            format.get_or_insert(GuestFormat {
                sample_rate: audio.sample_rate(),
                channels: audio.channels(),
                bits_per_sample: 16,
            });
            samples.extend_from_slice(audio.samples());
        }
    }
    format.filter(|_| !samples.is_empty()).map(|f| (f, samples))
}

/// Load `path_p` and return a new handle, or NULL if it cannot be decoded.
fn open_clip(ctx: &mut CallCtx<'_>, path_p: u32, mode: u32) -> Result<u32, KernelError> {
    if path_p == 0 {
        return Ok(0);
    }
    let path = crate::coredll::read_cstr_string(ctx, path_p, 260)?;
    let cached = STATE.lock().unwrap().by_path.get(&path).cloned();
    let (format, samples) = match cached {
        Some(hit) => hit,
        None => {
            let decoded = crate::coredll::read_guest_file_for(ctx, &path)
                .as_deref()
                .and_then(decode);
            let Some((format, samples)) = decoded else {
                log::warn!("fmod: cannot load {path:?}");
                return Ok(0);
            };
            let entry = (format, Arc::new(samples));
            STATE.lock().unwrap().by_path.insert(path, entry.clone());
            entry
        }
    };
    let mut state = STATE.lock().unwrap();
    let handle = FIRST_HANDLE + state.next_handle * 16;
    state.next_handle += 1;
    state.clips.insert(
        handle,
        Clip {
            format,
            samples,
            looped: mode & FSOUND_LOOP_NORMAL != 0,
        },
    );
    Ok(handle)
}

/// `FSOUND_SAMPLE *FSOUND_Sample_Load(int index, const char *name, unsigned mode, ...)`.
fn sample_load(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let name = ctx.arg_u32(1)?;
    let mode = ctx.arg_u32(2)?;
    Ok(DispatchOutcome::ReturnedR0(open_clip(ctx, name, mode)?))
}

/// `FSOUND_STREAM *FSOUND_Stream_OpenFile(const char *name, unsigned mode, int length)`.
fn stream_open(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let name = ctx.arg_u32(0)?;
    let mode = ctx.arg_u32(1)?;
    Ok(DispatchOutcome::ReturnedR0(open_clip(ctx, name, mode)?))
}

fn clip_free(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let handle = ctx.arg_u32(0)?;
    STATE.lock().unwrap().clips.remove(&handle);
    Ok(DispatchOutcome::ReturnedR0(1))
}

fn start_voice(ctx: &mut CallCtx<'_>, handle: u32, group: u32) -> u32 {
    let (clip, channel, volume) = {
        let mut state = STATE.lock().unwrap();
        let Some(clip) = state.clips.get(&handle).cloned() else {
            return u32::MAX;
        };
        let channel = state.next_channel;
        state.next_channel = (state.next_channel + 1) % 32;
        let volume = if group == SFX_GROUP {
            state.sfx_volume.unwrap_or(255) as f32 / 255.0
        } else {
            1.0
        };
        (clip, channel, volume)
    };
    ctx.kernel.audio.play_voice_with(
        &clip.samples,
        clip.format,
        VoiceParams {
            looped: clip.looped,
            group,
            volume,
        },
    );
    ctx.kernel.audio.start();
    channel
}

/// `int FSOUND_PlaySound(int channel, FSOUND_SAMPLE *sample)`; -1 on failure.
fn play_sound(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let handle = ctx.arg_u32(1)?;
    Ok(DispatchOutcome::ReturnedR0(start_voice(ctx, handle, SFX_GROUP)))
}

/// `int FSOUND_Stream_Play(int channel, FSOUND_STREAM *stream)`; -1 on failure.
fn stream_play(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let handle = ctx.arg_u32(1)?;
    Ok(DispatchOutcome::ReturnedR0(start_voice(ctx, handle, STREAM_GROUP)))
}

fn stream_stop(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    ctx.kernel.audio.stop_voice_group(STREAM_GROUP);
    Ok(DispatchOutcome::ReturnedR0(1))
}

/// `void FSOUND_SetSFXMasterVolume(int volume)`, 0..255. EverQuest's options
/// slider lands here, and it applies to effects started afterwards.
fn set_sfx_volume(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let volume = ctx.arg_u32(0)?.min(255);
    STATE.lock().unwrap().sfx_volume = Some(volume);
    Ok(DispatchOutcome::ReturnedR0(0))
}

/// `signed char FSOUND_SetPaused(int channel, signed char paused)`. EverQuest
/// pauses everything when it loses focus; per-voice pausing is not modelled.
fn set_paused(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let channel = ctx.arg_u32(0)? as i32;
    let paused = ctx.arg_u32(1)? != 0;
    if channel == FSOUND_ALL {
        ctx.kernel.audio.set_paused(paused);
    }
    Ok(DispatchOutcome::ReturnedR0(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garbage_is_not_mistaken_for_audio() {
        assert!(decode(&[0u8; 512]).is_none());
    }

    /// EverQuest's effects are 22,050 Hz 8-bit mono RIFF files.
    #[test]
    fn eight_bit_wave_decodes_to_signed_sixteen_bit() {
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF\x28\0\0\0WAVEfmt \x10\0\0\0\x01\0\x01\0");
        wav.extend_from_slice(&22_050u32.to_le_bytes());
        wav.extend_from_slice(&22_050u32.to_le_bytes());
        wav.extend_from_slice(b"\x01\0\x08\0data\x04\0\0\0\x80\xff\x00\x80");
        let (format, samples) = decode(&wav).expect("decodes");
        assert_eq!(format.sample_rate, 22_050);
        assert_eq!(samples.len(), 4);
        assert_eq!(samples[0], 0);
        assert!(samples[1] > 0 && samples[2] < 0);
    }
}
