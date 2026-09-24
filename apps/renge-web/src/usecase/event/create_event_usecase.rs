use crate::usecase::event::create_event_usecase::CreateEventOutput::Created;
use chrono::{DateTime, FixedOffset};
use domains_event::event::Event;
use domains_event::event::event_description::EventDescription;
use domains_event::event::event_id::EventId;
use domains_event::event::event_location::EventLocation;
use domains_event::event::event_repository::EventRepository;
use domains_event::event::event_title::EventTitle;
use libs_modeling::id_provider::IdProvider;
use libs_usecase::usecase::usecase::UseCase;
use uuid::Uuid;

pub struct CreateEventUseCase {
    id_provider: Box<dyn IdProvider<Uuid>>,
    event_repository: Box<dyn EventRepository>,
}

impl UseCase for CreateEventUseCase {
    type Input = CreateEventInput;
    type Output = CreateEventOutput;

    fn handle(&self, input: Self::Input) -> Self::Output {
        let event = Event::create(
            &*self.id_provider,
            input.title,
            input.description,
            input.starts_at,
            input.location,
        );
        self.event_repository.insert(&event);
        Created {
            event_id: event.id.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateEventInput {
    pub title: EventTitle,
    pub description: EventDescription,
    pub starts_at: DateTime<FixedOffset>,
    pub location: EventLocation,
}

#[derive(Debug, PartialEq)]
pub enum CreateEventOutput {
    Created { event_id: EventId },
}
