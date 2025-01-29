use rocket::{
    http::{ContentType, Status, StatusClass},
    response::{self, Responder},
    serde::json::json,
    Request,
};

use crate::errors::{ApiError, ApiErrorType};

#[derive(Debug)]
pub struct ApiResponse<T = ApiError> {
    status: Status,
    data: T,
}

impl ApiResponse {
    pub fn is_success(&self) -> bool {
        self.status.class() != StatusClass::ClientError
            || self.status.class() != StatusClass::ServerError
    }
    pub fn from_error(error: ApiError) -> Self {
        // dbg!(&error);
        ApiResponse {
            status: error.status(),
            data: error,
        }
    }
    pub fn from_error_type(error_type: ApiErrorType) -> Self {
        ApiResponse::from_error(ApiError::from_type(error_type))
    }
}

impl From<ApiError> for ApiResponse {
    fn from(error: ApiError) -> Self {
        ApiResponse {
            status: error.status(),
            data: error,
        }
    }
}

impl<T> ApiResponse<T>
where
    T: serde::Serialize,
{
    pub fn new(data: T) -> ApiResponse<T> {
        ApiResponse::<T> {
            status: Status::Ok,
            data,
        }
    }
}

impl<'r, T> Responder<'r, 'static> for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn respond_to(self, req: &Request<'_>) -> response::Result<'static> {
        let json_data = json!(self.data);
        response::Response::build_from(json_data.respond_to(&req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
