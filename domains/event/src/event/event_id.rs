use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}
