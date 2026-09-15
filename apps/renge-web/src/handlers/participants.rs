use crate::{
    error::AppError,
    handlers::{events::find_event, is_htmx, required},
    state::AppState,
    views::{Attendance, ParticipantRowTemplate},
};
use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::Utc;
use infra_postgres_renge_orm::orm::participants as participant;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct ParticipantForm {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub(crate) struct AttendanceForm {
    attendance: String,
}

pub(crate) async fn create_participant(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Form(form): Form<ParticipantForm>,
) -> Result<Response, AppError> {
    find_event(&state.dbc, id).await?;

    let model = participant::ActiveModel {
        id: Set(Uuid::new_v4()),
        event_id: Set(id),
        name: Set(required(&form.name, "氏名")?.into()),
        email: Set(required(&form.email, "メールアドレス")?.into()),
        attendance: Set("pending".into()),
        created_at: Set(Utc::now().fixed_offset()),
    }
    .insert(&state.dbc)
    .await?;

    Ok(if is_htmx(&headers) {
        Html(ParticipantRowTemplate::new(id, &model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{id}")).into_response()
    })
}

pub(crate) async fn update_attendance(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
    Form(form): Form<AttendanceForm>,
) -> Result<Response, AppError> {
    let value = Attendance::parse(&form.attendance)
        .ok_or_else(|| AppError::BadRequest("不正な出欠状態です。".into()))?;
    let model = participant::Entity::find_by_id(participant_id)
        .filter(participant::Column::EventId.eq(event_id))
        .one(&state.dbc)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut active: participant::ActiveModel = model.into();
    active.attendance = Set(value.value().into());
    let model = active.update(&state.dbc).await?;

    Ok(if is_htmx(&headers) {
        Html(ParticipantRowTemplate::new(event_id, &model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}

pub(crate) async fn delete_participant(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, AppError> {
    participant::Entity::delete_many()
        .filter(participant::Column::Id.eq(participant_id))
        .filter(participant::Column::EventId.eq(event_id))
        .exec(&state.dbc)
        .await?;

    Ok(if is_htmx(&headers) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}
