use crate::{
    handlers::{event_handlers, participant_handlers},
    state::AppState,
};
use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub(crate) fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(event_handlers::index))
        .route("/events", post(event_handlers::create_event))
        .route(
            "/events/{id}",
            get(event_handlers::show_event).delete(event_handlers::delete_event),
        )
        .route(
            "/events/{id}/participants",
            post(participant_handlers::create_participant),
        )
        .route(
            "/events/{id}/participants/{participant_id}/attendance",
            post(participant_handlers::update_attendance),
        )
        .route(
            "/events/{id}/participants/{participant_id}",
            delete(participant_handlers::delete_participant),
        )
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
