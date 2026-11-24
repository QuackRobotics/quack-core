// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! PFO_N (power-fail output of the HAT supervisor, active-low) debounce.
//!
//! A power-fail is reported only after PFO_N has read low continuously for the debounce
//! window, and cleared only after it has read high continuously for the same window, so
//! servo current spikes on the rail cannot toggle the state. The window is at least
//! [`MIN_DEBOUNCE`] (100 ms); shorter windows are refused.

use std::fmt;
use std::time::Duration;

pub const MIN_DEBOUNCE: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebounceTooShort(pub Duration);

impl fmt::Display for DebounceTooShort {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "PFO_N debounce {:?} is below the {:?} minimum",
            self.0, MIN_DEBOUNCE
        )
    }
}

impl std::error::Error for DebounceTooShort {}

#[derive(Debug, Clone)]
pub struct PowerFailDebounce {
    window: Duration,
    power_fail: bool,
    /// Monotonic time at which the raw pin first disagreed with `power_fail`.
    disagree_since: Option<Duration>,
}

impl PowerFailDebounce {
    pub fn new(window: Duration) -> Result<Self, DebounceTooShort> {
        if window < MIN_DEBOUNCE {
            return Err(DebounceTooShort(window));
        }
        Ok(Self {
            window,
            power_fail: false,
            disagree_since: None,
        })
    }

    /// Feed one PFO_N sample (`pin_high` = raw level) taken at monotonic time `now`.
    /// Returns the debounced power-fail state.
    pub fn update(&mut self, pin_high: bool, now: Duration) -> bool {
        let raw_power_fail = !pin_high;
        if raw_power_fail == self.power_fail {
            self.disagree_since = None;
            return self.power_fail;
        }
        let since = *self.disagree_since.get_or_insert(now);
        if now.saturating_sub(since) >= self.window {
            self.power_fail = raw_power_fail;
            self.disagree_since = None;
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

    const TICK: Duration = Duration::from_millis(20);

    fn milliseconds(value: u64) -> Duration {
        Duration::from_millis(value)
    }

    #[test]
    fn rejects_window_below_100_ms() {
        assert_eq!(
            PowerFailDebounce::new(milliseconds(99)).unwrap_err(),
            DebounceTooShort(milliseconds(99))
        );
        assert!(PowerFailDebounce::new(MIN_DEBOUNCE).is_ok());
    }

    #[test]
    fn low_for_99_ms_is_ignored() {
        let mut debounce = PowerFailDebounce::new(MIN_DEBOUNCE).unwrap();
        assert!(!debounce.update(false, milliseconds(0)));
        assert!(!debounce.update(false, milliseconds(99)));
        assert!(!debounce.update(true, milliseconds(100)));
        assert!(!debounce.power_fail());
    }

    #[test]
    fn low_for_100_ms_asserts() {
        let mut debounce = PowerFailDebounce::new(MIN_DEBOUNCE).unwrap();
        assert!(!debounce.update(false, milliseconds(1000)));
        assert!(debounce.update(false, milliseconds(1100)));
    }

    #[test]
    fn bouncing_pin_never_asserts() {
        let mut debounce = PowerFailDebounce::new(MIN_DEBOUNCE).unwrap();
        for step in 0..100u32 {
            let pin_high = step % 5 == 4;
            assert!(!debounce.update(pin_high, TICK * step), "step {step}");
        }
    }

    #[test]
    fn clearing_also_needs_100_ms_high() {
        let mut debounce = PowerFailDebounce::new(MIN_DEBOUNCE).unwrap();
        debounce.update(false, milliseconds(0));
        assert!(debounce.update(false, milliseconds(100)));
        assert!(debounce.update(true, milliseconds(120)));
        assert!(debounce.update(false, milliseconds(200)));
        assert!(debounce.update(true, milliseconds(220)));
        assert!(debounce.update(true, milliseconds(319)));
        assert!(!debounce.update(true, milliseconds(320)));
    }

    #[test]
    fn longer_window_is_honoured_at_50_hz() {
        let mut debounce = PowerFailDebounce::new(milliseconds(150)).unwrap();
        let states: Vec<bool> = (0..10u32)
            .map(|step| debounce.update(false, TICK * step))
            .collect();
        let first_assert = states.iter().position(|state| *state).unwrap();
        assert_eq!(TICK * first_assert as u32, milliseconds(160));
    }
}
