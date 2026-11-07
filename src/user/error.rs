use crate::user::UserServiceError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for UserServiceError {
    fn into_response(self) -> Response {
        let status = match self {
            UserServiceError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::PasswordHash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::EmailEncryption(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::EmailHMAC(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::UserParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::UserNotFound => StatusCode::NOT_FOUND,
            UserServiceError::InvalidData => StatusCode::BAD_REQUEST,
        };

        status.into_response()
    }
}
