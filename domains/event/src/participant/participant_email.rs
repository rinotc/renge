#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticipantEmail(String);

impl ParticipantEmail {
    pub fn try_new(value: String) -> Result<Self, ParticipantEmailError> {
        if value.trim().is_empty() {
            return Err(ParticipantEmailError::BlankNotAllowed);
        }

        let mut parts = value.split('@');
        let local = parts.next().unwrap_or_default();
        let domain = parts.next().unwrap_or_default();
        let has_extra_part = parts.next().is_some();
        if local.is_empty()
            || domain.is_empty()
            || has_extra_part
            || value.chars().any(char::is_whitespace)
            || local.starts_with('.')
            || local.ends_with('.')
            || local.contains("..")
            || domain.starts_with('.')
            || domain.ends_with('.')
            || domain.contains("..")
            || !local.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || ".!#$%&'*+-/=?^_`{|}~"
                        .bytes()
                        .any(|allowed| allowed == byte)
            })
            || domain.split('.').any(|label| {
                label.is_empty()
                    || label.starts_with('-')
                    || label.ends_with('-')
                    || !label
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            })
        {
            return Err(ParticipantEmailError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ParticipantEmail, ParticipantEmailError};

    #[test]
    fn accepts_basic_email_addresses() {
        for value in [
            "person@example.com",
            "first.last+tag@local-host.test",
            "person@localhost",
        ] {
            assert!(
                ParticipantEmail::try_new(value.to_owned()).is_ok(),
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_malformed_email_addresses() {
        for value in [
            "@example.com",
            "person@",
            "person@@example.com",
            "first..last@example.com",
            "person@-example.com",
            "person@example-.com",
            "person@exam ple.com",
            "person@example..com",
        ] {
            assert_eq!(
                ParticipantEmail::try_new(value.to_owned()),
                Err(ParticipantEmailError::InvalidFormat),
                "{value}"
            );
        }
    }

    #[test]
    fn rejects_blank_email_addresses() {
        assert_eq!(
            ParticipantEmail::try_new("  ".to_owned()),
            Err(ParticipantEmailError::BlankNotAllowed)
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticipantEmailError {
    BlankNotAllowed,
    InvalidFormat,
}
