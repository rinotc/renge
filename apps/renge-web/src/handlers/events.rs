use crate::{
    error::AppError,
    handlers::{is_htmx, optional, parse_datetime, required},
    state::AppState,
    views::{EventCardTemplate, EventDetailTemplate, EventIndexTemplate},
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::Utc;
use infra_postgres_renge_orm::orm::{events as event, participants as participant};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct EventForm {
    title: String,
    description: String,
    starts_at: String,
    location: String,
}

pub(crate) async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let events = event::Entity::find()
        .order_by_desc(event::Column::StartsAt)
        .all(&state.dbc)
        .await?;

    Ok(Html(EventIndexTemplate::new(events).render()?))
}

pub(crate) async fn show_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let event = find_event(&state.dbc, id).await?;
    let people = participant::Entity::find()
        .filter(participant::Column::EventId.eq(id))
        .order_by_asc(participant::Column::CreatedAt)
        .all(&state.dbc)
        .await?;

    Ok(Html(EventDetailTemplate::new(&event, people).render()?))
}

pub(crate) async fn create_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<EventForm>,
) -> Result<Response, AppError> {
    let model = event::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(required(&form.title, "イベント名")?.into()),
        description: Set(optional(&form.description)),
        starts_at: Set(parse_datetime(&form.starts_at)?),
        location: Set(optional(&form.location)),
        created_at: Set(Utc::now().fixed_offset()),
    }
    .insert(&state.dbc)
    .await?;

    Ok(if is_htmx(&headers) {
        Html(EventCardTemplate::new(&model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{}", model.id)).into_response()
    })
}

pub(crate) async fn delete_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    event::Entity::delete_by_id(id).exec(&state.dbc).await?;

    Ok(if is_htmx(&headers) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Redirect::to("/").into_response()
    })
}

pub(crate) async fn find_event(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<event::Model, AppError> {
    event::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)
}
