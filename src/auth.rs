use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response, Json},
};
use serde_json::json;

pub struct ApiKey;

impl<S: Sync> FromRequestParts<S> for ApiKey {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Response> {
        let expected = match std::env::var("API_KEY") {
            Ok(value) if !value.is_empty() => value,
            _ => return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "API key is not configured." })),
            ).into_response()),
        };

        match parts.headers.get("x-api-key").and_then(|v| v.to_str().ok()) {
            Some(key) if key == expected => Ok(ApiKey),
            _ => Err((
                StatusCode::FORBIDDEN,
                Json(json!({ "error": "Forbidden. Valid API key required." })),
            ).into_response()),
        }
    }
}
