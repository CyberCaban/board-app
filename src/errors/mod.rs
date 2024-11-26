use rocket::serde::json::Json;
use serde_json::{json, Value};
use diesel::result::Error as DieselError;

#[derive(serde::Serialize)]
pub enum ApiErrorType {
    Unauthorized,
    InvalidToken,
    InvalidFileType,
    UserAlreadyExists,
    UserNotFound,
    YouDoNotOwnThisFile,
    FailedToParseUUID,
    NotFound,
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
            ApiErrorType::FailedToParseUUID => "Failed to parse UUID".to_string(),
            ApiErrorType::NotFound => "Not Found".to_string(),
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
    pub fn from_error<E: std::error::Error + 'static>(error: E) -> Self {
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

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => ApiError::from_type(ApiErrorType::NotFound),
            _ => ApiError::from_message(error.to_string()),
        }
    }
}

impl From<ApiError> for DieselError {
    fn from(error: ApiError) -> Self {
        DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown, 
            Box::new(error.error_msg)
        )
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
    EmptyPassword,
}
impl ToString for RegisterError {
    fn to_string(&self) -> String {
        match self {
            RegisterError::UserAlreadyExists => "UserAlreadyExists".to_string(),
            RegisterError::WeakPassword => "WeakPassword".to_string(),
            RegisterError::EmptyUsername => "EmptyUsername".to_string(),
            RegisterError::EmptyPassword => "EmptyPassword".to_string(),
        }
    }
}
pub enum LoginError {
    UserNotFound,
    WrongPassword,
    EmptyPassword,
    EmptyUsername,
}
impl ToString for LoginError {
    fn to_string(&self) -> String {
        match self {
            LoginError::UserNotFound => "UserNotFound".to_string(),
            LoginError::WrongPassword => "WrongPassword".to_string(),
            LoginError::EmptyPassword => "EmptyPassword".to_string(),
            LoginError::EmptyUsername => "EmptyUsername".to_string(),
        }
    }
}
