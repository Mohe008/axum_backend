use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use validator::ValidationErrors;

pub enum AppError {
    NotFound(String),
    DatabaseError(sea_orm::DbErr),
    ValidationError(String)
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg)
        };
        let body = Json(ErrorResponse{ error: message });

        (status, body).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}