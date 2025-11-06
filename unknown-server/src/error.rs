use crate::user::Backend;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_login::Error;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("idk")]
    Code(StatusCode),

    #[error(transparent)]
    AxumLogin(#[from] axum_login::Error<Backend>),

    #[error(transparent)]
    JinjaError(#[from] minijinja::Error),
}

#[derive(Serialize)]
struct AppErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            AppError::SqlxError(err) => {
                tracing::error!(%err, "sqlx error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_string(),
                )
            }
            AppError::Code(code) => (
                code,
                code.canonical_reason()
                    .unwrap_or("UNKNOWN CODE")
                    .to_string(),
            ),
            AppError::AxumLogin(err) => {
                tracing::error!(%err, "Axum login error");
                match err {
                    // Session errors are always my issue.
                    Error::Session(err2) => {
                        tracing::error!(%err2, "session error");
                        (
                            StatusCode::UNAUTHORIZED,
                            StatusCode::UNAUTHORIZED
                                .canonical_reason()
                                .unwrap()
                                .to_string(),
                        )
                    }
                    // This can be any case related to `Backend`
                    Error::Backend(err2) => {
                        tracing::error!(%err2, "session error");
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Auth is dead".to_string(),
                        )
                    }
                }
            }
            AppError::JinjaError(err) => {
                error!(%err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_string(),
                )
            }
        };

        (code, ErrorJson(AppErrorResponse { message })).into_response()
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct ErrorJson<T>(T);

impl<T> IntoResponse for ErrorJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl From<StatusCode> for AppError {
    fn from(value: StatusCode) -> Self {
        Self::Code(value)
    }
}
