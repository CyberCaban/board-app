use rocket::serde::json::Json;
use serde_json::{json, Value};

#[derive(serde::Serialize)]
pub enum ApiErrorType {
    Unauthorized,
    InvalidToken,
    InvalidFileType,
    UserAlreadyExists,
    UserNotFound,
    YouDoNotOwnThisFile
}

impl ToString for ApiErrorType {
    fn to_string(&self) -> String {
        match self {
            ApiErrorType::Unauthorized => "Unauthorized".to_string(),
            ApiErrorType::InvalidToken => "Invalid Token".to_string(),
            ApiErrorType::InvalidFileType => "Invalid File Type".to_string(),
            ApiErrorType::UserAlreadyExists => "User Already Exists".to_string(),
            ApiErrorType::UserNotFound => "User Not Found".to_string(),
            ApiErrorType::YouDoNotOwnThisFile => "You do not own this file".to_string(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ApiError {
    error_type: String,
    error_msg: String,
}
impl ApiError {
    pub fn unknown_error() -> Self {
        eprintln!("Unknown error");
        ApiError {
            error_type: "Unknown Error".to_string(),
            error_msg: "Unknown Error".to_string(),
        }
    }
    pub fn from_message(message: String) -> Self {
        eprintln!("Error message: {}", message);
        ApiError {
            error_type: "Internal Server Error".to_string(),
            error_msg: message,
        }
    }
    pub fn from_error(error: &impl std::error::Error) -> Self {
        eprintln!("Error: {}", error);
        ApiError {
            error_type: "Internal Server Error".to_string(),
            error_msg: error.to_string(),
        }
    }
    pub fn new(error_type: &str, error_msg: impl ToString) -> Self {
        eprintln!("Error: {} {}", error_type, error_msg.to_string());
        ApiError {
            error_type: error_type.to_string(),
            error_msg: error_msg.to_string(),
        }
    }
    pub fn from_type(error_type: ApiErrorType) -> Self {
        ApiError {
            error_type: error_type.to_string(),
            error_msg: error_type.to_string(),
        }
    }
    pub fn to_json(&self) -> Json<Value> {
        Json(json!(self))
    }
}

#[derive(serde::Serialize)]
enum ApiResponse {
    Ok,
    Err,
}

#[derive(serde::Serialize)]
pub enum RegisterError {
    EmptyUsername,
    UserAlreadyExists,
    WeakPassword,
}
impl ToString for RegisterError {
    fn to_string(&self) -> String {
        match self {
            RegisterError::UserAlreadyExists => "UserAlreadyExists".to_string(),
            RegisterError::WeakPassword => "WeakPassword".to_string(),
            RegisterError::EmptyUsername => "EmptyUsername".to_string(),
        }
    }
}
pub enum LoginError {
    UserNotFound,
    WrongPassword,
}
impl ToString for LoginError {
    fn to_string(&self) -> String {
        match self {
            LoginError::UserNotFound => "UserNotFound".to_string(),
            LoginError::WrongPassword => "WrongPassword".to_string(),
        }
    }
}
