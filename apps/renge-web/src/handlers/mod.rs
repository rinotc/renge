pub(crate) mod event_handlers;
pub(crate) mod participant_handlers;

use axum::http::HeaderMap;

use crate::error::AppError;

pub(crate) fn required<'a>(value: &'a str, name: &str) -> Result<&'a str, AppError> {
    let value = value.trim();
    (!value.is_empty())
        .then_some(value)
        .ok_or_else(|| AppError::BadRequest(format!("{name}を入力してください。")))
}

pub(crate) fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|header| header.to_str().ok())
        == Some("true")
}
