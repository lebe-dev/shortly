use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::http::StatusCode;
use axum::{Extension, extract::State, response::IntoResponse};
use log::error;

use crate::SharedAppState;
use crate::domain::config::model::dto::{AdminDataDto, AdminUrlDto, AppConfigDto};
use server_lib::domain::auth::model::User;
use server_lib::domain::auth::ports::UserRepository;
use server_lib::domain::url::ports::UrlService;

pub async fn get_app_config_route(
    State(state): State<Arc<SharedAppState>>,
    user: Option<Extension<User>>,
) -> impl IntoResponse {
    let mut dto: AppConfigDto = state.config.clone().into();

    if let Some(ref user_ext) = user {
        let user_id = user_ext.id;
        error!("Config requested by user_id: {}", user_id);

        match state.user_repository.find_by_id(user_id).await {
            Ok(Some(fresh_user)) => {
                dto.features.create_url.max_per_user = fresh_user.max_urls_per_user as u32;
                dto.features.create_url.max_per_day = fresh_user.max_urls_per_day as u32;
            }
            Ok(None) => {
                error!("User {} not found in database", user_id);
            }
            Err(e) => {
                error!("Failed to fetch user {} from database: {:?}", user_id, e);
            }
        }

        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i64,
            Err(e) => {
                error!("Failed to get current timestamp: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                    .into_response();
            }
        };
        let day_ago = now - 86400;

        match state.url_service.count_named_urls_by_user(user_id).await {
            Ok(total_count) => {
                error!("Total count for user {}: {}", user_id, total_count);
                dto.features.create_url.current_urls = Some(total_count as u32);
            }
            Err(e) => {
                error!(
                    "Failed to fetch total URL count for user {}: {:?}",
                    user_id, e
                );
            }
        }

        match state
            .url_service
            .count_named_urls_by_user_since(user_id, day_ago)
            .await
        {
            Ok(daily_count) => {
                error!("Daily count for user {}: {}", user_id, daily_count);
                dto.features.create_url.current_urls_today = Some(daily_count as u32);
            }
            Err(e) => {
                error!(
                    "Failed to fetch daily URL count for user {}: {:?}",
                    user_id, e
                );
            }
        }

        if is_user_admin(&user_ext.username, &state.config) {
            error!("User {} is admin, fetching all URLs", user_ext.username);

            match state.url_service.list_all_urls().await {
                Ok(urls) => {
                    let mut admin_urls: Vec<AdminUrlDto> = Vec::new();

                    for url in urls {
                        let username = if let Some(uid) = url.user_id {
                            match state.user_repository.find_by_id(uid).await {
                                Ok(Some(user)) => Some(user.username),
                                Ok(None) => {
                                    error!("User {} not found for URL {}", uid, url.id);
                                    None
                                }
                                Err(e) => {
                                    error!("Failed to fetch user {}: {:?}", uid, e);
                                    None
                                }
                            }
                        } else {
                            None
                        };

                        admin_urls.push(AdminUrlDto {
                            id: url.id,
                            original_url: url.original_url,
                            created: url.created,
                            ttl: url.ttl,
                            user_id: url.user_id,
                            username,
                            custom_name: url.custom_name,
                        });
                    }

                    match state.user_repository.find_all().await {
                        Ok(all_users) => {
                            let mut admin_users: Vec<
                                crate::domain::config::model::dto::AdminUserDto,
                            > = Vec::new();

                            for user in all_users {
                                let url_count = admin_urls
                                    .iter()
                                    .filter(|url| url.user_id == Some(user.id))
                                    .count() as u32;

                                let is_admin = is_user_admin(&user.username, &state.config);

                                admin_users.push(crate::domain::config::model::dto::AdminUserDto {
                                    id: user.id,
                                    username: user.username.clone(),
                                    email: user.email,
                                    avatar_url: user.avatar_url,
                                    created_at: user.created_at,
                                    url_count,
                                    max_urls_per_user: user.max_urls_per_user,
                                    max_urls_per_day: user.max_urls_per_day,
                                    is_admin,
                                });
                            }

                            dto.admin = Some(AdminDataDto {
                                all_urls: admin_urls,
                                users: admin_users,
                            });
                        }
                        Err(e) => {
                            error!("failed to fetch users for admin: {:?}", e);
                            dto.admin = Some(AdminDataDto {
                                all_urls: admin_urls,
                                users: vec![],
                            });
                        }
                    }
                }
                Err(e) => {
                    error!("failed to fetch all URLs for admin: {:?}", e);
                }
            }
        }
    }

    (StatusCode::OK, Json(dto)).into_response()
}

pub fn is_user_admin(
    username: &str,
    config: &crate::domain::config::model::config::AppConfig,
) -> bool {
    config
        .auth
        .admin_users
        .as_ref()
        .map(|admins| {
            admins
                .split(',')
                .map(|s| s.trim())
                .any(|admin| admin.eq_ignore_ascii_case(username))
        })
        .unwrap_or(false)
}
