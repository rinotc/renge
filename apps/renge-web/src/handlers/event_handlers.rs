use crate::{
    error::AppError,
    handlers::is_htmx,
    presenters::event::{
        create_event_presenter::CreateEventRequest, index_events_presenter::EventIndexRequest,
    },
    state::AppState,
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use infra_postgres_renge_orm::orm::events as event;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub(crate) async fn index(
    State(state): State<AppState>,
    Query(request): Query<EventIndexRequest>,
) -> Result<Html<String>, AppError> {
    let presentation = state.index_events_presenter.present(request).await?;

    Ok(Html(presentation.template.render()?))
}

pub(crate) async fn show_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let presentation = state.show_event_presenter.present(id).await?;

    Ok(Html(presentation.template.render()?))
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
    state.delete_event_presenter.present(id).await?;

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
