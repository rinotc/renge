use crate::views::ErrorTemplate;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sea_orm::DbErr;

pub(crate) enum AppError {
    NotFound,
    BadRequest(String),
    Database(DbErr),
    Template(askama::Error),
}

impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        Self::Database(error)
    }
}

impl From<askama::Error> for AppError {
    fn from(error: askama::Error) -> Self {
        Self::Template(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "対象のイベントまたは参加者が見つかりません。".into(),
            ),
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Database(error) => {
                tracing::error!(%error, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "データベース処理に失敗しました。".into(),
                )
            }
            Self::Template(error) => {
                tracing::error!(%error, "template rendering error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ページの生成に失敗しました。".into(),
                )
            }
        };

        let body = ErrorTemplate { message }.render().unwrap_or_else(|error| {
            tracing::error!(%error, "error template rendering error");
            "<div class=\"alert alert-error\"><span>ページの生成に失敗しました。</span></div>"
                .to_owned()
        });

        (status, Html(body)).into_response()
    }
}
