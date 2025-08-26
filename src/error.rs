// src/error.rs
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use std::{error::Error, fmt};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Internal(String),
    Conflict(String),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(m) 
            | AppError::Internal(m) 
            | AppError::Conflict(m) 
            | AppError::NotFound(m) => f.write_str(m),
        }
    }
}
impl Error for AppError {}

#[derive(Serialize)]
struct ErrorBody { message: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) =>
                (StatusCode::BAD_REQUEST, Json(ErrorBody { message: msg })).into_response(),
            AppError::Internal(msg) =>
                (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorBody { message: msg })).into_response(),
            AppError::Conflict(msg) =>
                (StatusCode::CONFLICT, Json(ErrorBody { message: msg })).into_response(),
            AppError::NotFound(msg) =>
                (StatusCode::NOT_FOUND, Json(ErrorBody { message: msg })).into_response(),
        }
    }
}
