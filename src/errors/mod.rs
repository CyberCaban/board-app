use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::{json, Value};

#[derive(serde::Serialize, Debug, PartialEq, Clone)]
pub enum ApiErrorType {
    Unauthorized,
    InvalidToken,
    InvalidFileType,
    UserAlreadyExists,
    UserNotFound,
    YouDoNotOwnThisFile,
    FailedToParseUUID,
    NotFound,
    FailedToDeleteFile,
    UnknownError,
    InternalServerError,
    InvalidUserId,
    InvalidRequest,
    AlreadyFriends,
    Other(String),
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
            ApiErrorType::FailedToDeleteFile => "Failed to delete file".to_string(),
            ApiErrorType::UnknownError => "Unknown Error".to_string(),
            ApiErrorType::InternalServerError => "Internal Server Error".to_string(),
            ApiErrorType::InvalidUserId => "Invalid User Id".to_string(),
            ApiErrorType::InvalidRequest => "Invalid Request".to_string(),
            ApiErrorType::AlreadyFriends => "Already Friends".to_string(),
            ApiErrorType::Other(error) => error.to_string(),
        }
    }
}

impl From<String> for ApiErrorType {
    fn from(error: String) -> Self {
        match error.as_str() {
            "Unauthorized" => ApiErrorType::Unauthorized,
            "Invalid Token" => ApiErrorType::InvalidToken,
            "Invalid File Type" => ApiErrorType::InvalidFileType,
            "User Already Exists" => ApiErrorType::UserAlreadyExists,
            "User Not Found" => ApiErrorType::UserNotFound,
            "You do not own this file" => ApiErrorType::YouDoNotOwnThisFile,
            "Failed to parse UUID" => ApiErrorType::FailedToParseUUID,
            "Not Found" => ApiErrorType::NotFound,
            "Failed to delete file" => ApiErrorType::FailedToDeleteFile,
            "Invalid User Id" => ApiErrorType::InvalidUserId,
            "Invalid Request" => ApiErrorType::InvalidRequest,
            "Already Friends" => ApiErrorType::AlreadyFriends,
            _ => ApiErrorType::Other(error),
        }
    }
}

#[derive(serde::Serialize, Debug)]
pub struct ApiError {
    error_type: ApiErrorType,
    error_msg: String,
}
impl ApiError {
    pub fn unknown_error() -> Self {
        eprintln!("Unknown error");
        ApiError {
            error_type: ApiErrorType::UnknownError,
            error_msg: "Unknown Error".to_string(),
        }
    }
    pub fn from_message(message: String) -> Self {
        eprintln!("Error message: {}", message);
        ApiError {
            error_type: ApiErrorType::UnknownError,
            error_msg: message,
        }
    }
    pub fn from_error<E: std::error::Error + 'static>(error: E) -> Self {
        eprintln!("Error: {}", error);
        ApiError {
            error_type: ApiErrorType::InternalServerError,
            error_msg: error.to_string(),
        }
    }
    pub fn new(error_type: &str, error_msg: impl ToString) -> Self {
        eprintln!("Error: {}", &error_msg.to_string());
        ApiError {
            error_type: error_type.to_string().clone().into(),
            error_msg: error_msg.to_string(),
        }
    }
    pub fn from_type(error_type: ApiErrorType) -> Self {
        eprintln!("Error: {}", &error_type.to_string());
        let error_msg = error_type.to_string().clone();
        ApiError {
            error_type,
            error_msg,
        }
    }
    pub fn to_json(&self) -> Json<Value> {
        Json(json!(self))
    }
    pub fn status(&self) -> Status {
        // dbg!(&self.error_type);
        match self.error_type {
            ApiErrorType::Unauthorized => Status::Unauthorized,
            ApiErrorType::NotFound => Status::NotFound,
            ApiErrorType::InvalidToken => Status::Unauthorized,
            ApiErrorType::InvalidFileType => Status::BadRequest,
            ApiErrorType::UserAlreadyExists => Status::Conflict,
            ApiErrorType::UserNotFound => Status::NotFound,
            ApiErrorType::YouDoNotOwnThisFile => Status::Forbidden,
            ApiErrorType::FailedToParseUUID => Status::BadRequest,
            _ => Status::InternalServerError,
        }
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
            Box::new(error.error_msg),
        )
    }
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
