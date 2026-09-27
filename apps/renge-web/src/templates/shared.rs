use chrono::{DateTime, FixedOffset};
use domains_event::event::Event as DomainEvent;
use domains_event::participant::{Participant as DomainParticipant, ParticipantAttendance};
use uuid::Uuid;

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

impl From<&DomainParticipant> for ParticipantView {
    fn from(person: &DomainParticipant) -> Self {
        Self {
            id: person.id.value(),
            name: person.name.as_str().to_owned(),
            email: person.email.as_str().to_owned(),
            status: AttendanceView::from(person.attendance),
        }
    }
}

pub(super) struct AttendanceView {
    pub(super) value: &'static str,
    pub(super) label: &'static str,
    pub(super) class: &'static str,
}

impl From<ParticipantAttendance> for AttendanceView {
    fn from(status: ParticipantAttendance) -> Self {
        Self {
            value: status.value(),
            label: attendance_label(status),
            class: attendance_class(status),
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
            value: ParticipantAttendance::Pending.value(),
            label: attendance_label(ParticipantAttendance::Pending),
        },
        AttendanceOption {
            value: ParticipantAttendance::Attending.value(),
            label: attendance_label(ParticipantAttendance::Attending),
        },
        AttendanceOption {
            value: ParticipantAttendance::Declined.value(),
            label: attendance_label(ParticipantAttendance::Declined),
        },
    ]
}

fn attendance_label(attendance: ParticipantAttendance) -> &'static str {
    match attendance {
        ParticipantAttendance::Pending => "未定",
        ParticipantAttendance::Attending => "参加予定",
        ParticipantAttendance::Declined => "欠席",
    }
}

fn attendance_class(attendance: ParticipantAttendance) -> &'static str {
    match attendance {
        ParticipantAttendance::Pending => "badge-warning",
        ParticipantAttendance::Attending => "badge-success",
        ParticipantAttendance::Declined => "badge-ghost",
    }
}

fn date(value: DateTime<FixedOffset>) -> String {
    value
        .with_timezone(&chrono::Local)
        .format("%Y年%-m月%-d日 %-H:%M")
        .to_string()
}
