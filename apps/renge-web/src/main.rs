mod state;

use crate::state::AppState;
use askama::Template;
use axum::{
    Form, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use infra_postgres_renge_orm::orm::{events as event, participants as participant};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Deserialize)]
struct EventForm {
    title: String,
    description: String,
    starts_at: String,
    location: String,
}
#[derive(Deserialize)]
struct ParticipantForm {
    name: String,
    email: String,
}
#[derive(Deserialize)]
struct AttendanceForm {
    attendance: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Attendance {
    Pending,
    Attending,
    Declined,
}
impl Attendance {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "attending" => Some(Self::Attending),
            "declined" => Some(Self::Declined),
            _ => None,
        }
    }
    fn value(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Attending => "attending",
            Self::Declined => "declined",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Pending => "未定",
            Self::Attending => "参加予定",
            Self::Declined => "欠席",
        }
    }
    fn class(self) -> &'static str {
        match self {
            Self::Pending => "badge-warning",
            Self::Attending => "badge-success",
            Self::Declined => "badge-ghost",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "renge=debug,tower_http=info".into()),
        )
        .init();
    let app = Router::new()
        .route("/", get(index))
        .route("/events", post(create_event))
        .route("/events/{id}", get(show_event).delete(delete_event))
        .route("/events/{id}/participants", post(create_participant))
        .route(
            "/events/{id}/participants/{participant_id}/attendance",
            post(update_attendance),
        )
        .route(
            "/events/{id}/participants/{participant_id}",
            axum::routing::delete(delete_participant),
        )
        .with_state(AppState::from_env().await?)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("http://localhost:3000 で蓮華を起動しました");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index(State(s): State<AppState>) -> Result<Html<String>, AppError> {
    let events = event::Entity::find()
        .order_by_desc(event::Column::StartsAt)
        .all(&s.dbc)
        .await?;
    Ok(Html(EventIndexTemplate::new(events).render()?))
}
async fn show_event(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let event = find_event(&s.dbc, id).await?;
    let people = participant::Entity::find()
        .filter(participant::Column::EventId.eq(id))
        .order_by_asc(participant::Column::CreatedAt)
        .all(&s.dbc)
        .await?;
    Ok(Html(EventDetailTemplate::new(&event, people).render()?))
}
async fn create_event(
    State(s): State<AppState>,
    headers: HeaderMap,
    Form(f): Form<EventForm>,
) -> Result<Response, AppError> {
    let model = event::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(required(&f.title, "イベント名")?.into()),
        description: Set(optional(&f.description)),
        starts_at: Set(parse_datetime(&f.starts_at)?),
        location: Set(optional(&f.location)),
        created_at: Set(Utc::now().fixed_offset()),
    }
    .insert(&s.dbc)
    .await?;
    Ok(if htmx(&headers) {
        Html(EventCardTemplate::new(&model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{}", model.id)).into_response()
    })
}
async fn delete_event(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    event::Entity::delete_by_id(id).exec(&s.dbc).await?;
    Ok(if htmx(&headers) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Redirect::to("/").into_response()
    })
}
async fn create_participant(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Form(f): Form<ParticipantForm>,
) -> Result<Response, AppError> {
    find_event(&s.dbc, id).await?;
    let model = participant::ActiveModel {
        id: Set(Uuid::new_v4()),
        event_id: Set(id),
        name: Set(required(&f.name, "氏名")?.into()),
        email: Set(required(&f.email, "メールアドレス")?.into()),
        attendance: Set("pending".into()),
        created_at: Set(Utc::now().fixed_offset()),
    }
    .insert(&s.dbc)
    .await?;
    Ok(if htmx(&headers) {
        Html(ParticipantRowTemplate::new(id, &model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{id}")).into_response()
    })
}
async fn update_attendance(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
    Form(f): Form<AttendanceForm>,
) -> Result<Response, AppError> {
    let value = Attendance::parse(&f.attendance)
        .ok_or_else(|| AppError::BadRequest("不正な出欠状態です。".into()))?;
    let model = participant::Entity::find_by_id(participant_id)
        .filter(participant::Column::EventId.eq(event_id))
        .one(&s.dbc)
        .await?
        .ok_or(AppError::NotFound)?;
    let mut active: participant::ActiveModel = model.into();
    active.attendance = Set(value.value().into());
    let model = active.update(&s.dbc).await?;
    Ok(if htmx(&headers) {
        Html(ParticipantRowTemplate::new(event_id, &model).render()?).into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}
async fn delete_participant(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path((event_id, participant_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, AppError> {
    participant::Entity::delete_many()
        .filter(participant::Column::Id.eq(participant_id))
        .filter(participant::Column::EventId.eq(event_id))
        .exec(&s.dbc)
        .await?;
    Ok(if htmx(&headers) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        Redirect::to(&format!("/events/{event_id}")).into_response()
    })
}
async fn find_event(db: &DatabaseConnection, id: Uuid) -> Result<event::Model, AppError> {
    event::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)
}
fn required<'a>(v: &'a str, name: &str) -> Result<&'a str, AppError> {
    let v = v.trim();
    (!v.is_empty())
        .then_some(v)
        .ok_or_else(|| AppError::BadRequest(format!("{name}を入力してください。")))
}
fn optional(v: &str) -> Option<String> {
    (!v.trim().is_empty()).then(|| v.trim().into())
}
fn parse_datetime(v: &str) -> Result<DateTime<FixedOffset>, AppError> {
    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M")
        .map(|d| d.and_utc().fixed_offset())
        .map_err(|_| AppError::BadRequest("開催日時を入力してください。".into()))
}
fn htmx(headers: &HeaderMap) -> bool {
    headers.get("HX-Request").and_then(|h| h.to_str().ok()) == Some("true")
}

fn date(value: DateTime<FixedOffset>) -> String {
    value
        .with_timezone(&chrono::Local)
        .format("%Y年%-m月%-d日 %-H:%M")
        .to_string()
}
enum AppError {
    NotFound,
    BadRequest(String),
    Database(DbErr),
    Template(askama::Error),
}
impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        Self::Database(e)
    }
}
impl From<askama::Error> for AppError {
    fn from(e: askama::Error) -> Self {
        Self::Template(e)
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "対象のイベントまたは参加者が見つかりません。".into(),
            ),
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            Self::Database(e) => {
                tracing::error!(%e, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "データベース処理に失敗しました。".into(),
                )
            }
            Self::Template(e) => {
                tracing::error!(%e, "template rendering error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ページの生成に失敗しました。".into(),
                )
            }
        };
        let body = ErrorTemplate { message }.render().unwrap_or_else(|e| {
            tracing::error!(%e, "error template rendering error");
            "<div class=\"alert alert-error\"><span>ページの生成に失敗しました。</span></div>"
                .to_owned()
        });
        (status, Html(body)).into_response()
    }
}

#[derive(Template)]
#[template(path = "event_index.html")]
struct EventIndexTemplate {
    page_title: &'static str,
    events: Vec<EventView>,
}

impl EventIndexTemplate {
    fn new(events: Vec<event::Model>) -> Self {
        Self {
            page_title: "イベント一覧",
            events: events.iter().map(EventView::from).collect(),
        }
    }
}

#[derive(Template)]
#[template(path = "event_detail.html")]
struct EventDetailTemplate {
    page_title: String,
    event_id: Uuid,
    event: EventView,
    people: Vec<ParticipantView>,
    attendance_options: [AttendanceOption; 3],
}

impl EventDetailTemplate {
    fn new(event: &event::Model, people: Vec<participant::Model>) -> Self {
        Self {
            page_title: event.title.clone(),
            event_id: event.id,
            event: EventView::from(event),
            people: people.iter().map(ParticipantView::from).collect(),
            attendance_options: attendance_options(),
        }
    }
}

#[derive(Template)]
#[template(path = "event_card.html")]
struct EventCardTemplate {
    event: EventView,
}

impl EventCardTemplate {
    fn new(event: &event::Model) -> Self {
        Self {
            event: EventView::from(event),
        }
    }
}

#[derive(Template)]
#[template(path = "participant_row.html")]
struct ParticipantRowTemplate {
    event_id: Uuid,
    person: ParticipantView,
    attendance_options: [AttendanceOption; 3],
}

impl ParticipantRowTemplate {
    fn new(event_id: Uuid, person: &participant::Model) -> Self {
        Self {
            event_id,
            person: ParticipantView::from(person),
            attendance_options: attendance_options(),
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

struct EventView {
    id: Uuid,
    title: String,
    description: String,
    has_description: bool,
    starts_at: String,
    location: String,
}

impl From<&event::Model> for EventView {
    fn from(event: &event::Model) -> Self {
        let description = event.description.clone().unwrap_or_default();
        Self {
            id: event.id,
            title: event.title.clone(),
            has_description: !description.is_empty(),
            description,
            starts_at: date(event.starts_at),
            location: event
                .location
                .clone()
                .unwrap_or_else(|| "会場未定".to_owned()),
        }
    }
}

struct ParticipantView {
    id: Uuid,
    name: String,
    email: String,
    status: AttendanceView,
}

impl From<&participant::Model> for ParticipantView {
    fn from(person: &participant::Model) -> Self {
        let status = Attendance::parse(&person.attendance).unwrap_or(Attendance::Pending);
        Self {
            id: person.id,
            name: person.name.clone(),
            email: person.email.clone(),
            status: AttendanceView::from(status),
        }
    }
}

struct AttendanceView {
    value: &'static str,
    label: &'static str,
    class: &'static str,
}

impl From<Attendance> for AttendanceView {
    fn from(status: Attendance) -> Self {
        Self {
            value: status.value(),
            label: status.label(),
            class: status.class(),
        }
    }
}

struct AttendanceOption {
    value: &'static str,
    label: &'static str,
}

fn attendance_options() -> [AttendanceOption; 3] {
    [
        AttendanceOption {
            value: Attendance::Pending.value(),
            label: Attendance::Pending.label(),
        },
        AttendanceOption {
            value: Attendance::Attending.value(),
            label: Attendance::Attending.label(),
        },
        AttendanceOption {
            value: Attendance::Declined.value(),
            label: Attendance::Declined.label(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_index_renders_empty_state() {
        let html = EventIndexTemplate::new(Vec::new()).render().unwrap();

        assert!(html.contains("まだイベントがありません。右のフォームから作成できます。"));
    }

    #[test]
    fn event_card_escapes_user_input() {
        let html = EventCardTemplate {
            event: EventView {
                id: Uuid::nil(),
                title: "<script>alert(1)</script>".to_owned(),
                description: "説明".to_owned(),
                has_description: true,
                starts_at: "2026年9月7日 12:00".to_owned(),
                location: "会場".to_owned(),
            },
        }
        .render()
        .unwrap();

        assert!(html.contains("&#60;script&#62;alert(1)&#60;/script&#62;"));
        assert!(!html.contains("<script>alert(1)</script>"));
    }

    #[test]
    fn participant_row_selects_current_attendance() {
        let html = ParticipantRowTemplate {
            event_id: Uuid::nil(),
            person: ParticipantView {
                id: Uuid::nil(),
                name: "参加者".to_owned(),
                email: "person@example.com".to_owned(),
                status: AttendanceView::from(Attendance::Attending),
            },
            attendance_options: attendance_options(),
        }
        .render()
        .unwrap();

        assert!(html.contains("value=\"attending\" selected"));
    }
}
