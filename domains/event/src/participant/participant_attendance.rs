#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticipantAttendance {
    Pending,
    Attending,
    Declined,
}

impl ParticipantAttendance {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "attending" => Some(Self::Attending),
            "declined" => Some(Self::Declined),
            _ => None,
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Attending => "attending",
            Self::Declined => "declined",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParticipantAttendance;

    #[test]
    fn parses_and_serializes_attendance_values() {
        for (value, attendance) in [
            ("pending", ParticipantAttendance::Pending),
            ("attending", ParticipantAttendance::Attending),
            ("declined", ParticipantAttendance::Declined),
        ] {
            assert_eq!(ParticipantAttendance::parse(value), Some(attendance));
            assert_eq!(attendance.value(), value);
        }
        assert_eq!(ParticipantAttendance::parse("unknown"), None);
    }
}
