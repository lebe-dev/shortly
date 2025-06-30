use axum::http::StatusCode;
use axum::response::IntoResponse;
use server_lib::VERSION;

pub async fn get_version_route() -> impl IntoResponse {
    (StatusCode::OK, VERSION).into_response()
}
