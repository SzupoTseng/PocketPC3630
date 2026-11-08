//! The part of FMOD 3 (`fmodce.dll`) that EverQuest for the Pocket PC uses.
//!
//! EverQuest imports 14 `FSOUND_*` functions: 8-bit PCM `.wav` effects and one
//! `EQTheme.mp3` stream. Running the real DLL needs its mixer thread polling
//! `waveOutGetPosition`, which starves the cooperative scheduler. Decoding here
//! and handing the PCM to [`pocket_kernel::AudioEngine`] avoids that and works
//! wherever the engine does. Titles using other FMOD entry points, such as
//! Xtrakt's `Create_FmodSoundDevice`, keep the ignored-DLL stub.

use crate::{CallCtx, WinCeDispatcher};
use pocket_kernel::fmod::{FmodChannel, FmodClip as Clip, FmodState};
use pocket_kernel::{DispatchOutcome, GuestFormat, KernelError, VoiceParams};
use rmp3::{DecoderOwned, Frame};
use std::sync::Arc;

const FSOUND_LOOP_NORMAL: u32 = 0x2;
const FSOUND_ALL: i32 = -3;
const FIRST_HANDLE: u32 = 0x0FD0_0010;
// One private mixer group per FMOD channel; HSS and ordinary voices are separate.
const CHANNEL_GROUP_BASE: u32 = 0x10000;
const CHANNELS: u32 = 32;
fn channel_group(channel: u32) -> u32 {
    CHANNEL_GROUP_BASE + channel
}

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
    for &channel in ctx.kernel.fmod.channels.keys() {
        ctx.kernel.audio.stop_voice_group(channel_group(channel));
    }
    ctx.kernel.fmod = FmodState::default();
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
    let cached = ctx.kernel.fmod.by_path.get(&path).cloned();
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
            log::debug!(
                "fmod: decoded {path:?}: {} Hz, {} ch, {} samples",
                format.sample_rate,
                format.channels,
                samples.len()
            );
            let entry = (format, Arc::new(samples));
            ctx.kernel.fmod.by_path.insert(path, entry.clone());
            entry
        }
    };
    let state = &mut ctx.kernel.fmod;
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

fn stop_clip(ctx: &mut CallCtx<'_>, handle: u32, streams_only: bool) {
    let channels: Vec<_> = ctx
        .kernel
        .fmod
        .channels
        .iter()
        .filter(|(_, c)| c.clip == handle && (!streams_only || c.stream))
        .map(|(&channel, _)| channel)
        .collect();
    for channel in channels {
        ctx.kernel.audio.stop_voice_group(channel_group(channel));
        ctx.kernel.fmod.channels.remove(&channel);
    }
}

fn clip_free(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let handle = ctx.arg_u32(0)?;
    stop_clip(ctx, handle, false);
    ctx.kernel.fmod.clips.remove(&handle);
    Ok(DispatchOutcome::ReturnedR0(1))
}

fn choose_channel(
    state: &mut FmodState,
    audio: &pocket_kernel::AudioEngine,
    requested: i32,
) -> Option<u32> {
    if requested == -1 {
        // EverQuest keeps its theme looping while effects come and go. A
        // round-robin counter alone steals the music after 32 button clicks.
        let channel = (0..CHANNELS)
            .map(|offset| (state.next_channel + offset) % CHANNELS)
            .find(|&channel| !audio.voice_group_active(channel_group(channel)))?;
        state.next_channel = (channel + 1) % CHANNELS;
        Some(channel)
    } else if (0..CHANNELS as i32).contains(&requested) {
        Some(requested as u32)
    } else {
        None
    }
}

fn start_voice(ctx: &mut CallCtx<'_>, handle: u32, mut requested: i32, stream: bool) -> u32 {
    let Some(clip) = ctx.kernel.fmod.clips.get(&handle).cloned() else {
        return u32::MAX;
    };
    if requested != -1 && !(0..CHANNELS as i32).contains(&requested) {
        return u32::MAX;
    }
    if stream {
        // A stream has one playback cursor: replaying it restarts its existing
        // channel rather than layering a second copy of the theme.
        if requested == -1 {
            if let Some((&channel, _)) = ctx
                .kernel
                .fmod
                .channels
                .iter()
                .find(|(_, c)| c.clip == handle && c.stream)
            {
                requested = channel as i32;
            }
        }
        stop_clip(ctx, handle, true);
    }
    let state = &mut ctx.kernel.fmod;
    let Some(channel) = choose_channel(state, &ctx.kernel.audio, requested) else {
        return u32::MAX;
    };
    let volume = state.sfx_volume as f32 / 255.0;
    state.channels.insert(
        channel,
        FmodChannel {
            clip: handle,
            stream,
        },
    );
    let group = channel_group(channel);
    // FSOUND_PlaySound on an occupied channel replaces that voice.
    ctx.kernel.audio.stop_voice_group(group);
    ctx.kernel.audio.play_voice_with(
        &clip.samples,
        clip.format,
        VoiceParams {
            looped: clip.looped,
            group,
            volume,
        },
    );
    log::debug!("fmod: play clip=0x{handle:08x} stream={stream} requested={requested} channel={channel} gain={volume:.3}");
    ctx.kernel.audio.start();
    channel
}

fn play_sound(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let channel = ctx.arg_u32(0)? as i32;
    let handle = ctx.arg_u32(1)?;
    Ok(DispatchOutcome::ReturnedR0(start_voice(
        ctx, handle, channel, false,
    )))
}

fn stream_play(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let channel = ctx.arg_u32(0)? as i32;
    let handle = ctx.arg_u32(1)?;
    Ok(DispatchOutcome::ReturnedR0(start_voice(
        ctx, handle, channel, true,
    )))
}

fn stream_stop(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let handle = ctx.arg_u32(0)?;
    stop_clip(ctx, handle, true);
    Ok(DispatchOutcome::ReturnedR0(1))
}

/// EverQuest's volume slider also affects effects currently playing.
fn set_sfx_volume(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let volume = ctx.arg_u32(0)?.min(255);
    ctx.kernel.fmod.sfx_volume = volume;
    // Both FSOUND samples and MP3 streams play through FSOUND channels.
    for &channel in ctx.kernel.fmod.channels.keys() {
        ctx.kernel
            .audio
            .set_voice_group_volume(channel_group(channel), volume as f32 / 255.0);
    }
    Ok(DispatchOutcome::ReturnedR0(0))
}

/// EverQuest pauses FMOD channels on focus loss; other audio transports keep
/// their own state. A single-channel pause must not freeze unrelated effects.
fn set_paused(ctx: &mut CallCtx<'_>) -> Result<DispatchOutcome, KernelError> {
    let requested = ctx.arg_u32(0)? as i32;
    let paused = ctx.arg_u32(1)? != 0;
    for &channel in ctx.kernel.fmod.channels.keys() {
        if requested == FSOUND_ALL || requested == channel as i32 {
            ctx.kernel
                .audio
                .set_voice_group_paused(channel_group(channel), paused);
        }
    }
    Ok(DispatchOutcome::ReturnedR0(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_eq_effects_never_steal_a_looping_or_paused_music_channel() {
        let audio = pocket_kernel::AudioEngine::new();
        let format = GuestFormat {
            sample_rate: 44100,
            channels: 1,
            bits_per_sample: 16,
        };
        let mut state = FmodState::default();
        audio.play_voice_with(
            &[8192],
            format,
            VoiceParams {
                looped: true,
                group: channel_group(0),
                volume: 1.0,
            },
        );
        let tap = audio.tap();
        let mut mix = [0.0; 4];
        for _ in 0..100 {
            let channel = choose_channel(&mut state, &audio, -1).unwrap();
            assert_ne!(channel, 0);
            audio.play_voice_with(
                &[4096],
                format,
                VoiceParams {
                    looped: false,
                    group: channel_group(channel),
                    volume: 1.0,
                },
            );
            tap.render_mix(&mut mix, 44100, 2);
            assert_eq!(&mix[2..], &[0.25, 0.25]);
            assert!(audio.voice_group_active(channel_group(0)));
        }
        audio.set_voice_group_paused(channel_group(0), true);
        state.next_channel = 0;
        assert_ne!(choose_channel(&mut state, &audio, -1), Some(0));
    }

    #[test]
    fn a_full_eq_mixer_reports_no_free_channel_without_stopping_a_voice() {
        let audio = pocket_kernel::AudioEngine::new();
        let mut state = FmodState::default();
        for channel in 0..CHANNELS {
            audio.play_voice_with(
                &[1, 1],
                GuestFormat::default(),
                VoiceParams {
                    looped: true,
                    group: channel_group(channel),
                    volume: 1.0,
                },
            );
        }
        assert_eq!(choose_channel(&mut state, &audio, -1), None);
        assert!((0..CHANNELS).all(|c| audio.voice_group_active(channel_group(c))));
        assert_eq!(choose_channel(&mut state, &audio, 7), Some(7));
    }

    #[test]
    fn replaying_an_eq_stream_reuses_one_channel_instead_of_layering_music() {
        let mut cpu = pocket_cpu::stub::StubCpu::new();
        let mut kernel = crate::coredll::tests::fresh_kernel();
        kernel.fmod.clips.insert(
            FIRST_HANDLE,
            Clip {
                format: GuestFormat::default(),
                samples: Arc::new(vec![0, 0]),
                looped: true,
            },
        );
        let thunk = pocket_kernel::Thunk {
            thunk_va: 0,
            iat_va: 0,
            dll: "fmodce.dll".into(),
            binding: pocket_pe::ImportBinding::Name("test".into()),
            friendly_name: None,
        };
        let mut ctx = CallCtx {
            cpu: &mut cpu,
            kernel: &mut kernel,
            thunk: &thunk,
        };
        let first = start_voice(&mut ctx, FIRST_HANDLE, -1, true);
        assert_ne!(first, u32::MAX);
        assert_eq!(start_voice(&mut ctx, FIRST_HANDLE, -1, true), first);
        assert_eq!(ctx.kernel.fmod.channels.len(), 1);
        close(&mut ctx).unwrap();
        ctx.kernel.audio.stop();
    }

    #[test]
    fn eq_live_volume_pause_and_stream_stop_are_channel_scoped() {
        use pocket_cpu::{regs::ArmReg, stub::StubCpu};
        let mut cpu = StubCpu::new();
        let mut kernel = crate::coredll::tests::fresh_kernel();
        let thunk = pocket_kernel::Thunk {
            thunk_va: 0,
            iat_va: 0,
            dll: "fmodce.dll".into(),
            binding: pocket_pe::ImportBinding::Name("test".into()),
            friendly_name: None,
        };
        let fmt = GuestFormat {
            sample_rate: 22050,
            channels: 1,
            bits_per_sample: 16,
        };
        for (channel, clip, stream, sample) in [
            (0, 10, false, 8192),
            (1, 11, true, 4096),
            (2, 12, true, 2048),
        ] {
            kernel
                .fmod
                .channels
                .insert(channel, FmodChannel { clip, stream });
            kernel.audio.play_voice_with(
                &[sample],
                fmt,
                VoiceParams {
                    group: channel_group(channel),
                    looped: true,
                    volume: 1.0,
                },
            );
        }
        let tap = kernel.audio.tap();
        let mut ctx = CallCtx {
            cpu: &mut cpu,
            kernel: &mut kernel,
            thunk: &thunk,
        };
        let mut out = [0.0; 2];
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.4375; 2]);
        ctx.cpu.write_reg(ArmReg::R0, 0).unwrap();
        set_sfx_volume(&mut ctx).unwrap();
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.0; 2]);
        ctx.cpu.write_reg(ArmReg::R0, 255).unwrap();
        set_sfx_volume(&mut ctx).unwrap();
        ctx.cpu.write_reg(ArmReg::R0, 1).unwrap();
        ctx.cpu.write_reg(ArmReg::R1, 1).unwrap();
        set_paused(&mut ctx).unwrap();
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.3125; 2]);
        ctx.cpu.write_reg(ArmReg::R0, FSOUND_ALL as u32).unwrap();
        ctx.cpu.write_reg(ArmReg::R1, 0).unwrap();
        set_paused(&mut ctx).unwrap();
        ctx.cpu.write_reg(ArmReg::R0, 11).unwrap();
        stream_stop(&mut ctx).unwrap();
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.3125; 2]);
        assert!(ctx.kernel.fmod.channels.contains_key(&0));
        assert!(ctx.kernel.fmod.channels.contains_key(&2));
        close(&mut ctx).unwrap();
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.0; 2]);
        assert_eq!(ctx.kernel.fmod.sfx_volume, 255);
    }

    #[test]
    fn eq_sessions_have_independent_clip_handles_cache_and_volume() {
        let mut first = crate::coredll::tests::fresh_kernel();
        first.fmod.sfx_volume = 0;
        first.fmod.by_path.insert(
            "same.wav".into(),
            (GuestFormat::default(), Arc::new(vec![123])),
        );
        let second = crate::coredll::tests::fresh_kernel();
        assert_eq!(second.fmod.sfx_volume, 255);
        assert!(second.fmod.by_path.is_empty());
        assert!(second.fmod.clips.is_empty());
    }

    /// Actual EQ audio stays in the user's local library, never in test fixtures.
    #[test]
    #[ignore = "requires POCKETHLE_EQ_AUDIO_DIRS pointing at local episode assets"]
    fn local_eq_assets_decode_and_reach_the_shared_output_mix() {
        fn collect(root: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(root).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    collect(&path, files);
                } else if path
                    .extension()
                    .is_some_and(|s| s.eq_ignore_ascii_case("wav") || s.eq_ignore_ascii_case("mp3"))
                {
                    files.push(path);
                }
            }
        }
        let roots =
            std::env::var_os("POCKETHLE_EQ_AUDIO_DIRS").expect("local asset paths required");
        let mut count = 0;
        for root in std::env::split_paths(&roots) {
            let mut files = Vec::new();
            collect(&root, &mut files);
            files.sort();
            assert!(!files.is_empty(), "no audio in {}", root.display());
            for path in files {
                let bytes = std::fs::read(&path).unwrap();
                let (format, samples) =
                    decode(&bytes).unwrap_or_else(|| panic!("cannot decode {}", path.display()));
                assert!(
                    samples.iter().any(|&v| v != 0),
                    "silent source {}",
                    path.display()
                );
                let audio = pocket_kernel::AudioEngine::new();
                audio.play_voice(&samples, format, false);
                let tap = audio.tap();
                let mut buffer = vec![0.0; 4096];
                let mut audible = false;
                for _ in 0..120 {
                    // at most ~5.6 seconds, resampled to stereo 44.1 kHz
                    tap.render_mix(&mut buffer, 44100, 2);
                    if buffer.iter().any(|v| v.abs() > 0.00001) {
                        audible = true;
                        break;
                    }
                }
                assert!(audible, "no mixed output {}", path.display());
                println!(
                    "EQ_AUDIO_OK {} | {} Hz | {} ch | {} samples",
                    path.display(),
                    format.sample_rate,
                    format.channels,
                    samples.len()
                );
                count += 1;
            }
        }
        assert!(
            count >= 150,
            "expected all three local episode libraries, got {count}"
        );
        println!("EQ_AUDIO_TOTAL {count}");
    }

    #[test]
    fn garbage_is_not_mistaken_for_audio() {
        assert!(decode(&[0u8; 512]).is_none());
    }

    #[test]
    fn incomplete_and_non_audio_mp3_inputs_are_rejected_without_aborting() {
        for len in 0..=512 {
            assert!(decode_mp3(&vec![0; len]).is_none());
        }
        for bytes in [
            b"ID3".as_slice(),
            &[0xff],
            &[0xff, 0xfb],
            &[0xff, 0xfb, 0x90],
        ] {
            assert!(decode_mp3(bytes).is_none());
        }
    }

    #[test]
    fn mp3_audio_survives_leading_and_trailing_non_audio_data() {
        // Twelve valid MPEG-1 Layer III silence frames: 128 kb/s, 44.1 kHz,
        // stereo, no padding. No copyrighted audio fixture is required.
        let mut bytes = vec![0; 37];
        for _ in 0..12 {
            bytes.extend_from_slice(&[0xff, 0xfb, 0x90, 0x00]);
            bytes.extend_from_slice(&[0; 413]);
        }
        bytes.extend_from_slice(&[0; 512]);
        let (format, samples) = decode_mp3(&bytes).expect("valid silence frames decode");
        assert_eq!(format.sample_rate, 44_100);
        assert_eq!(format.channels, 2);
        // minimp3 may discard the final frame when trailing junk prevents sync.
        assert!(samples.len() >= 11 * 2304);
        assert!(samples.iter().all(|&sample| sample == 0));
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
