//! The guest's sense of elapsed time, which stands still while play is
//! paused.
//!
//! `GetTickCount`, `timeGetTime`, the tick page the native `GetTickCount`
//! thunk reads, `QueryPerformanceCounter`, the message loop's timers and the
//! audio engine's virtual playback cursor all take time from here. A paused
//! game that read the host clock directly would see the whole pause pass at
//! once on resuming: timers firing in a burst, a game's frame-time step
//! jumping forward, a countdown running out. Wall-clock calls
//! (`GetLocalTime`, `GetSystemTime`) keep the real date and time, as a
//! device's clock does.
//!
//! The clock is process-wide, as the host clocks it replaces were: one
//! emulator runs per process in every frontend.

use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Default)]
struct State {
    /// Time spent paused so far, not counting a pause still running.
    paused_total: Duration,
    /// When the running pause began.
    paused_since: Option<Instant>,
}

fn start() -> Instant {
    static START: OnceLock<Instant> = OnceLock::new();
    *START.get_or_init(Instant::now)
}

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(State::default()))
}

/// Guest time elapsed since the first call, paused time left out.
pub fn elapsed() -> Duration {
    let start = start();
    let now = Instant::now();
    let state = state().lock().unwrap_or_else(|e| e.into_inner());
    let running_pause = state
        .paused_since
        .map_or(Duration::ZERO, |since| now.saturating_duration_since(since));
    now.saturating_duration_since(start)
        .saturating_sub(state.paused_total + running_pause)
}

/// Milliseconds of guest time, for `GetTickCount` and timers.
pub fn millis() -> u64 {
    elapsed().as_millis() as u64
}

/// The same as [`millis`]; the name the Codex runners use.
pub fn milliseconds() -> u64 {
    millis()
}

/// Microseconds of guest time, for `QueryPerformanceCounter`.
pub fn micros() -> u64 {
    elapsed().as_micros() as u64
}

/// Stop or restart the guest clock. Pausing twice, or resuming a clock that
/// runs, changes nothing.
pub fn set_paused(paused: bool) {
    start();
    let now = Instant::now();
    let mut state = state().lock().unwrap_or_else(|e| e.into_inner());
    match (paused, state.paused_since) {
        (true, None) => state.paused_since = Some(now),
        (false, Some(since)) => {
            state.paused_total += now.saturating_duration_since(since);
            state.paused_since = None;
        }
        _ => {}
    }
}

/// Whether the guest clock is stopped.
pub fn is_paused() -> bool {
    state()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .paused_since
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paused_time_does_not_count() {
        let before = elapsed();
        set_paused(true);
        assert!(is_paused());
        let frozen = elapsed();
        std::thread::sleep(Duration::from_millis(60));
        assert!(elapsed() - frozen < Duration::from_millis(5));
        set_paused(true); // a second pause changes nothing
        set_paused(false);
        assert!(!is_paused());
        set_paused(false); // nor does a second resume
        let resumed = elapsed();
        assert!(resumed >= before);
        assert!(resumed - frozen < Duration::from_millis(20));
        std::thread::sleep(Duration::from_millis(30));
        assert!(elapsed() - resumed >= Duration::from_millis(25));
    }
}
