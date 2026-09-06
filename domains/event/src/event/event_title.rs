const MAX_TITLE_LENGTH: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTitle(String);

impl EventTitle {
    pub fn new(title: String) -> Self {
        Self::validate(&title).map(|_| Self(title)).unwrap()
    }

    pub fn try_new(title: String) -> Result<Self, EventTitleError> {
        Self::validate(&title)?;
        Ok(Self(title))
    }

    /// イベントタイトルは100文字以内
    /// 空白を許容しない
    fn validate(value: &str) -> Result<(), EventTitleError> {
        if value.is_empty() {
            return Err(EventTitleError::BlankNotAllowed);
        }
        if value.len() > MAX_TITLE_LENGTH {
            return Err(EventTitleError::TooLong);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTitleError {
    BlankNotAllowed,
    TooLong,
}
