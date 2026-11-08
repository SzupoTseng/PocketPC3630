//! Monotonic guest time on the emulation thread. Menu pauses freeze guest
//! timers as well as execution, so EverQuest cannot jump forward on resume.
use std::cell::RefCell;
use std::time::{Duration, Instant};

struct Clock {
    epoch: Instant,
    removed: Duration,
    paused: Option<Instant>,
}
impl Clock {
    fn new() -> Self {
        Self {
            epoch: Instant::now(),
            removed: Duration::ZERO,
            paused: None,
        }
    }
    fn elapsed(&self, now: Instant) -> Duration {
        self.paused
            .unwrap_or(now)
            .saturating_duration_since(self.epoch)
            .saturating_sub(self.removed)
    }
    fn set_paused(&mut self, paused: bool, now: Instant) {
        if paused {
            self.paused.get_or_insert(now);
        } else if let Some(start) = self.paused.take() {
            self.removed += now.saturating_duration_since(start);
        }
    }
}
thread_local! { static CLOCK: RefCell<Clock> = RefCell::new(Clock::new()); }
pub fn milliseconds() -> u64 {
    CLOCK.with(|c| c.borrow().elapsed(Instant::now()).as_millis() as u64)
}
pub fn set_paused(paused: bool) {
    CLOCK.with(|c| c.borrow_mut().set_paused(paused, Instant::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_pause_requests_freeze_time_without_a_resume_jump() {
        let epoch = Instant::now();
        let mut clock = Clock {
            epoch,
            removed: Duration::ZERO,
            paused: None,
        };
        clock.set_paused(true, epoch + Duration::from_secs(2));
        clock.set_paused(true, epoch + Duration::from_secs(3));
        assert_eq!(
            clock.elapsed(epoch + Duration::from_secs(20)),
            Duration::from_secs(2)
        );
        clock.set_paused(false, epoch + Duration::from_secs(20));
        assert_eq!(
            clock.elapsed(epoch + Duration::from_secs(21)),
            Duration::from_secs(3)
        );
    }
}
