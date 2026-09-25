use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use libs_modeling::identifier::Identifier;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteEventUseCase {
    event_repository: Arc<dyn EventRepository>,
}

impl DeleteEventUseCase {
    pub fn new(event_repository: Arc<dyn EventRepository>) -> Self {
        Self { event_repository }
    }
}

#[async_trait]
impl libs_usecase::usecase::usecase::UseCase for DeleteEventUseCase {
    type Input = DeleteEventInput;
    type Output = DeleteEventOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        self.event_repository.delete(&input.event_id).await?;
        Ok(DeleteEventOutput::Deleted)
    }
}

pub struct DeleteEventInput {
    pub event_id: EventId,
}

impl DeleteEventInput {
    pub fn new(event_id: Uuid) -> Self {
        Self {
            event_id: EventId::new(event_id),
        }
    }
}

pub enum DeleteEventOutput {
    Deleted,
}
