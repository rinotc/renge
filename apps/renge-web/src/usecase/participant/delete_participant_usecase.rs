use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::participant::{ParticipantId, ParticipantRepository};
use libs_modeling::identifier::Identifier;
use libs_usecase::usecase::usecase::UseCase;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteParticipantUseCase {
    participant_repository: Arc<dyn ParticipantRepository>,
}

impl DeleteParticipantUseCase {
    pub fn new(participant_repository: Arc<dyn ParticipantRepository>) -> Self {
        Self {
            participant_repository,
        }
    }
}

#[async_trait]
impl UseCase for DeleteParticipantUseCase {
    type Input = DeleteParticipantInput;
    type Output = DeleteParticipantOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        self.participant_repository
            .delete_by_event_and_id(&input.event_id, &input.participant_id)
            .await?;
        Ok(DeleteParticipantOutput::Deleted)
    }
}

pub struct DeleteParticipantInput {
    pub event_id: EventId,
    pub participant_id: ParticipantId,
}

impl DeleteParticipantInput {
    pub fn new(event_id: Uuid, participant_id: Uuid) -> Self {
        Self {
            event_id: EventId::new(event_id),
            participant_id: ParticipantId::new(participant_id),
        }
    }
}

pub enum DeleteParticipantOutput {
    Deleted,
}
