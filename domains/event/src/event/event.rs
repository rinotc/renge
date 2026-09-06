use crate::event::event_description::EventDescription;
use crate::event::event_id::EventId;
use crate::event::event_title::EventTitle;

#[derive(Clone)]
pub struct Event {
    pub id: EventId,
    pub title: EventTitle,
    pub description: EventDescription,
}

