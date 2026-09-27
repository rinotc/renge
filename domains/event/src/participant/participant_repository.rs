use crate::event::event_id::EventId;
use crate::event::event_repository::RepositoryError;
use crate::participant::{Participant, ParticipantId};
use async_trait::async_trait;

#[async_trait]
pub trait ParticipantRepository: Send + Sync {
    async fn list_by_event_id(
        &self,
        event_id: &EventId,
    ) -> Result<Vec<Participant>, RepositoryError>;

    async fn insert(&self, participant: &Participant) -> Result<(), RepositoryError>;

    async fn find_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<Option<Participant>, RepositoryError>;

    async fn update(&self, participant: &Participant) -> Result<(), RepositoryError>;

    async fn delete_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<(), RepositoryError>;
}
