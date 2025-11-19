use std::future::Future;

use crate::domain::url::model::FindUrlError;

use super::model::{ShortUrlGenerationError, Url};

pub trait UrlService: Clone + Send + Sync + 'static {
    fn is_url_valid(&self, url: &str) -> impl Future<Output = bool> + Send;

    fn register_url(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Url, ShortUrlGenerationError>> + Send;

    fn generate_short_url(&self, url: &Url) -> impl Future<Output = String> + Send;

    fn find_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Url>, FindUrlError>> + Send;

    fn cleanup_expired_urls(&self) -> impl Future<Output = Result<(), FindUrlError>> + Send;
}

pub trait UrlRepository: Send + Sync + Clone + 'static {
    fn save(&self, url: &Url) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
    fn find_by_id(&self, id: &str)
    -> impl Future<Output = Result<Option<Url>, sqlx::Error>> + Send;
    fn delete_expired(&self) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}
