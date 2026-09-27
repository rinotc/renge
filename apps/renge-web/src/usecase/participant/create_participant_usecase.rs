use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use domains_event::participant::{Participant, ParticipantName, ParticipantRepository};
use libs_modeling::id_provider::IdProvider;
use libs_shared_primitive::email::Email;
use libs_usecase::usecase::usecase::UseCase;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateParticipantUseCase {
    id_provider: Arc<dyn IdProvider<Uuid>>,
    event_repository: Arc<dyn EventRepository>,
    participant_repository: Arc<dyn ParticipantRepository>,
}

impl CreateParticipantUseCase {
    pub fn new(
        id_provider: Arc<dyn IdProvider<Uuid>>,
        event_repository: Arc<dyn EventRepository>,
        participant_repository: Arc<dyn ParticipantRepository>,
    ) -> Self {
        Self {
            id_provider,
            event_repository,
            participant_repository,
        }
    }
}

#[async_trait]
impl UseCase for CreateParticipantUseCase {
    type Input = CreateParticipantInput;
    type Output = CreateParticipantOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        if self
            .event_repository
            .find_by_id(&input.event_id)
            .await?
            .is_none()
        {
            return Ok(CreateParticipantOutput::EventNotFound);
        }

        let participant = Participant::create(
            self.id_provider.as_ref(),
            input.event_id,
            input.name,
            input.email,
        );
        self.participant_repository.insert(&participant).await?;
        Ok(CreateParticipantOutput::Created { participant })
    }
}

pub struct CreateParticipantInput {
    pub event_id: EventId,
    pub name: ParticipantName,
    pub email: Email,
}

pub enum CreateParticipantOutput {
    Created { participant: Participant },
    EventNotFound,
}
