use async_trait::async_trait;
use chrono::Utc;
use domains_event::event::Event;
use domains_event::event::event_description::EventDescription;
use domains_event::event::event_id::EventId;
use domains_event::event::event_location::EventLocation;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use domains_event::event::event_title::EventTitle;
use infra_postgres_renge_orm::orm::events;
use libs_modeling::identifier::Identifier;
use libs_paging::paging::offset_paged::OffsetPaged;
use libs_paging::paging::offset_paging::OffsetPaging;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, QuerySelect, Set,
};
use std::io::{Error as IoError, ErrorKind};

pub struct PostgresEventRepository {
    dbc: DatabaseConnection,
}

impl PostgresEventRepository {
    pub fn new(dbc: DatabaseConnection) -> Self {
        Self { dbc }
    }
}

fn database_error(error: DbErr) -> RepositoryError {
    Box::new(error)
}

fn invalid_data(message: impl Into<String>) -> RepositoryError {
    Box::new(IoError::new(ErrorKind::InvalidData, message.into()))
}

fn to_domain(model: events::Model) -> Result<Event, RepositoryError> {
    let title = EventTitle::try_new(model.title)
        .map_err(|error| invalid_data(format!("invalid event title: {error:?}")))?;
    let description = model
        .description
        .map(|value| {
            EventDescription::try_new(value)
                .map_err(|error| invalid_data(format!("invalid event description: {error:?}")))
        })
        .transpose()?;
    let location = model.location.map(EventLocation::new);

    Ok(Event::new(
        EventId::new(model.id),
        title,
        description,
        model.starts_at,
        location,
    ))
}

fn description_value(event: &Event) -> Option<String> {
    event
        .description
        .as_ref()
        .map(|description| description.as_str().to_owned())
}

fn location_value(event: &Event) -> Option<String> {
    event
        .location
        .as_ref()
        .map(|location| location.as_str().to_owned())
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
    async fn find_by_id(&self, id: &EventId) -> Result<Option<Event>, RepositoryError> {
        events::Entity::find_by_id(id.value())
            .one(&self.dbc)
            .await
            .map_err(database_error)?
            .map(to_domain)
            .transpose()
    }

    async fn list(&self, paging: OffsetPaging) -> Result<OffsetPaged<Event>, RepositoryError> {
        let models = events::Entity::find()
            .order_by_desc(events::Column::StartsAt)
            .order_by_desc(events::Column::Id)
            .offset(u64::from(paging.offset))
            .limit(u64::from(paging.limit))
            .all(&self.dbc)
            .await
            .map_err(database_error)?;
        let events = models
            .into_iter()
            .map(to_domain)
            .collect::<Result<_, _>>()?;

        Ok(OffsetPaged::new(events, paging))
    }

    async fn insert(&self, event: &Event) -> Result<(), RepositoryError> {
        events::ActiveModel {
            id: Set(event.id.value()),
            title: Set(event.title.as_str().to_owned()),
            description: Set(description_value(event)),
            starts_at: Set(event.start_at),
            location: Set(location_value(event)),
            created_at: Set(Utc::now().fixed_offset()),
        }
        .insert(&self.dbc)
        .await
        .map(|_| ())
        .map_err(database_error)
    }

    async fn update(&self, event: &Event) -> Result<(), RepositoryError> {
        events::ActiveModel {
            id: Set(event.id.value()),
            title: Set(event.title.as_str().to_owned()),
            description: Set(description_value(event)),
            starts_at: Set(event.start_at),
            location: Set(location_value(event)),
            ..Default::default()
        }
        .update(&self.dbc)
        .await
        .map(|_| ())
        .map_err(database_error)
    }

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError> {
        events::Entity::delete_by_id(id.value())
            .exec(&self.dbc)
            .await
            .map(|_| ())
            .map_err(database_error)
    }
}
