use crate::event::event_description::EventDescription;
use crate::event::event_id::EventId;
use crate::event::event_location::EventLocation;
use crate::event::event_title::EventTitle;
use chrono::{DateTime, FixedOffset};
use libs_modeling::id_provider::IdProvider;
use libs_modeling::identifier::Identifier;
use uuid::Uuid;

#[derive(Clone)]
pub struct Event {
    pub id: EventId,
    pub title: EventTitle,
    pub description: EventDescription,
    pub start_at: DateTime<FixedOffset>,
    pub location: EventLocation,
}

impl Event {
    pub fn create(
        uuid_provider: &dyn IdProvider<Uuid>,
        title: EventTitle,
        description: EventDescription,
        start_at: DateTime<FixedOffset>,
        location: EventLocation,
    ) -> Self {
        Self {
            id: EventId::generate(uuid_provider),
            title,
            description,
            start_at,
            location,
        }
    }

    pub fn new(
        id: EventId,
        title: EventTitle,
        description: EventDescription,
        start_at: DateTime<FixedOffset>,
        location: EventLocation,
    ) -> Self {
        Self {
            id,
            title,
            description,
            start_at,
            location,
        }
    }
}
