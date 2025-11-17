use std::sync::Arc;

use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use log::error;
use server_lib::domain::url::ports::UrlService;

use crate::SharedAppState;
use crate::route::url::generate::RegisterUrlResponse;

pub async fn get_short_url_by_id_route(
    State(state): State<Arc<SharedAppState>>,
    Path(url_id): Path<String>,
) -> impl IntoResponse {
    match state.url_service.find_by_id(&url_id).await {
        Ok(url) => {
            if let Some(url) = url {
                (
                    StatusCode::OK,
                    Json(RegisterUrlResponse {
                        url: url.original_url,
                    }),
                )
                    .into_response()
            } else {
                error!("url wasn't found by id '{}'", url_id);
                (StatusCode::BAD_REQUEST).into_response()
            }
        }
        Err(e) => {
            error!("unable to get url by id '{}': {:?}", url_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
