use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response, Json},
};
use serde_json::json;

pub struct ApiKey;

impl<S> FromRequestParts<S> for ApiKey {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Response> {
        let expected = std::env::var("API_KEY").unwrap_or_default();
        match parts.headers.get("x-api-key").and_then(|v| v.to_str().ok()) {
            Some(key) if key == expected => Ok(ApiKey),
            _ => Err((
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "Forbidden. Valid API key required." })),
            ).into_response()),
        }
    }
}