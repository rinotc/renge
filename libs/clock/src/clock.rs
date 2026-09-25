use chrono::DateTime;
use std::time::SystemTime;

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}

pub struct SystemClock {}

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

impl SystemClock {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct FixedClock {
    now: SystemTime,
}

impl Clock for FixedClock {
    fn now(&self) -> SystemTime {
        self.now
    }
}

impl FixedClock {
    pub fn new(now: SystemTime) -> Self {
        Self { now }
    }

    pub fn fixed<Tz: chrono::TimeZone>(date_time: DateTime<Tz>) -> Self {
        let now = date_time.into();
        Self::new(now)
    }
}
