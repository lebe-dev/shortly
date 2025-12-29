use axum::extract::{Request, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::{debug, info};
use serde_json::json;
use server_lib::domain::auth::ports::AuthService;
use std::sync::Arc;

use crate::AppState;

/// Authentication middleware that protects API endpoints based on configuration.
///
/// # How it works
///
/// 1. **Path exemption**: Always allows certain paths without authentication:
///    - `/api/version` (public endpoint)
///    - `/api/auth/*` (authentication endpoints)
///    - Non-API paths (static assets, frontend routes)
///
/// 2. **Feature-based access control**: When `auth-only` is disabled for URL creation,
///    allows unauthenticated access to:
///    - `POST /api/url` (create short URLs)
///    - `GET /api/url/{url_id}` (retrieve URLs by ID)
///
/// 3. **Authentication bypass**: If authentication is globally disabled in config,
///    allows all requests to pass through.
///
/// 4. **Token validation**: For protected endpoints when auth is enabled:
///    - Extracts session token from Authorization header (Bearer token) or cookies
///    - Validates token using the configured auth service
///    - On success: adds authenticated user to request extensions and continues
///    - On failure: returns 401 Unauthorized response
///
/// # Request flow
///
/// The middleware executes in this order:
/// - Check path exemptions → Check feature flags → Check global auth toggle → Validate token
///
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path();
    let method = request.method();

    if path == "/api/version"
        || path == "/api/health"
        || path.starts_with("/api/auth/")
        || !path.starts_with("/api/")
    {
        debug!("path '{}' '{}' exempted from authentication", method, path);
        return next.run(request).await;
    }

    if path == "/api/config" && method == "GET" {
        debug!("Config request: attempting optional auth");

        if let Some(token) = extract_session_token(request.headers()) {
            if let Some(auth_service) = &state.auth_service {
                match auth_service.validate_session(&token).await {
                    Ok(user) => {
                        debug!("optional auth successful for user: {}", user.username);
                        request.extensions_mut().insert(user);
                        return next.run(request).await;
                    }
                    Err(e) => {
                        debug!("session validation failed: {:?}", e);
                    }
                }
            }
        }

        return next.run(request).await;
    }

    if !state.config.features.create_url.auth_only {
        if path == "/api/url" && method == "POST" {
            debug!("URL creation: auth-only=false, attempting optional auth");

            if let Some(token) = extract_session_token(request.headers()) {
                if let Some(auth_service) = &state.auth_service {
                    match auth_service.validate_session(&token).await {
                        Ok(user) => {
                            debug!("Optional auth successful for user: {}", user.username);
                            request.extensions_mut().insert(user);
                        }
                        Err(e) => {
                            debug!("optional auth failed, continuing anonymously: {:?}", e);
                        }
                    }
                }
            } else {
                debug!("no session token found, creating URL anonymously");
            }

            return next.run(request).await;
        }
        if path.starts_with("/api/url/") && method == "GET" {
            debug!("URL retrieval allowed without auth (auth-only=false)");
            return next.run(request).await;
        }
    }

    if !state.config.auth.enabled {
        debug!(
            "authentication disabled globally, allowing {} {}",
            method, path
        );
        return next.run(request).await;
    }

    let auth_service = match &state.auth_service {
        Some(service) => service,
        None => return unauthorized_response(),
    };

    let token = match extract_session_token(request.headers()) {
        Some(t) => {
            debug!("session token extracted for '{}' '{}'", method, path);
            t
        }
        None => {
            debug!("no session token found for '{}' '{}'", method, path);
            return unauthorized_response();
        }
    };

    match auth_service.validate_session(&token).await {
        Ok(user) => {
            debug!("session validated for user: {}", user.username);
            request.extensions_mut().insert(user);
            next.run(request).await
        }
        Err(e) => {
            debug!(
                "session validation failed for '{}' '{}': {:?}",
                method, path, e
            );
            unauthorized_response()
        }
    }
}

fn unauthorized_response() -> Response {
    info!("unauthorized access attempt");
    (
        StatusCode::UNAUTHORIZED,
        [("content-type", HeaderValue::from_static("application/json"))],
        json!({
            "error": "Authentication required",
            "message": "Please log in to continue"
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_bearer_token() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer abc123"));

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_cookie_token() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static("session_token=xyz789"));

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("xyz789".to_string()));
    }

    #[test]
    fn test_extract_cookie_with_multiple_cookies() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static("other=value; session_token=token123; another=val"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("token123".to_string()));
    }

    #[test]
    fn test_bearer_takes_precedence_over_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer bearer_token"),
        );
        headers.insert(
            "cookie",
            HeaderValue::from_static("session_token=cookie_token"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("bearer_token".to_string()));
    }

    #[test]
    fn test_extract_no_token() {
        let headers = HeaderMap::new();

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_malformed_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("NotBearer token123"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_cookie_without_session_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static("other=value; another=val"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_empty_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer "));

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("".to_string()));
    }

    #[test]
    fn test_extract_bearer_with_whitespace() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer   token_with_spaces_before"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("  token_with_spaces_before".to_string()));
    }

    #[test]
    fn test_extract_cookie_token_trimmed() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static(" session_token=trimmed_token "),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("trimmed_token".to_string()));
    }

    #[test]
    fn test_extract_case_sensitive_header_names() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer CaseSensitiveToken"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("CaseSensitiveToken".to_string()));
    }

    #[test]
    fn test_extract_cookie_first_match() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static("session_token=first; session_token=second"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("first".to_string()));
    }

    #[test]
    fn test_extract_from_empty_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static(""));

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_cookie_semicolon_handling() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static(";;;session_token=token_value;;;"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("token_value".to_string()));
    }

    #[test]
    fn test_extract_bearer_exact_prefix() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearertoken"));

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_cookie_exact_prefix() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "cookie",
            HeaderValue::from_static("my_session_token=should_not_match"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_multiple_header_values() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static("first=value"));
        headers.append(
            "cookie",
            HeaderValue::from_static("session_token=from_second"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_bearer_with_special_characters() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer token-with_special.chars+123"),
        );

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("token-with_special.chars+123".to_string()));
    }

    #[test]
    fn test_extract_cookie_with_equals_in_value() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static("session_token=base64=="));

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("base64==".to_string()));
    }

    #[test]
    fn test_extract_invalid_utf8_handling() {
        let mut headers = HeaderMap::new();
        let invalid_bytes = b"Bearer \xFF\xFE";
        if let Ok(header_value) = HeaderValue::from_bytes(invalid_bytes) {
            headers.insert("authorization", header_value);
            let result = extract_session_token(&headers);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn test_extract_cookie_empty_value() {
        let mut headers = HeaderMap::new();
        headers.insert("cookie", HeaderValue::from_static("session_token="));

        let result = extract_session_token(&headers);
        assert_eq!(result, Some("".to_string()));
    }
}
