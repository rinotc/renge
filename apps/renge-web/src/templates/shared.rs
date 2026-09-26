use chrono::{DateTime, FixedOffset};
use domains_event::event::Event as DomainEvent;
use domains_event::participant::Participant as DomainParticipant;
use infra_postgres_renge_orm::orm::participants as participant;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Attendance {
    Pending,
    Attending,
    Declined,
}

impl Attendance {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "attending" => Some(Self::Attending),
            "declined" => Some(Self::Declined),
            _ => None,
        }
    }

    pub(crate) fn value(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Attending => "attending",
            Self::Declined => "declined",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Pending => "未定",
            Self::Attending => "参加予定",
            Self::Declined => "欠席",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Pending => "badge-warning",
            Self::Attending => "badge-success",
            Self::Declined => "badge-ghost",
        }
    }
}

pub(super) struct EventView {
    pub(super) id: Uuid,
    pub(super) title: String,
    pub(super) description: String,
    pub(super) has_description: bool,
    pub(super) starts_at: String,
    pub(super) location: String,
}

impl From<&DomainEvent> for EventView {
    fn from(event: &DomainEvent) -> Self {
        let description = event
            .description
            .as_ref()
            .map(|description| description.as_str().to_owned())
            .unwrap_or_default();
        Self {
            id: event.id.value(),
            title: event.title.as_str().to_owned(),
            has_description: !description.is_empty(),
            description,
            starts_at: date(event.start_at),
            location: event
                .location
                .as_ref()
                .map(|location| location.as_str().to_owned())
                .unwrap_or_else(|| "会場未定".to_owned()),
        }
    }
}

pub(super) struct ParticipantView {
    pub(super) id: Uuid,
    pub(super) name: String,
    pub(super) email: String,
    pub(super) status: AttendanceView,
}

impl From<&participant::Model> for ParticipantView {
    fn from(person: &participant::Model) -> Self {
        let status = Attendance::parse(&person.attendance).unwrap_or(Attendance::Pending);
        Self {
            id: person.id,
            name: person.name.clone(),
            email: person.email.clone(),
            status: AttendanceView::from(status),
        }
    }
}

impl From<&DomainParticipant> for ParticipantView {
    fn from(person: &DomainParticipant) -> Self {
        let status = Attendance::parse(&person.attendance).unwrap_or(Attendance::Pending);
        Self {
            id: person.id,
            name: person.name.clone(),
            email: person.email.clone(),
            status: AttendanceView::from(status),
        }
    }
}

pub(super) struct AttendanceView {
    pub(super) value: &'static str,
    pub(super) label: &'static str,
    pub(super) class: &'static str,
}

impl From<Attendance> for AttendanceView {
    fn from(status: Attendance) -> Self {
        Self {
            value: status.value(),
            label: status.label(),
            class: status.class(),
        }
    }
}

pub(super) struct AttendanceOption {
    pub(super) value: &'static str,
    pub(super) label: &'static str,
}

pub(super) fn attendance_options() -> [AttendanceOption; 3] {
    [
        AttendanceOption {
            value: Attendance::Pending.value(),
            label: Attendance::Pending.label(),
        },
        AttendanceOption {
            value: Attendance::Attending.value(),
            label: Attendance::Attending.label(),
        },
        AttendanceOption {
            value: Attendance::Declined.value(),
            label: Attendance::Declined.label(),
        },
    ]
}

fn date(value: DateTime<FixedOffset>) -> String {
    value
        .with_timezone(&chrono::Local)
        .format("%Y年%-m月%-d日 %-H:%M")
        .to_string()
}
