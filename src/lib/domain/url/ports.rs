use std::future::Future;

use crate::domain::url::model::FindUrlError;

use super::model::{DeleteUrlError, ShortUrlGenerationError, Url};

pub trait UrlService: Clone + Send + Sync + 'static {
    fn is_url_valid(&self, url: &str) -> impl Future<Output = bool> + Send;

    fn register_url(
        &self,
        url: &str,
        user_id: Option<i64>,
    ) -> impl Future<Output = Result<Url, ShortUrlGenerationError>> + Send;

    fn generate_short_url(&self, url: &Url) -> impl Future<Output = String> + Send;

    fn find_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Url>, FindUrlError>> + Send;

    fn cleanup_expired_urls(&self) -> impl Future<Output = Result<(), FindUrlError>> + Send;

    fn list_user_urls(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<Url>, FindUrlError>> + Send;

    fn delete_url(
        &self,
        url_id: &str,
        user_id: i64,
    ) -> impl Future<Output = Result<(), DeleteUrlError>> + Send;
}

pub trait UrlRepository: Send + Sync + Clone + 'static {
    fn save(&self, url: &Url) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
    fn find_by_id(&self, id: &str)
    -> impl Future<Output = Result<Option<Url>, sqlx::Error>> + Send;
    fn delete_expired(&self) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
    fn find_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<Url>, sqlx::Error>> + Send;
    fn delete_by_id(&self, id: &str) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}
