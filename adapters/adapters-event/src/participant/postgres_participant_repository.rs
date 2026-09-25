use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::event::participant::Participant;
use domains_event::event::participant_repository::ParticipantRepository;
use infra_postgres_renge_orm::orm::participants;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use std::error::Error;

pub struct PostgresParticipantRepository {
    dbc: DatabaseConnection,
}

impl PostgresParticipantRepository {
    pub fn new(dbc: DatabaseConnection) -> Self {
        Self { dbc }
    }
}

fn database_error(error: impl Error + Send + Sync + 'static) -> RepositoryError {
    Box::new(error)
}

#[async_trait]
impl ParticipantRepository for PostgresParticipantRepository {
    async fn list_by_event_id(
        &self,
        event_id: &EventId,
    ) -> Result<Vec<Participant>, RepositoryError> {
        let models = participants::Entity::find()
            .filter(participants::Column::EventId.eq(event_id.value()))
            .order_by_asc(participants::Column::CreatedAt)
            .all(&self.dbc)
            .await
            .map_err(database_error)?;

        Ok(models
            .into_iter()
            .map(|model| Participant::new(model.id, model.name, model.email, model.attendance))
            .collect())
    }
}
