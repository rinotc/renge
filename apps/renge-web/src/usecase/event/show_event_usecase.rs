use async_trait::async_trait;
use domains_event::event::Event;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use domains_event::event::participant::Participant;
use domains_event::event::participant_repository::ParticipantRepository;
use libs_modeling::identifier::Identifier;
use std::sync::Arc;
use uuid::Uuid;

pub struct ShowEventUseCase {
    event_repository: Arc<dyn EventRepository>,
    participant_repository: Arc<dyn ParticipantRepository>,
}

impl ShowEventUseCase {
    pub fn new(
        event_repository: Arc<dyn EventRepository>,
        participant_repository: Arc<dyn ParticipantRepository>,
    ) -> Self {
        Self {
            event_repository,
            participant_repository,
        }
    }
}

#[async_trait]
impl libs_usecase::usecase::usecase::UseCase for ShowEventUseCase {
    type Input = ShowEventInput;
    type Output = Option<ShowEventOutput>;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let Some(event) = self.event_repository.find_by_id(&input.event_id).await? else {
            return Ok(None);
        };
        let participants = self
            .participant_repository
            .list_by_event_id(&input.event_id)
            .await?;

        Ok(Some(ShowEventOutput {
            event,
            participants,
        }))
    }
}

pub struct ShowEventInput {
    pub event_id: EventId,
}

impl ShowEventInput {
    pub fn new(event_id: Uuid) -> Self {
        Self {
            event_id: EventId::new(event_id),
        }
    }
}

pub struct ShowEventOutput {
    pub event: Event,
    pub participants: Vec<Participant>,
}
