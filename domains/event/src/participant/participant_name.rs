#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticipantName(String);

impl ParticipantName {
    pub fn try_new(value: String) -> Result<Self, ParticipantNameError> {
        if value.trim().is_empty() {
            return Err(ParticipantNameError::BlankNotAllowed);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ParticipantName, ParticipantNameError};

    #[test]
    fn accepts_non_blank_names() {
        let name = ParticipantName::try_new("参加者".to_owned()).unwrap();

        assert_eq!(name.as_str(), "参加者");
    }

    #[test]
    fn rejects_blank_names() {
        assert_eq!(
            ParticipantName::try_new(" \t ".to_owned()),
            Err(ParticipantNameError::BlankNotAllowed)
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticipantNameError {
    BlankNotAllowed,
}
