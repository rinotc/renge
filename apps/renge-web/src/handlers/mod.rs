pub(crate) mod event_handlers;
pub(crate) mod participant_handlers;

use axum::http::HeaderMap;

pub(crate) fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|header| header.to_str().ok())
        == Some("true")
}
