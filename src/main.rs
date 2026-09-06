mod entities;

use axum::{
    Form, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use entities::{event, participant};
use maud::{DOCTYPE, Markup, html};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}
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
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL が未設定です。.env.example を参考に設定してください。");
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
        .with_state(AppState {
            db: Database::connect(url).await?,
        })
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("http://localhost:3000 で蓮華を起動しました");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index(State(s): State<AppState>) -> Result<Html<String>, AppError> {
    let events = event::Entity::find()
        .order_by_desc(event::Column::StartsAt)
        .all(&s.db)
        .await?;
    Ok(Html(layout("イベント一覧", event_index(events))))
}
async fn show_event(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, AppError> {
    let event = find_event(&s.db, id).await?;
    let people = participant::Entity::find()
        .filter(participant::Column::EventId.eq(id))
        .order_by_asc(participant::Column::CreatedAt)
        .all(&s.db)
        .await?;
    Ok(Html(layout(&event.title, event_detail(&event, people))))
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
        created_at: Set(Utc::now()),
    }
    .insert(&s.db)
    .await?;
    Ok(if htmx(&headers) {
        Html(event_card(&model).into_string()).into_response()
    } else {
        Redirect::to(&format!("/events/{}", model.id)).into_response()
    })
}
async fn delete_event(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    event::Entity::delete_by_id(id).exec(&s.db).await?;
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
    find_event(&s.db, id).await?;
    let model = participant::ActiveModel {
        id: Set(Uuid::new_v4()),
        event_id: Set(id),
        name: Set(required(&f.name, "氏名")?.into()),
        email: Set(required(&f.email, "メールアドレス")?.into()),
        attendance: Set("pending".into()),
        created_at: Set(Utc::now()),
    }
    .insert(&s.db)
    .await?;
    Ok(if htmx(&headers) {
        Html(participant_row(id, &model).into_string()).into_response()
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
        .one(&s.db)
        .await?
        .ok_or(AppError::NotFound)?;
    let mut active: participant::ActiveModel = model.into();
    active.attendance = Set(value.value().into());
    let model = active.update(&s.db).await?;
    Ok(if htmx(&headers) {
        Html(participant_row(event_id, &model).into_string()).into_response()
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
        .exec(&s.db)
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
fn parse_datetime(v: &str) -> Result<DateTime<Utc>, AppError> {
    NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M")
        .map(|d| d.and_utc())
        .map_err(|_| AppError::BadRequest("開催日時を入力してください。".into()))
}
fn htmx(headers: &HeaderMap) -> bool {
    headers.get("HX-Request").and_then(|h| h.to_str().ok()) == Some("true")
}

fn layout(title: &str, content: Markup) -> String {
    html! { (DOCTYPE) html lang="ja" data-theme="light" { head { meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1"; title { (title) " | 蓮華" } link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/daisyui@5"; script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {} script src="https://unpkg.com/htmx.org@2.0.4" {} } body class="min-h-screen bg-base-200" { header class="navbar bg-base-100 shadow-sm" { div class="mx-auto w-full max-w-6xl px-4" { a href="/" class="text-xl font-bold" { "蓮華" } span class="ml-3 text-sm text-base-content/60" { "イベント管理" } } } main class="mx-auto max-w-6xl p-4 md:p-8" { (content) } } } }.into_string()
}
fn event_index(events: Vec<event::Model>) -> Markup {
    html! { div class="mb-8" { h1 class="text-3xl font-bold" { "イベント一覧" } p class="text-base-content/60" { "準備の苦労を、綺麗なイベントとして結実させましょう。" } } div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]" { section { div id="event-list" class="grid gap-4 md:grid-cols-2" { @if events.is_empty() { div class="alert alert-info md:col-span-2" { span { "まだイベントがありません。右のフォームから作成できます。" } } } @for event in events { (event_card(&event)) } } } aside class="card h-fit bg-base-100 shadow" { div class="card-body" { h2 class="card-title" { "イベントを作成" } form hx-post="/events" hx-target="#event-list" hx-swap="afterbegin" hx-on::after-request="if(event.detail.successful) this.reset()" class="space-y-3" { label class="form-control" { div class="label" { span class="label-text" { "イベント名" } } input class="input input-bordered" name="title" required; } label class="form-control" { div class="label" { span class="label-text" { "開催日時" } } input class="input input-bordered" type="datetime-local" name="starts_at" required; } label class="form-control" { div class="label" { span class="label-text" { "会場" } } input class="input input-bordered" name="location"; } label class="form-control" { div class="label" { span class="label-text" { "説明" } } textarea class="textarea textarea-bordered" name="description" {} } button class="btn btn-primary w-full" type="submit" { "作成する" } } } } } }
}
fn event_card(event: &event::Model) -> Markup {
    html! { article id=(format!("event-{}", event.id)) class="card bg-base-100 shadow" { div class="card-body" { h2 class="card-title" { (event.title) } p class="text-sm text-base-content/70" { (date(event.starts_at)) " · " (event.location.as_deref().unwrap_or("会場未定")) } @if let Some(text) = &event.description { p class="line-clamp-2" { (text) } } div class="card-actions justify-end" { a class="btn btn-outline btn-sm" href=(format!("/events/{}", event.id)) { "詳細" } button class="btn btn-ghost btn-sm text-error" hx-delete=(format!("/events/{}", event.id)) hx-target=(format!("#event-{}", event.id)) hx-swap="outerHTML" hx-confirm="このイベントを削除しますか？" { "削除" } } } } }
}
fn event_detail(event: &event::Model, people: Vec<participant::Model>) -> Markup {
    html! { a class="link link-hover mb-5 inline-block" href="/" { "← イベント一覧へ" } div class="mb-6 flex flex-col justify-between gap-4 md:flex-row" { div { h1 class="text-3xl font-bold" { (event.title) } p class="mt-2 text-base-content/70" { (date(event.starts_at)) " · " (event.location.as_deref().unwrap_or("会場未定")) } @if let Some(text) = &event.description { p class="mt-3 whitespace-pre-wrap" { (text) } } } button class="btn btn-outline btn-error" hx-delete=(format!("/events/{}", event.id)) hx-confirm="このイベントを削除しますか？" hx-on::after-request="if(event.detail.successful) window.location='/'" { "イベントを削除" } } div class="grid gap-6 lg:grid-cols-[minmax(0,1fr)_22rem]" { section class="card bg-base-100 shadow" { div class="card-body" { h2 class="card-title" { "参加者" } div id="participant-list" class="space-y-3" { @if people.is_empty() { p class="text-base-content/60" { "参加者はまだ登録されていません。" } } @for person in people { (participant_row(event.id, &person)) } } } } aside class="card h-fit bg-base-100 shadow" { div class="card-body" { h2 class="card-title" { "参加者を追加" } form hx-post=(format!("/events/{}/participants", event.id)) hx-target="#participant-list" hx-swap="afterbegin" hx-on::after-request="if(event.detail.successful) this.reset()" class="space-y-3" { label class="form-control" { div class="label" { span class="label-text" { "氏名" } } input class="input input-bordered" name="name" required; } label class="form-control" { div class="label" { span class="label-text" { "メールアドレス" } } input class="input input-bordered" type="email" name="email" required; } button class="btn btn-primary w-full" type="submit" { "追加する" } } } } } }
}
fn participant_row(event_id: Uuid, p: &participant::Model) -> Markup {
    let status = Attendance::parse(&p.attendance).unwrap_or(Attendance::Pending);
    html! { div id=(format!("participant-{}", p.id)) class="flex flex-col gap-3 rounded-box border border-base-300 p-4 sm:flex-row sm:items-center" { div class="min-w-0 flex-1" { div class="font-semibold" { (p.name) } div class="truncate text-sm text-base-content/60" { (p.email) } } span class=(format!("badge {}", status.class())) { (status.label()) } select class="select select-bordered select-sm" name="attendance" hx-trigger="change" hx-post=(format!("/events/{event_id}/participants/{}/attendance", p.id)) hx-target=(format!("#participant-{}", p.id)) hx-swap="outerHTML" { @for option in [Attendance::Pending, Attendance::Attending, Attendance::Declined] { option value=(option.value()) selected[option == status] { (option.label()) } } } button class="btn btn-ghost btn-sm text-error" hx-delete=(format!("/events/{event_id}/participants/{}", p.id)) hx-target=(format!("#participant-{}", p.id)) hx-swap="outerHTML" hx-confirm="この参加者を削除しますか？" { "削除" } } }
}
fn date(value: DateTime<Utc>) -> String {
    value
        .with_timezone(&chrono::Local)
        .format("%Y年%-m月%-d日 %-H:%M")
        .to_string()
}
enum AppError {
    NotFound,
    BadRequest(String),
    Database(DbErr),
}
impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        Self::Database(e)
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
        };
        (
            status,
            Html(html! { div class="alert alert-error" { span { (message) } } }.into_string()),
        )
            .into_response()
    }
}
