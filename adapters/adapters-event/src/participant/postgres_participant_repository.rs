use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::participant::{
    Participant, ParticipantAttendance, ParticipantEmail, ParticipantId, ParticipantName,
    ParticipantRepository,
};
use infra_postgres_renge_orm::orm::participants;
use libs_modeling::identifier::Identifier;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

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

fn invalid_data(message: impl Into<String>) -> RepositoryError {
    Box::new(IoError::new(ErrorKind::InvalidData, message.into()))
}

fn to_domain(model: participants::Model) -> Result<Participant, RepositoryError> {
    let name = ParticipantName::try_new(model.name)
        .map_err(|error| invalid_data(format!("invalid participant name: {error:?}")))?;
    let email = ParticipantEmail::try_new(model.email)
        .map_err(|error| invalid_data(format!("invalid participant email: {error:?}")))?;
    let attendance = ParticipantAttendance::parse(&model.attendance)
        .ok_or_else(|| invalid_data("invalid participant attendance"))?;

    Ok(Participant::new(
        ParticipantId::new(model.id),
        EventId::new(model.event_id),
        name,
        email,
        attendance,
    ))
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

        models.into_iter().map(to_domain).collect()
    }

    async fn insert(&self, participant: &Participant) -> Result<(), RepositoryError> {
        participants::ActiveModel {
            id: Set(participant.id.value()),
            event_id: Set(participant.event_id.value()),
            name: Set(participant.name.as_str().to_owned()),
            email: Set(participant.email.as_str().to_owned()),
            attendance: Set(participant.attendance.value().to_owned()),
            created_at: Set(chrono::Utc::now().fixed_offset()),
        }
        .insert(&self.dbc)
        .await
        .map(|_| ())
        .map_err(database_error)
    }

    async fn find_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<Option<Participant>, RepositoryError> {
        participants::Entity::find_by_id(participant_id.value())
            .filter(participants::Column::EventId.eq(event_id.value()))
            .one(&self.dbc)
            .await
            .map_err(database_error)?
            .map(to_domain)
            .transpose()
    }

    async fn update(&self, participant: &Participant) -> Result<(), RepositoryError> {
        let model = participants::Entity::find_by_id(participant.id.value())
            .filter(participants::Column::EventId.eq(participant.event_id.value()))
            .one(&self.dbc)
            .await
            .map_err(database_error)?
            .ok_or_else(|| invalid_data("participant disappeared before update"))?;

        let mut active: participants::ActiveModel = model.into();
        active.name = Set(participant.name.as_str().to_owned());
        active.email = Set(participant.email.as_str().to_owned());
        active.attendance = Set(participant.attendance.value().to_owned());
        active
            .update(&self.dbc)
            .await
            .map(|_| ())
            .map_err(database_error)
    }

    async fn delete_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<(), RepositoryError> {
        participants::Entity::delete_many()
            .filter(participants::Column::Id.eq(participant_id.value()))
            .filter(participants::Column::EventId.eq(event_id.value()))
            .exec(&self.dbc)
            .await
            .map(|_| ())
            .map_err(database_error)
    }
}
