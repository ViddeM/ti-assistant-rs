use chrono::{DateTime, TimeDelta, TimeZone, Utc};

/// A fake clock that hands out timestamps for applied events.
///
/// Every accepted event advances the clock by `step`, so scenarios are deterministic without
/// having to think about time unless they want to.
#[derive(Debug, Clone)]
pub struct Clock {
    now: DateTime<Utc>,
    step: TimeDelta,
}

impl Clock {
    /// The time at which every scenario starts.
    pub fn base() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 1, 1, 12, 0, 0).unwrap()
    }

    /// A clock starting at [Clock::base] that advances `step` per event.
    pub fn new(step: TimeDelta) -> Self {
        Self {
            now: Self::base(),
            step,
        }
    }

    /// The timestamp the next event will get.
    pub fn now(&self) -> DateTime<Utc> {
        self.now
    }

    /// Move the clock forward. Time can never go backwards, the game logic rejects that.
    #[track_caller]
    pub fn advance(&mut self, delta: TimeDelta) {
        assert!(delta >= TimeDelta::zero(), "the clock can't go backwards");
        self.now += delta;
    }

    /// Advance by one step, called after each accepted event.
    pub(crate) fn tick(&mut self) {
        self.now += self.step;
    }
}
