const MAX_DESCRIPTION_LENGTH: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDescription(String);

impl EventDescription {
    pub fn new(description: String) -> Self {
        Self::validate(&description).map(|_| Self(description)).unwrap()
    }

    pub fn try_new(description: String) -> Result<Self, EventDescriptionError> {
        Self::validate(&description)?;
        Ok(Self(description))
    }

    /// イベント説明は1000文字以内
    /// 空白を許容しない
    fn validate(value: &str) -> Result<(), EventDescriptionError> {
        if value.is_empty() {
            return Err(EventDescriptionError::BlankNotAllowed);
        }
        if value.len() > MAX_DESCRIPTION_LENGTH {
            return Err(EventDescriptionError::TooLong);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventDescriptionError {
    BlankNotAllowed,
    TooLong,
}
