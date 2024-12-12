use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    serde::json::json,
    Request,
};

use crate::errors::ApiError;

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub status: Status,
    pub error: Option<ApiError>,
    pub data: Option<T>,
}

impl<'r, T> Responder<'r, 'static> for ApiResponse<T>
where
    T: serde::Serialize,
{
    fn respond_to(self, req: &Request<'_>) -> response::Result<'static> {
        let json_data = match self.error {
            Some(e) => json!(*e.to_json()),
            None => json!(self.data),
        };
        response::Response::build_from(json!(json_data).respond_to(&req)?)
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

impl<T> ApiResponse<T> {
    pub fn new(data: T, status: Status) -> Self {
        ApiResponse {
            status,
            error: None,
            data: Some(data),
        }
    }
    pub fn from_data(data: T) -> Self {
        ApiResponse {
            status: Status::Ok,
            error: None,
            data: Some(data),
        }
    }
    pub fn error(error: ApiError) -> Self {
        dbg!(&error);
        ApiResponse {
            status: error.status(),
            error: Some(error),
            data: None,
        }
    }
}
