use axum::extract::Request;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::info;
use serde_json::json;

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let path = request.uri().path();

    if path == "/api/version"
        || path == "/api/login"
        || path == "/api/auth/session"
        || !path.starts_with("/api/")
    {
        return next.run(request).await;
    }

    if let Some(auth_header) = request.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") && auth_str.len() > 7 {
                return next.run(request).await;
            }
        }
    }

    if let Some(cookie_header) = request.headers().get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("session_token=") && cookie.len() > 14 {
                    return next.run(request).await;
                }
            }
        }
    }

    info!("Unauthorized access attempt to: {}", path);
    (
        StatusCode::UNAUTHORIZED,
        [("content-type", HeaderValue::from_static("application/json"))],
        json!({
            "error": "Authentication required",
            "message": "Please provide a valid session token in Authorization header or cookie"
        })
        .to_string(),
    )
        .into_response()
}

/// Extract session token from request headers (Authorization header or cookies)
pub fn extract_session_token(headers: &HeaderMap) -> Option<String> {
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(token) = cookie.strip_prefix("session_token=") {
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}
