// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! PFO_N (power-fail output, active-low) debounce.
//!
//! Servo current steps dip the 5 V rail for a few ms; reacting to every dip would trip the
//! shutdown path while walking. The reported state flips only after the raw level has
//! held steady for the whole window, in both directions.

/// Shortest window allowed. Below this, servo-load dips get through as power fails.
pub const MIN_DEBOUNCE_MS: u32 = 100;

#[derive(Debug, PartialEq, Eq)]
pub struct DebounceTooShort(pub u32);

#[derive(Debug)]
pub struct PfoDebounce {
    window_ms: u32,
    power_fail: bool,
    pending_since_ms: Option<u32>,
}

impl PfoDebounce {
    pub fn new(window_ms: u32) -> Result<Self, DebounceTooShort> {
        if window_ms < MIN_DEBOUNCE_MS {
            return Err(DebounceTooShort(window_ms));
        }
        Ok(Self {
            window_ms,
            power_fail: false,
            pending_since_ms: None,
        })
    }

    /// Feeds one sample of PFO_N (`pfo_n_low` = pin reads low) taken at `now_ms`
    /// (monotonic, wrapping). Returns the debounced power-fail state.
    pub fn update(&mut self, now_ms: u32, pfo_n_low: bool) -> bool {
        if pfo_n_low == self.power_fail {
            self.pending_since_ms = None;
            return self.power_fail;
        }
        let since = *self.pending_since_ms.get_or_insert(now_ms);
        if now_ms.wrapping_sub(since) >= self.window_ms {
            self.power_fail = pfo_n_low;
            self.pending_since_ms = None;
        }
        self.power_fail
    }

    pub fn power_fail(&self) -> bool {
        self.power_fail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Samples at the 50 Hz loop period.
    const TICK_MS: u32 = 20;

    fn feed(debounce: &mut PfoDebounce, start_ms: u32, duration_ms: u32, low: bool) -> bool {
        let mut state = debounce.power_fail();
        let mut now = start_ms;
        while now.wrapping_sub(start_ms) <= duration_ms {
            state = debounce.update(now, low);
            now = now.wrapping_add(TICK_MS);
        }
        state
    }

    #[test]
    fn window_below_100_ms_is_rejected() {
        assert_eq!(PfoDebounce::new(99).unwrap_err(), DebounceTooShort(99));
        assert!(PfoDebounce::new(MIN_DEBOUNCE_MS).is_ok());
    }

    #[test]
    fn short_dip_is_ignored() {
        let mut debounce = PfoDebounce::new(100).unwrap();
        assert!(!feed(&mut debounce, 0, 80, true));
        assert!(!debounce.update(100, false));
        assert!(!feed(&mut debounce, 120, 80, true));
    }

    #[test]
    fn low_held_for_window_trips() {
        let mut debounce = PfoDebounce::new(100).unwrap();
        assert!(!debounce.update(0, true));
        assert!(!debounce.update(99, true));
        assert!(debounce.update(100, true));
    }

    #[test]
    fn recovery_also_needs_full_window() {
        let mut debounce = PfoDebounce::new(100).unwrap();
        assert!(feed(&mut debounce, 0, 100, true));
        assert!(debounce.update(200, false));
        assert!(debounce.update(299, false));
        assert!(!debounce.update(300, false));
    }

    #[test]
    fn bounce_restarts_the_window() {
        let mut debounce = PfoDebounce::new(100).unwrap();
        assert!(!debounce.update(0, true));
        assert!(!debounce.update(60, true));
        assert!(!debounce.update(80, false));
        assert!(!debounce.update(100, true));
        assert!(!debounce.update(180, true));
        assert!(debounce.update(200, true));
    }

    #[test]
    fn clock_wrap_is_handled() {
        let mut debounce = PfoDebounce::new(100).unwrap();
        assert!(!debounce.update(u32::MAX - 50, true));
        assert!(debounce.update(u32::MAX.wrapping_add(50), true));
    }
}
