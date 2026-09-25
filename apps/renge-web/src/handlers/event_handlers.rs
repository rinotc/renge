use crate::{
    error::AppError,
    handlers::is_htmx,
    presenters::event::create_event_presenter::CreateEventRequest,
    state::AppState,
    views::{EventDetailTemplate, EventIndexTemplate},
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use infra_postgres_renge_orm::orm::{events as event, participants as participant};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

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
    Form(request): Form<CreateEventRequest>,
) -> Result<Response, AppError> {
    let presentation = state.create_event_presenter.present(request).await?;

    Ok(if is_htmx(&headers) {
        Html(presentation.event_card.render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{}", presentation.event_id)).into_response()
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
