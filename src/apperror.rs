use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    NotFound,
    BadRequest,
    UnprocessableEntity,
    InternalServerError,
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND.into_response(),
            AppError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            AppError::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
