use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Authorization error: {0}")]
    ForbiddenError(String),
    
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        
        let error_response = ErrorResponse {
            status: status.to_string(),
            message: self.to_string(),
        };
        
        HttpResponse::build(status).json(error_response)
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::AuthError(_) => StatusCode::UNAUTHORIZED,
            ApiError::ForbiddenError(_) => StatusCode::FORBIDDEN,
            ApiError::NotFoundError(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequestError(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}