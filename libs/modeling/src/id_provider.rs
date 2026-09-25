use libs_clock::clock::Clock;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{ContextV7, Timestamp, Uuid};

pub trait IdProvider<ID>: Send + Sync {
    fn generate(&self) -> ID;
}

pub struct UuidIdProvider {
    clock: dyn Clock,
}

impl IdProvider<Uuid> for UuidIdProvider {
    fn generate(&self) -> Uuid {
        Uuid::new_v7(from_system_time(self.clock.now()))
    }
}

fn from_system_time(time: SystemTime) -> Timestamp {
    let duration = time
        .duration_since(UNIX_EPOCH)
        .expect("system time is before UNIX_EPOCH");

    Timestamp::from_unix(
        ContextV7::new(),
        duration.as_secs(),
        duration.subsec_nanos(),
    )
}
