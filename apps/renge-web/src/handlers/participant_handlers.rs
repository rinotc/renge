use crate::{
    error::AppError,
    handlers::is_htmx,
    presenters::participant::{
        create_participant_presenter::CreateParticipantRequest,
        update_participant_attendance_presenter::UpdateParticipantAttendanceRequest,
    },
    state::AppState,
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use uuid::Uuid;

pub(crate) async fn create_participant(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(event_id): Path<Uuid>,
    Form(request): Form<CreateParticipantRequest>,
) -> Result<Response, AppError> {
    let presentation = state
        .create_participant_presenter
        .present(event_id, request)
        .await?;

    Ok(if is_htmx(&headers) {
        Html(presentation.participant_row.render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{}", presentation.event_id)).into_response()
    })
}

pub(crate) async fn update_attendance(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
    Form(request): Form<UpdateParticipantAttendanceRequest>,
) -> Result<Response, AppError> {
    let presentation = state
        .update_participant_attendance_presenter
        .present(event_id, participant_id, request)
        .await?;

    Ok(if is_htmx(&headers) {
        Html(presentation.participant_row.render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}

pub(crate) async fn delete_participant(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, AppError> {
    state
        .delete_participant_presenter
        .present(event_id, participant_id)
        .await?;

    Ok(if is_htmx(&headers) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}
