//! Real-time audio output backed by [`cpal`].
//!
//! The Pocket PC `waveOut*` API and `PlaySoundW` / `PlaySoundA` /
//! `sndPlaySoundW` family are routed through this module so that
//! games which previously had no sound can drive real PCM samples
//! out of the host's default audio device. The mixing model is
//! intentionally tiny — a single shared lock-protected ring buffer
//! of interleaved `i16` samples that the cpal output callback drains
//! at the host device's native sample rate. Two rate-adapter
//! strategies cover most Pocket PC content:
//!
//! * **Same-rate path** (the host opens the device at exactly the
//!   guest's requested rate, e.g. 44.1 kHz / 22.05 kHz / 11.025 kHz):
//!   samples are popped 1:1.
//! * **Resampled path** (the host device only supports a different
//!   rate): we use a nearest-neighbour resampler. This is good
//!   enough for the Pocket PC sound effects and looped music
//!   (typically 11.025 kHz mono, 22.05 kHz stereo) which already
//!   contain noticeable quantisation noise.
//!
//! The engine never blocks the emulator: if the host audio thread is
//! late and the ring buffer has overflowed, new samples are
//! dropped. Conversely, if the ring is empty when cpal asks for
//! more, we emit silence. This keeps the emulator's frame loop
//! decoupled from the audio device's buffer churn.
//!
//! The cpal feature is optional. When the `audio-cpal` feature is
//! disabled (e.g. on the Android JNI build, or in `--no-default-features`
//! CI runs that have no audio devices), PCM and voices remain available
//! through [`AudioTap::render_mix`]. Android AudioTrack and Web Audio pull
//! this same mix; disabling cpal only disables the host cpal device.

use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Maximum number of i16 samples we keep buffered. At 44.1 kHz
/// stereo this is about twelve seconds — plenty to absorb scheduling
/// jitter, but small enough that overflows produce dropped samples
/// instead of unbounded memory growth.
const RING_CAPACITY_SAMPLES: usize = 1 << 20; // 1048576

/// Audio format last requested by the guest. We remember it so the
/// cpal callback can decide whether to upsample or play 1:1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuestFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl Default for GuestFormat {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            bits_per_sample: 16,
        }
    }
}

#[derive(Debug)]
struct AudioVoice {
    samples: Vec<i16>,
    format: GuestFormat,
    position_q16: u64,
    looped: bool,
    volume: f32,
    group: u32,
    paused: bool,
}

/// How a voice submitted through [`AudioEngine::play_voice_with`]
/// should behave.
///
/// `group` exists so a caller can silence one class of sound without
/// touching the rest: `hss.dll`'s `stopMusics` must stop the music
/// without cutting off the sound effects that are still playing.
#[derive(Debug, Clone, Copy)]
pub struct VoiceParams {
    pub looped: bool,
    /// Tag for [`AudioEngine::stop_voice_group`]. Voices submitted
    /// through [`AudioEngine::play_voice`] land in group 0.
    pub group: u32,
    /// Linear gain. 1.0 plays the samples as submitted.
    pub volume: f32,
}

impl Default for VoiceParams {
    fn default() -> Self {
        Self {
            looped: false,
            group: 0,
            volume: 1.0,
        }
    }
}

/// Inner state shared between the emulator thread (which calls
/// [`AudioEngine::push_samples`]) and the cpal output callback.
struct Shared {
    ring: Vec<i16>,
    /// Number of samples currently in the ring.
    len: usize,
    /// Read cursor.
    read: usize,
    /// Write cursor.
    write: usize,
    /// Format of the most recently submitted guest stream.
    guest_format: GuestFormat,
    /// Stable output format used by host frontends for the lifetime of
    /// the audio session. Android AudioTrack must not be reconfigured
    /// halfway through a session when another waveOut device opens.
    mix_format: GuestFormat,
    mix_format_ready: bool,
    /// Track whether the guest explicitly opened audio so Android does
    /// not build AudioTrack from the fallback format.
    guest_format_ready: bool,
    /// Sub-sample fraction for the nearest-neighbour resampler (in
    /// units of 1/65536). Carried across cpal callbacks so we don't
    /// lose pitch on long playbacks.
    resampler_phase: u64,
    /// Total guest samples ever submitted through [`Shared::push`].
    written: u64,
    /// Total guest samples the host device has actually consumed.
    /// Only meaningful while `device_active` is set.
    consumed: u64,
    /// `true` once a cpal output stream is playing. With no host
    /// device we fall back to a wall-clock playback estimate so
    /// guests that wait for buffer-done notifications still make
    /// progress.
    device_active: bool,
    /// Wall-clock playback estimate, in guest samples.
    virtual_cursor: u64,
    /// When the wall-clock estimate was last advanced.
    /// Guest time of the last advance; guest time stands still while play
    /// is paused, so the virtual cursor does too.
    virtual_tick: Option<std::time::Duration>,
    /// `true` between `waveOutPause` and `waveOutRestart`. The device
    /// keeps its queue but stops consuming, so the playback cursor
    /// freezes and buffer-done notifications stop until playback
    /// resumes.
    paused: bool,
    /// Independent sound-effect voices mixed over the waveOut stream.
    voices: Vec<AudioVoice>,
    reported_output: bool,
    /// Optional WAV tap so headless runs can verify that a game
    /// really produces sound on a machine with no audio hardware.
    capture: Option<WavCapture>,
}

impl Shared {
    fn new() -> Self {
        Self {
            ring: vec![0i16; RING_CAPACITY_SAMPLES],
            len: 0,
            read: 0,
            write: 0,
            guest_format: GuestFormat::default(),
            mix_format: GuestFormat::default(),
            mix_format_ready: false,
            guest_format_ready: false,
            resampler_phase: 0,
            written: 0,
            consumed: 0,
            device_active: false,
            virtual_cursor: 0,
            virtual_tick: None,
            paused: false,
            voices: Vec::new(),
            reported_output: false,
            capture: None,
        }
    }

    /// Guest samples played so far. Backed by the host device when we
    /// have one and by a wall clock otherwise. Never runs ahead of
    /// what the guest actually submitted, so a game that stops
    /// feeding buffers doesn't see phantom progress.
    fn cursor(&mut self) -> u64 {
        if self.paused {
            // Reset the wall-clock reference so a long pause doesn't
            // release a burst of buffers the moment playback resumes.
            self.virtual_tick = None;
            return if self.device_active {
                self.consumed.min(self.written)
            } else {
                self.virtual_cursor
            };
        }
        if self.device_active {
            return self.consumed.min(self.written);
        }
        let rate =
            self.guest_format.sample_rate.max(1) as u64 * self.guest_format.channels.max(1) as u64;
        let now = crate::guest_clock::elapsed();
        let last = *self.virtual_tick.get_or_insert(now);
        let elapsed_us = now.saturating_sub(last).as_micros() as u64;
        if elapsed_us > 0 {
            self.virtual_tick = Some(now);
            self.virtual_cursor = self
                .virtual_cursor
                .saturating_add(elapsed_us.saturating_mul(rate) / 1_000_000);
        }
        self.virtual_cursor = self.virtual_cursor.min(self.written);
        self.virtual_cursor
    }

    fn push(&mut self, samples: &[i16]) {
        self.written = self.written.saturating_add(samples.len() as u64);
        let fmt = self.guest_format;
        if let Some(cap) = self.capture.as_mut() {
            cap.write(samples, fmt);
        }
        let cap = self.ring.len();
        for &s in samples {
            if self.len == cap {
                // Ring is full — drop the oldest sample to make room.
                self.read = (self.read + 1) % cap;
                self.len -= 1;
            }
            self.ring[self.write] = s;
            self.write = (self.write + 1) % cap;
            self.len += 1;
        }
    }

    fn pop_one(&mut self) -> Option<i16> {
        if self.len == 0 {
            return None;
        }
        let v = self.ring[self.read];
        self.read = (self.read + 1) % self.ring.len();
        self.len -= 1;
        self.consumed = self.consumed.saturating_add(1);
        Some(v)
    }

    fn clear(&mut self) {
        self.len = 0;
        self.read = 0;
        self.write = 0;
        self.resampler_phase = 0;
        self.written = 0;
        self.consumed = 0;
        self.virtual_cursor = 0;
        self.virtual_tick = None;
        self.voices.clear();
    }

    fn stop_voices(&mut self) {
        self.voices.clear();
    }

    fn stop_voice_group(&mut self, group: u32) {
        self.voices.retain(|v| v.group != group);
    }

    fn add_voice(&mut self, samples: Vec<i16>, format: GuestFormat, params: VoiceParams) {
        if samples.is_empty() {
            return;
        }
        if let Some(cap) = self.capture.as_mut() {
            cap.write(&samples, format);
        }
        self.voices.push(AudioVoice {
            samples,
            format,
            position_q16: 0,
            looped: params.looped,
            volume: params.volume,
            group: params.group,
            paused: false,
        });
    }

    fn render_frames(&mut self, output: &mut [f32], output_rate: u32, output_channels: u16) {
        let channels = output_channels.max(1) as usize;
        let frames = output.len() / channels;
        for frame in 0..frames {
            let mut left = 0.0f32;
            let mut right = 0.0f32;
            if !self.paused {
                let guest_channels = self.guest_format.channels.max(1) as usize;
                if self.len >= guest_channels {
                    let left_sample = self.ring[self.read] as f32 / 32768.0;
                    let right_sample = if guest_channels > 1 {
                        self.ring[(self.read + 1) % self.ring.len()] as f32 / 32768.0
                    } else {
                        left_sample
                    };
                    left += left_sample;
                    right += right_sample;
                    let step_q16 = ((self.guest_format.sample_rate.max(1) as u64) << 16)
                        / output_rate.max(1) as u64;
                    self.resampler_phase = self.resampler_phase.saturating_add(step_q16);
                    let advance_frames = (self.resampler_phase >> 16) as usize;
                    self.resampler_phase &= 0xFFFF;
                    for _ in 0..advance_frames.saturating_mul(guest_channels) {
                        let _ = self.pop_one();
                    }
                } else {
                    self.resampler_phase = 0;
                }

                let mut index = 0;
                while index < self.voices.len() {
                    let voice = &mut self.voices[index];
                    if voice.paused {
                        index += 1;
                        continue;
                    }
                    let voice_channels = voice.format.channels.max(1) as usize;
                    let frame_count = voice.samples.len() / voice_channels;
                    let source_frame = (voice.position_q16 >> 16) as usize;
                    if frame_count == 0 || source_frame >= frame_count {
                        if voice.looped && frame_count > 0 {
                            voice.position_q16 %= (frame_count as u64) << 16;
                        } else {
                            self.voices.swap_remove(index);
                            continue;
                        }
                    }
                    let source_frame = (voice.position_q16 >> 16) as usize;
                    let sample_index = source_frame * voice_channels;
                    let left_sample = voice.samples[sample_index] as f32 / 32768.0 * voice.volume;
                    let right_sample = if voice_channels > 1 {
                        voice.samples[sample_index + 1] as f32 / 32768.0 * voice.volume
                    } else {
                        left_sample
                    };
                    left += left_sample;
                    right += right_sample;
                    voice.position_q16 = voice.position_q16.saturating_add(
                        ((voice.format.sample_rate.max(1) as u64) << 16)
                            / output_rate.max(1) as u64,
                    );
                    index += 1;
                }
            }
            left = left.clamp(-1.0, 1.0);
            right = right.clamp(-1.0, 1.0);
            if !self.reported_output && (left.abs() > 0.00001 || right.abs() > 0.00001) {
                self.reported_output = true;
                log::info!("AudioEngine: first non-silent mixed output: {output_rate} Hz / {output_channels} ch");
            }
            for channel in 0..channels {
                output[frame * channels + channel] = if channel == 0 { left } else { right };
            }
        }
        for sample in &mut output[frames * channels..] {
            *sample = 0.0;
        }
    }
}

/// Minimal streaming WAV writer behind [`AudioEngine::capture_to`].
///
/// The RIFF sizes are rewritten after every submission rather than
/// only on drop: emulator runs are frequently killed by a signal
/// (timeouts, frame-budget harnesses) and a half-written header would
/// make the capture useless exactly when it is needed most.
struct WavCapture {
    file: std::fs::File,
    data_bytes: u64,
    header: Option<GuestFormat>,
}

impl WavCapture {
    fn create(path: &Path) -> std::io::Result<Self> {
        Ok(Self {
            file: std::fs::File::create(path)?,
            data_bytes: 0,
            header: None,
        })
    }

    fn write(&mut self, samples: &[i16], fmt: GuestFormat) {
        if self.header.is_none() {
            // Capture is always 16-bit: `push_samples_u8` widens 8-bit
            // PCM before it reaches the ring.
            let fmt = GuestFormat {
                bits_per_sample: 16,
                ..fmt
            };
            if self.write_header(fmt).is_err() {
                return;
            }
            self.header = Some(fmt);
        }
        let mut bytes = Vec::with_capacity(samples.len() * 2);
        for s in samples {
            bytes.extend_from_slice(&s.to_le_bytes());
        }
        if self.file.write_all(&bytes).is_err() {
            return;
        }
        self.data_bytes = self.data_bytes.saturating_add(bytes.len() as u64);
        let _ = self.patch_sizes();
    }

    fn write_header(&mut self, fmt: GuestFormat) -> std::io::Result<()> {
        let channels = fmt.channels.max(1);
        let rate = fmt.sample_rate.max(1);
        let block_align = channels * 2;
        let byte_rate = rate * block_align as u32;
        let mut h = Vec::with_capacity(44);
        h.extend_from_slice(b"RIFF");
        h.extend_from_slice(&0u32.to_le_bytes()); // patched by patch_sizes
        h.extend_from_slice(b"WAVEfmt ");
        h.extend_from_slice(&16u32.to_le_bytes());
        h.extend_from_slice(&1u16.to_le_bytes()); // WAVE_FORMAT_PCM
        h.extend_from_slice(&channels.to_le_bytes());
        h.extend_from_slice(&rate.to_le_bytes());
        h.extend_from_slice(&byte_rate.to_le_bytes());
        h.extend_from_slice(&block_align.to_le_bytes());
        h.extend_from_slice(&16u16.to_le_bytes());
        h.extend_from_slice(b"data");
        h.extend_from_slice(&0u32.to_le_bytes()); // patched by patch_sizes
        self.file.write_all(&h)
    }

    fn patch_sizes(&mut self) -> std::io::Result<()> {
        let data = self.data_bytes.min(u32::MAX as u64 - 36) as u32;
        self.file.seek(SeekFrom::Start(4))?;
        self.file.write_all(&(36 + data).to_le_bytes())?;
        self.file.seek(SeekFrom::Start(40))?;
        self.file.write_all(&data.to_le_bytes())?;
        self.file.seek(SeekFrom::End(0))?;
        Ok(())
    }
}

/// Public handle the rest of the emulator interacts with. Cheaply
/// cloneable via [`AudioEngine::clone`] (the underlying state is
/// behind an `Arc<Mutex<_>>`); the kernel keeps one copy and
/// [`AudioEngine::start`] / [`AudioEngine::stop`] hand additional
/// clones to the cpal callback.
pub struct AudioEngine {
    shared: Arc<Mutex<Shared>>,
    /// Whether the audio worker thread is alive. The thread itself
    /// owns the cpal `Stream` (which is `!Send` on some platforms),
    /// so we communicate with it via [`Shared`] and the
    /// [`Self::shutdown`] flag.
    #[cfg(feature = "audio-cpal")]
    worker: Option<std::thread::JoinHandle<()>>,
    #[cfg(not(feature = "audio-cpal"))]
    worker: Option<()>,
    /// Set by [`Self::stop`] to ask the audio thread to drop its
    /// stream and exit.
    #[cfg(feature = "audio-cpal")]
    shutdown: Arc<std::sync::atomic::AtomicBool>,
    /// `true` once we've tried (and possibly failed) to open the
    /// device. Stops us from spamming the user log with retries on
    /// every `waveOutOpen`.
    init_attempted: bool,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEngine")
            .field("init_attempted", &self.init_attempted)
            .field("worker_alive", &self.worker.is_some())
            .finish()
    }
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            shared: Arc::new(Mutex::new(Shared::new())),
            worker: None,
            #[cfg(feature = "audio-cpal")]
            shutdown: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            init_attempted: false,
        }
    }

    /// Update the guest-side format. Called from `waveOutOpen` /
    /// `PlaySound` so the resampler knows what rate the i16 samples
    /// are coming in at.
    pub fn set_guest_format(&self, fmt: GuestFormat) {
        if let Ok(mut s) = self.shared.lock() {
            s.guest_format = fmt;
            if !s.mix_format_ready {
                s.mix_format = fmt;
                s.mix_format_ready = true;
            }
            s.guest_format_ready = true;
            s.resampler_phase = 0;
        }
    }

    /// Submit a chunk of interleaved 16-bit PCM. The samples are
    /// queued for the host audio thread to play. Returns the number
    /// of samples actually queued (we never block; if the ring is
    /// full the oldest samples get dropped).
    pub fn push_samples(&self, samples: &[i16]) -> usize {
        if let Ok(mut s) = self.shared.lock() {
            s.push(samples);
            samples.len()
        } else {
            0
        }
    }

    pub fn play_voice(&self, samples: &[i16], format: GuestFormat, looped: bool) -> usize {
        self.play_voice_with(
            samples,
            format,
            VoiceParams {
                looped,
                ..Default::default()
            },
        )
    }

    /// Submit a voice with an explicit group and gain — see
    /// [`VoiceParams`].
    pub fn play_voice_with(
        &self,
        samples: &[i16],
        format: GuestFormat,
        params: VoiceParams,
    ) -> usize {
        if let Ok(mut s) = self.shared.lock() {
            s.add_voice(samples.to_vec(), format, params);
            samples.len()
        } else {
            0
        }
    }

    pub fn stop_voices(&self) {
        if let Ok(mut s) = self.shared.lock() {
            s.stop_voices();
        }
    }

    /// Stop only the voices tagged with `group`, leaving every other
    /// voice playing.
    pub fn stop_voice_group(&self, group: u32) {
        if let Ok(mut s) = self.shared.lock() {
            s.stop_voice_group(group);
        }
    }

    /// Convenience for unsigned 8-bit PCM (the format `PlaySound` and
    /// some old WAV resources use). Each byte is mapped to the
    /// signed 16-bit range linearly.
    /// A paused voice still owns its channel. Finished one-shots need not wait
    /// for the next output callback to release their FMOD channel.
    pub fn voice_group_active(&self, group: u32) -> bool {
        self.shared
            .lock()
            .map(|shared| {
                shared.voices.iter().any(|voice| {
                    let frames =
                        (voice.samples.len() / voice.format.channels.max(1) as usize) as u64;
                    voice.group == group
                        && frames > 0
                        && (voice.looped || (voice.position_q16 >> 16) < frames)
                })
            })
            .unwrap_or(false)
    }

    /// EverQuest's FMOD options change voices already playing, not just new clips.
    pub fn set_voice_group_volume(&self, group: u32, volume: f32) {
        if let Ok(mut shared) = self.shared.lock() {
            for voice in shared
                .voices
                .iter_mut()
                .filter(|voice| voice.group == group)
            {
                voice.volume = volume.clamp(0.0, 1.0);
            }
        }
    }

    /// Pause a middleware channel without consuming its playback position.
    pub fn set_voice_group_paused(&self, group: u32, paused: bool) {
        if let Ok(mut shared) = self.shared.lock() {
            for voice in shared
                .voices
                .iter_mut()
                .filter(|voice| voice.group == group)
            {
                voice.paused = paused;
            }
        }
    }

    pub fn push_samples_u8(&self, bytes: &[u8]) -> usize {
        let mut buf = Vec::with_capacity(bytes.len());
        for &b in bytes {
            let v = (b as i16 - 128) * 256;
            buf.push(v);
        }
        self.push_samples(&buf)
    }

    /// Drop any queued samples. Called by `waveOutReset` and on
    /// engine shutdown.
    pub fn flush(&self) {
        if let Ok(mut s) = self.shared.lock() {
            s.clear();
            s.mix_format_ready = false;
            s.guest_format_ready = false;
            s.resampler_phase = 0;
        }
    }

    /// Suspend or resume playback (`waveOutPause` / `waveOutRestart`).
    /// While paused the device keeps its queue but stops consuming, so
    /// [`Self::playback_cursor`] freezes and no further buffers are
    /// reported as finished.
    pub fn set_paused(&self, paused: bool) {
        if let Ok(mut s) = self.shared.lock() {
            s.paused = paused;
        }
    }

    /// Number of samples currently queued.
    pub fn buffered_samples(&self) -> usize {
        self.shared.lock().map(|s| s.len).unwrap_or(0)
    }

    /// Total guest samples submitted since the stream was opened (or
    /// since the last [`Self::flush`]).
    pub fn written_samples(&self) -> u64 {
        self.shared.lock().map(|s| s.written).unwrap_or(0)
    }

    /// Guest samples played back so far. `waveOutGetPosition` and the
    /// `WOM_DONE` bookkeeping in `waveOut*` are both driven from this.
    pub fn playback_cursor(&self) -> u64 {
        self.shared.lock().map(|mut s| s.cursor()).unwrap_or(0)
    }

    /// A cheap, clone-able handle that lets the *host* play the guest's
    /// PCM itself instead of leaving it to cpal.
    ///
    /// Android needs this: cpal's Android backend goes through
    /// `oboe-sys`, which needs an NDK C++ toolchain we do not ship, so
    /// the `audio-cpal` feature is off there and the ring buffer would
    /// otherwise just fill up and drop samples. The JNI layer pulls from
    /// a tap and writes into a Java `AudioTrack` in streaming mode — the
    /// same shape J2ME emulators such as J2ME Loader use for PCM output.
    ///
    /// The tap can be moved to another thread; it locks the same shared
    /// state the guest pushes into.
    pub fn tap(&self) -> AudioTap {
        AudioTap {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Tee every submitted sample into a 16-bit PCM WAV file. Used by
    /// `pockethle run --dump-audio-to` so a run on a machine with no
    /// sound card can still prove the game produced audio.
    pub fn capture_to(&self, path: &std::path::Path) -> std::io::Result<()> {
        let capture = WavCapture::create(path)?;
        if let Ok(mut s) = self.shared.lock() {
            s.capture = Some(capture);
        }
        Ok(())
    }

    /// Open the host audio device and start streaming. Idempotent —
    /// subsequent calls are no-ops as long as the worker is still
    /// alive. When the `audio-cpal` feature is disabled this is a
    /// no-op.
    pub fn start(&mut self) {
        if self.worker.is_some() || self.init_attempted {
            return;
        }
        self.init_attempted = true;
        self.start_impl();
    }

    #[cfg(feature = "audio-cpal")]
    fn start_impl(&mut self) {
        // cpal::Stream is `!Send` on some platforms, so the stream
        // has to be owned by a single dedicated thread. We hand the
        // shared ring + a shutdown flag to the worker; the worker
        // builds the stream, plays it, and parks until told to
        // exit, at which point dropping the stream stops audio
        // playback.
        let shared = Arc::clone(&self.shared);
        let shutdown = Arc::clone(&self.shutdown);
        shutdown.store(false, std::sync::atomic::Ordering::SeqCst);
        let handle = match std::thread::Builder::new()
            .name("pockethle-audio".to_string())
            .spawn(move || run_audio_worker(shared, shutdown))
        {
            Ok(handle) => Some(handle),
            Err(e) => {
                log::warn!("AudioEngine: could not spawn audio thread — running silently: {e}");
                None
            }
        };
        self.worker = handle;
    }

    #[cfg(not(feature = "audio-cpal"))]
    fn start_impl(&mut self) {
        // EverQuest voices are also mixed by Web Audio / Android AudioTrack.
        // No cpal device does not mean the shared mixer is silent.
        log::info!("AudioEngine: cpal output disabled; audio is available through AudioTap");
    }

    /// Stop the host stream and clear any pending samples. The
    /// engine can be re-`start`ed afterwards.
    pub fn stop(&mut self) {
        #[cfg(feature = "audio-cpal")]
        {
            self.shutdown
                .store(true, std::sync::atomic::Ordering::SeqCst);
            if let Some(h) = self.worker.take() {
                let _ = h.join();
            }
        }
        #[cfg(not(feature = "audio-cpal"))]
        {
            self.worker = None;
        }
        self.flush();
        self.init_attempted = false;
    }
}

/// Host-side pull handle produced by [`AudioEngine::tap`].
///
/// Draining moves the playback cursor forward, so `waveOutGetPosition`
/// and the `WHDR_DONE` notifications stay in step with what the host has
/// actually played. While nothing drains the tap the engine keeps using
/// its wall-clock estimate, so a frontend that never calls
/// [`AudioTap::drain_into`] behaves exactly as it did before.
#[derive(Clone)]
pub struct AudioTap {
    shared: Arc<Mutex<Shared>>,
}

impl std::fmt::Debug for AudioTap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioTap").finish_non_exhaustive()
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        // EverQuest sessions can end through an error or a replaced WASM
        // session too. Do not leave a cpal worker or a looping theme alive.
        self.stop();
    }
}

impl AudioTap {
    /// The format the guest is currently submitting samples in, so the
    /// host can configure its own output device to match.
    pub fn guest_format(&self) -> GuestFormat {
        self.shared
            .lock()
            .map(|s| {
                if s.mix_format_ready {
                    s.mix_format
                } else {
                    s.guest_format
                }
            })
            .unwrap_or_default()
    }

    pub fn format_ready(&self) -> bool {
        self.shared
            .lock()
            .map(|s| s.guest_format_ready)
            .unwrap_or(false)
    }

    /// Move up to `dst.len()` queued samples out of the ring into `dst`,
    /// returning how many were written. Never blocks: a starved host
    /// gets a short read and should pad the rest with silence.
    /// Mix the waveOut stream and every voice into interleaved floats.
    ///
    /// EverQuest's FMOD effects are voices that overlap each other, so hosts
    /// without cpal (Android, the browser) must pull a mix: draining the raw
    /// ring would miss them, and queueing them would play them one by one.
    pub fn render_mix(&self, output: &mut [f32], output_rate: u32, output_channels: u16) {
        output.fill(0.0);
        if let Ok(mut s) = self.shared.lock() {
            s.device_active = true;
            s.render_frames(output, output_rate, output_channels);
        }
    }

    pub fn drain_into(&self, dst: &mut [i16]) -> usize {
        let Ok(mut s) = self.shared.lock() else {
            return 0;
        };
        s.device_active = true;
        let mut n = 0;
        while n < dst.len() {
            match s.pop_one() {
                Some(v) => {
                    dst[n] = v;
                    n += 1;
                }
                None => break,
            }
        }
        n
    }

    /// Move up to `dst.len()` samples without taking ownership of the
    /// playback cursor. This is used by Android while it is probing the
    /// guest format; probing must not switch the engine to device-clock
    /// mode before AudioTrack has actually started.
    pub fn peek_into(&self, dst: &mut [i16]) -> usize {
        let Ok(s) = self.shared.lock() else {
            return 0;
        };
        let mut n = 0;
        let mut index = s.read;
        let mut remaining = s.len;
        while n < dst.len() && remaining > 0 {
            dst[n] = s.ring[index];
            n += 1;
            remaining -= 1;
            index = (index + 1) % s.ring.len();
        }
        n
    }

    /// How many samples are queued and ready to be drained.
    pub fn buffered_samples(&self) -> usize {
        self.shared.lock().map(|s| s.len).unwrap_or(0)
    }
}

#[cfg(feature = "audio-cpal")]
fn run_audio_worker(shared: Arc<Mutex<Shared>>, shutdown: Arc<std::sync::atomic::AtomicBool>) {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();
    let device = match host.default_output_device() {
        Some(d) => d,
        None => {
            // Every early return in this function means the run is
            // silent. That is a user-visible failure, not a detail, so
            // these log at `warn`: the desktop GUI is built with
            // `windows_subsystem = "windows"` and has no console, so an
            // `info` line reaches nobody and "no sound" arrives with no
            // explanation attached.
            log::warn!("AudioEngine: no default output device — running silently");
            return;
        }
    };
    let device_name = device.name().unwrap_or_else(|_| "<unnamed>".to_string());
    let config = match device.default_output_config() {
        Ok(c) => c,
        Err(e) => {
            log::warn!(
                "AudioEngine: default_output_config() failed on {device_name:?} \
                 — running silently: {e}"
            );
            return;
        }
    };

    let host_rate = config.sample_rate().0;
    let host_channels = config.channels();
    let sample_format = config.sample_format();
    let stream_config: cpal::StreamConfig = config.clone().into();
    let err_fn = |err| log::warn!("AudioEngine: cpal output error: {err}");

    let stream = match sample_format {
        cpal::SampleFormat::F32 => {
            let shared = Arc::clone(&shared);
            device.build_output_stream(
                &stream_config,
                move |data: &mut [f32], _| {
                    fill_output_f32(&shared, data, host_rate, host_channels);
                },
                err_fn,
                None,
            )
        }
        cpal::SampleFormat::I16 => {
            let shared = Arc::clone(&shared);
            device.build_output_stream(
                &stream_config,
                move |data: &mut [i16], _| {
                    fill_output_i16(&shared, data, host_rate, host_channels);
                },
                err_fn,
                None,
            )
        }
        cpal::SampleFormat::U16 => {
            let shared = Arc::clone(&shared);
            device.build_output_stream(
                &stream_config,
                move |data: &mut [u16], _| {
                    fill_output_u16(&shared, data, host_rate, host_channels);
                },
                err_fn,
                None,
            )
        }
        other => {
            log::warn!(
                "AudioEngine: host device {device_name:?} wants sample format \
                 {other:?}, which is not supported — running silently"
            );
            return;
        }
    };
    let stream = match stream {
        Ok(s) => s,
        Err(e) => {
            log::warn!(
                "AudioEngine: build_output_stream failed on {device_name:?} \
                 ({host_rate} Hz / {host_channels} ch / {sample_format:?}) \
                 — running silently: {e}"
            );
            return;
        }
    };
    if let Err(e) = stream.play() {
        log::warn!("AudioEngine: stream.play() failed on {device_name:?} — running silently: {e}");
        return;
    }
    log::info!(
        "AudioEngine: opened {:?} at {} Hz / {} ch ({:?})",
        device_name,
        host_rate,
        host_channels,
        sample_format
    );
    if let Ok(mut s) = shared.lock() {
        s.device_active = true;
    }
    while !shutdown.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    if let Ok(mut s) = shared.lock() {
        s.device_active = false;
    }
    drop(stream);
}

#[cfg(feature = "audio-cpal")]
fn fill_output_f32(
    shared: &Arc<Mutex<Shared>>,
    data: &mut [f32],
    host_rate: u32,
    host_channels: u16,
) {
    let mut s = match shared.lock() {
        Ok(g) => g,
        Err(_) => {
            data.fill(0.0);
            return;
        }
    };
    s.render_frames(data, host_rate, host_channels);
}

#[cfg(feature = "audio-cpal")]
fn fill_output_i16(
    shared: &Arc<Mutex<Shared>>,
    data: &mut [i16],
    host_rate: u32,
    host_channels: u16,
) {
    let mut tmp = vec![0f32; data.len()];
    fill_output_f32(shared, &mut tmp, host_rate, host_channels);
    for (i, v) in tmp.iter().enumerate() {
        let s = (v * 32767.0).clamp(-32768.0, 32767.0) as i16;
        data[i] = s;
    }
}

#[cfg(feature = "audio-cpal")]
fn fill_output_u16(
    shared: &Arc<Mutex<Shared>>,
    data: &mut [u16],
    host_rate: u32,
    host_channels: u16,
) {
    let mut tmp = vec![0f32; data.len()];
    fill_output_f32(shared, &mut tmp, host_rate, host_channels);
    for (i, v) in tmp.iter().enumerate() {
        let s = ((v + 1.0) * 32767.5).clamp(0.0, 65535.0) as u16;
        data[i] = s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voice_pause_retains_position_and_live_gain_does_not_mute_other_groups() {
        let e = AudioEngine::new();
        let fmt = GuestFormat {
            sample_rate: 22050,
            channels: 1,
            bits_per_sample: 16,
        };
        e.play_voice_with(
            &[8192, 16384],
            fmt,
            VoiceParams {
                group: 10,
                looped: true,
                volume: 1.0,
            },
        );
        let tap = e.tap();
        let mut out = [0.0; 2];
        tap.render_mix(&mut out, 22050, 2);
        assert_eq!(out, [0.25; 2]);
        e.set_voice_group_paused(10, true);
        tap.render_mix(&mut out, 22050, 2);
        assert_eq!(out, [0.0; 2]);
        e.set_voice_group_paused(10, false);
        e.set_voice_group_volume(10, 0.5);
        tap.render_mix(&mut out, 22050, 2);
        assert_eq!(out, [0.25; 2]); // second sample at half gain, not restarted
        e.play_voice_with(
            &[4096],
            fmt,
            VoiceParams {
                group: 11,
                looped: true,
                volume: 1.0,
            },
        );
        e.set_voice_group_volume(10, 0.0);
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.125; 2]); // music remains audible when effects are muted
        e.stop_voice_group(11);
        tap.render_mix(&mut out, 44100, 2);
        assert_eq!(out, [0.0; 2]);
    }

    #[test]
    fn dropping_an_engine_stops_voices_and_pcm_even_when_a_tap_survives() {
        let audio = AudioEngine::new();
        let format = GuestFormat {
            sample_rate: 44100,
            channels: 1,
            bits_per_sample: 16,
        };
        audio.set_guest_format(format);
        audio.push_samples(&[4096; 8]);
        audio.play_voice(&[8192], format, true);
        let tap = audio.tap();
        let mut output = [0.0; 2];
        tap.render_mix(&mut output, 44100, 2);
        assert_eq!(output, [0.375; 2]);
        drop(audio);
        tap.render_mix(&mut output, 44100, 2);
        assert_eq!(output, [0.0; 2]);
    }

    #[test]
    fn engine_starts_silently_with_no_feature() {
        let mut e = AudioEngine::new();
        e.start();
        // start() must not panic regardless of host configuration.
        e.stop();
    }

    #[test]
    fn ring_drops_oldest_on_overflow() {
        let mut s = Shared::new();
        let big = vec![1i16; RING_CAPACITY_SAMPLES + 8];
        s.push(&big);
        assert_eq!(s.len, RING_CAPACITY_SAMPLES);
    }

    #[test]
    fn push_then_buffered_samples() {
        let e = AudioEngine::new();
        e.set_guest_format(GuestFormat {
            sample_rate: 22050,
            channels: 1,
            bits_per_sample: 16,
        });
        e.push_samples(&[0, 1, 2, 3]);
        assert_eq!(e.buffered_samples(), 4);
        e.flush();
        assert_eq!(e.buffered_samples(), 0);
    }

    #[test]
    fn push_u8_maps_to_signed_range() {
        let e = AudioEngine::new();
        let n = e.push_samples_u8(&[0x80, 0x00, 0xFF]);
        assert_eq!(n, 3);
        assert_eq!(e.buffered_samples(), 3);
    }
}
