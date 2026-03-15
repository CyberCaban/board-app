use chrono::Utc;
use jwt::{EncodingKey, Header};
use serde::Serialize;

use crate::{
    errors::ApiError,
    models::{api_response::ApiResponse, auth::AuthResult},
};

#[derive(Serialize)]
pub struct CentrifugoConnectClaims {
    sub: String,
    iat: usize,
    exp: usize,
}

#[derive(Serialize)]
pub struct CentrifugoConnectResponse {
    token: String,
}

#[get("/centrifugo/connect")]
pub async fn api_centrifugo_connect(
    auth: AuthResult,
) -> Result<ApiResponse<CentrifugoConnectResponse>, ApiResponse> {
    let auth = auth.unpack()?;
    let now = Utc::now().timestamp() as usize;
    let claims = CentrifugoConnectClaims {
        sub: auth.id.to_string(),
        iat: now,
        exp: now + 60 * 60,
    };

    let centrifugo_secret =
        std::env::var("CENTRIFUGO_HMAC_SECRET_KEY").unwrap_or_else(|_| "secret".to_string());

    let token = jwt::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(centrifugo_secret.as_bytes()),
    )
    .map_err(|e| ApiResponse::from_error(ApiError::from_error(e)))?;

    Ok(ApiResponse::new(CentrifugoConnectResponse { token }))
}
