use crate::event::event_id::EventId;
use crate::event::event_repository::RepositoryError;
use crate::participant::Participant;
use async_trait::async_trait;

#[async_trait]
pub trait ParticipantRepository: Send + Sync {
    async fn list_by_event_id(
        &self,
        event_id: &EventId,
    ) -> Result<Vec<Participant>, RepositoryError>;
}
