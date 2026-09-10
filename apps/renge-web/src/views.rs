use askama::Template;
use chrono::{DateTime, FixedOffset};
use infra_postgres_renge_orm::orm::{events as event, participants as participant};
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

#[derive(Template)]
#[template(path = "event_index.askama.html")]
pub(crate) struct EventIndexTemplate {
    page_title: &'static str,
    events: Vec<EventView>,
}

impl EventIndexTemplate {
    pub(crate) fn new(events: Vec<event::Model>) -> Self {
        Self {
            page_title: "イベント一覧",
            events: events.iter().map(EventView::from).collect(),
        }
    }
}

#[derive(Template)]
#[template(path = "event_detail.askama.html")]
pub(crate) struct EventDetailTemplate {
    page_title: String,
    event_id: Uuid,
    event: EventView,
    people: Vec<ParticipantView>,
    attendance_options: [AttendanceOption; 3],
}

impl EventDetailTemplate {
    pub(crate) fn new(event: &event::Model, people: Vec<participant::Model>) -> Self {
        Self {
            page_title: event.title.clone(),
            event_id: event.id,
            event: EventView::from(event),
            people: people.iter().map(ParticipantView::from).collect(),
            attendance_options: attendance_options(),
        }
    }
}

#[derive(Template)]
#[template(path = "event_card.askama.html")]
pub(crate) struct EventCardTemplate {
    event: EventView,
}

impl EventCardTemplate {
    pub(crate) fn new(event: &event::Model) -> Self {
        Self {
            event: EventView::from(event),
        }
    }
}

#[derive(Template)]
#[template(path = "participant_row.askama.html")]
pub(crate) struct ParticipantRowTemplate {
    event_id: Uuid,
    person: ParticipantView,
    attendance_options: [AttendanceOption; 3],
}

impl ParticipantRowTemplate {
    pub(crate) fn new(event_id: Uuid, person: &participant::Model) -> Self {
        Self {
            event_id,
            person: ParticipantView::from(person),
            attendance_options: attendance_options(),
        }
    }
}

#[derive(Template)]
#[template(path = "error.askama.html")]
pub(crate) struct ErrorTemplate {
    pub(crate) message: String,
}

struct EventView {
    id: Uuid,
    title: String,
    description: String,
    has_description: bool,
    starts_at: String,
    location: String,
}

impl From<&event::Model> for EventView {
    fn from(event: &event::Model) -> Self {
        let description = event.description.clone().unwrap_or_default();
        Self {
            id: event.id,
            title: event.title.clone(),
            has_description: !description.is_empty(),
            description,
            starts_at: date(event.starts_at),
            location: event
                .location
                .clone()
                .unwrap_or_else(|| "会場未定".to_owned()),
        }
    }
}

struct ParticipantView {
    id: Uuid,
    name: String,
    email: String,
    status: AttendanceView,
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

struct AttendanceView {
    value: &'static str,
    label: &'static str,
    class: &'static str,
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

struct AttendanceOption {
    value: &'static str,
    label: &'static str,
}

fn attendance_options() -> [AttendanceOption; 3] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_index_renders_empty_state() {
        let html = EventIndexTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("まだイベントがありません。右のフォームから作成できます。"));
    }

    #[test]
    fn event_card_escapes_user_input() {
        let html = EventCardTemplate {
            event: EventView {
                id: Uuid::nil(),
                title: "<script>alert(1)</script>".to_owned(),
                description: "説明".to_owned(),
                has_description: true,
                starts_at: "2026年9月7日 12:00".to_owned(),
                location: "会場".to_owned(),
            },
        }
        .render()
        .unwrap();

        assert!(html.contains("&#60;script&#62;alert(1)&#60;/script&#62;"));
        assert!(!html.contains("<script>alert(1)</script>"));
    }

    #[test]
    fn participant_row_selects_current_attendance() {
        let html = ParticipantRowTemplate {
            event_id: Uuid::nil(),
            person: ParticipantView {
                id: Uuid::nil(),
                name: "参加者".to_owned(),
                email: "person@example.com".to_owned(),
                status: AttendanceView::from(Attendance::Attending),
            },
            attendance_options: attendance_options(),
        }
        .render()
        .unwrap();

        assert!(html.contains("value=\"attending\" selected"));
    }
}
