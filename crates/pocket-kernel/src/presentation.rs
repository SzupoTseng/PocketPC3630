//! Choose guest presentation boundaries without blocking host input or pause.
use std::time::Duration;

/// EQ redraws a character slot with many GDI/memcpy calls. A dirty counter
/// alone can publish its black erase rectangle before the slot is painted.
/// Prefer a completed copy/frame or guest wait. The bounded fallback keeps
/// held GAPI mappings (Pocket Quake) and GDI-only programs visible too.
pub fn frame_ready(
    complete: u64,
    last_complete: u64,
    waiting: bool,
    since_publish: Duration,
) -> bool {
    waiting || complete != last_complete || since_publish >= Duration::from_millis(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_mid_redraw_waits_for_completion_or_guest_wait() {
        assert!(!frame_ready(7, 7, false, Duration::from_millis(16)));
        assert!(frame_ready(8, 7, false, Duration::from_millis(16)));
        assert!(frame_ready(7, 7, true, Duration::from_millis(16)));
    }

    #[test]
    fn held_framebuffer_without_end_draw_does_not_freeze() {
        assert!(!frame_ready(0, 0, false, Duration::from_millis(99)));
        assert!(frame_ready(0, 0, false, Duration::from_millis(100)));
    }
}
