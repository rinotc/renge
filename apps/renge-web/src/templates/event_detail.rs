use super::shared::{AttendanceOption, EventView, ParticipantView, attendance_options};
use askama::Template;
use domains_event::event::Event as DomainEvent;
use domains_event::participant::Participant as DomainParticipant;
use uuid::Uuid;

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
    pub(crate) fn new(event: &DomainEvent, people: Vec<DomainParticipant>) -> Self {
        Self {
            page_title: event.title.as_str().to_owned(),
            event_id: event.id.value(),
            event: EventView::from(event),
            people: people.iter().map(ParticipantView::from).collect(),
            attendance_options: attendance_options(),
        }
    }
}
