pub(crate) mod events;
pub(crate) mod participants;

use axum::http::HeaderMap;
use chrono::{DateTime, FixedOffset, NaiveDateTime};

use crate::error::AppError;

pub(crate) fn required<'a>(value: &'a str, name: &str) -> Result<&'a str, AppError> {
    let value = value.trim();
    (!value.is_empty())
        .then_some(value)
        .ok_or_else(|| AppError::BadRequest(format!("{name}を入力してください。")))
}

pub(crate) fn optional(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.trim().into())
}

pub(crate) fn parse_datetime(value: &str) -> Result<DateTime<FixedOffset>, AppError> {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M")
        .map(|date| date.and_utc().fixed_offset())
        .map_err(|_| AppError::BadRequest("開催日時を入力してください。".into()))
}

pub(crate) fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|header| header.to_str().ok())
        == Some("true")
}
