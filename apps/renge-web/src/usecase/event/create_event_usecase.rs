use crate::usecase::event::create_event_usecase::CreateEventOutput::Created;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use domains_event::event::Event;
use domains_event::event::event_description::EventDescription;
use domains_event::event::event_location::EventLocation;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use domains_event::event::event_title::EventTitle;
use libs_modeling::id_provider::IdProvider;
use libs_usecase::usecase::usecase::UseCase;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateEventUseCase {
    id_provider: Arc<dyn IdProvider<Uuid>>,
    event_repository: Arc<dyn EventRepository>,
}

impl CreateEventUseCase {
    pub fn new(
        id_provider: Arc<dyn IdProvider<Uuid>>,
        event_repository: Arc<dyn EventRepository>,
    ) -> Self {
        Self {
            id_provider,
            event_repository,
        }
    }
}

#[async_trait]
impl UseCase for CreateEventUseCase {
    type Input = CreateEventInput;
    type Output = CreateEventOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let event = Event::create(
            &*self.id_provider,
            input.title,
            input.description,
            input.starts_at,
            input.location,
        );
        self.event_repository.insert(&event).await?;
        Ok(Created { event })
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateEventInput {
    pub title: EventTitle,
    pub description: Option<EventDescription>,
    pub starts_at: DateTime<FixedOffset>,
    pub location: Option<EventLocation>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateEventOutput {
    Created { event: Event },
}
