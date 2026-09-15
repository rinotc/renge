use crate::{
    handlers::{events, participants},
    state::AppState,
};
use axum::{
    Router,
    routing::{delete, get, post},
};
use tower_http::trace::TraceLayer;

pub(crate) fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(events::index))
        .route("/events", post(events::create_event))
        .route(
            "/events/{id}",
            get(events::show_event).delete(events::delete_event),
        )
        .route(
            "/events/{id}/participants",
            post(participants::create_participant),
        )
        .route(
            "/events/{id}/participants/{participant_id}/attendance",
            post(participants::update_attendance),
        )
        .route(
            "/events/{id}/participants/{participant_id}",
            delete(participants::delete_participant),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
