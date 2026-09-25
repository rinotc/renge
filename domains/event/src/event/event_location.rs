#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventLocation(String);

impl EventLocation {
    pub fn new(location: String) -> Self {
        Self(location)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
