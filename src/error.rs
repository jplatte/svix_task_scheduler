use axum::{http::StatusCode, response::IntoResponse};
use tracing::error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Database error: {0}")]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        // Emit error in From instead of IntoResponse such that tracing spans
        // that the `?` is used in apply to this tracing event.
        let e = Self::Database(value);
        error!("{e}");
        e
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
