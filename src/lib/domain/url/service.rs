use log::{error, info};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::url::ports::{UrlRepository, UrlService};

const MAX_RETRIES: usize = 100;

#[derive(Debug, Clone)]
pub struct UrlServiceImpl<R>
where
    R: UrlRepository,
{
    base_url: String,
    ttl: u32,
    repo: R,
}

impl<R> UrlServiceImpl<R>
where
    R: UrlRepository,
{
    pub fn new(base_url: &str, ttl: u32, repo: R) -> Self {
        Self {
            base_url: base_url.to_string(),
            ttl,
            repo,
        }
    }
}

impl<R> UrlService for UrlServiceImpl<R>
where
    R: UrlRepository,
{
    async fn register_url(
        &self,
        url: &str,
    ) -> Result<super::model::Url, super::model::ShortUrlGenerationError> {
        info!("registering url '{}'..", url);

        for attempt in 0..MAX_RETRIES {
            let random_number: u64 = rand::thread_rng().gen_range(100_000..1_000_000);

            let short_id = base62::encode(random_number);

            match self.repo.find_by_id(&short_id).await {
                Ok(Some(_)) => {
                    info!(
                        "collision detected for ID '{}' on attempt {}",
                        short_id,
                        attempt + 1
                    );
                    continue;
                }
                Ok(None) => {
                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| {
                            super::model::ShortUrlGenerationError::from(anyhow::anyhow!(
                                "failed to get timestamp: {}",
                                e
                            ))
                        })?
                        .as_secs() as i64;

                    let new_url = super::model::Url {
                        id: short_id.clone(),
                        original_url: url.to_string(),
                        ttl: self.ttl,
                        created: timestamp,
                    };

                    self.repo
                        .save(&new_url)
                        .await
                        .map_err(|e| super::model::ShortUrlGenerationError::from(e))?;

                    info!("registered url '{}' with id '{}'", url, short_id);
                    return Ok(new_url);
                }
                Err(e) => {
                    error!("failed to register url '{}': {}", url, e);
                    return Err(super::model::ShortUrlGenerationError::from(e));
                }
            }
        }

        Err(super::model::ShortUrlGenerationError::from(
            anyhow::anyhow!(
                "failed to generate unique short url after {} attempt(s)",
                MAX_RETRIES
            ),
        ))
    }

    async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<super::model::Url>, super::model::FindUrlError> {
        info!("finding url by id '{id}'");

        if let Some(url) = &self.repo.find_by_id(id).await? {
            Ok(Some(url.clone()))
        } else {
            info!("url wasn't found by id '{id}'");
            Ok(None)
        }
    }

    async fn generate_short_url(&self, url: &super::model::Url) -> String {
        format!("{}{}", self.base_url, url.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::database::get_in_memory_db;

    #[tokio::test]
    async fn test_register_url_success() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 3600, db);

        let result = service.register_url("https://example.com").await;

        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.original_url, "https://example.com");
        assert_eq!(url.ttl, 3600);
        assert!(!url.id.is_empty());
    }

    #[tokio::test]
    async fn test_register_url_generates_unique_id() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 3600, db);

        let url1 = service.register_url("https://example.com").await.unwrap();
        let url2 = service.register_url("https://google.com").await.unwrap();

        assert_ne!(url1.id, url2.id);
    }

    #[tokio::test]
    async fn test_find_by_id_existing_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 3600, db);

        let registered_url = service.register_url("https://example.com").await.unwrap();
        let found_url = service.find_by_id(&registered_url.id).await.unwrap();

        assert!(found_url.is_some());
        let found = found_url.unwrap();
        assert_eq!(found.id, registered_url.id);
        assert_eq!(found.original_url, "https://example.com");
        assert_eq!(found.ttl, 3600);
    }

    #[tokio::test]
    async fn test_find_by_id_non_existing_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 3600, db);

        let result = service.find_by_id("nonexistent").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_generate_short_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 3600, db);

        let registered_url = service.register_url("https://example.com").await.unwrap();
        let short_url = service.generate_short_url(&registered_url).await;

        assert_eq!(short_url, format!("http://localhost:8080/{}", registered_url.id));
        assert!(short_url.starts_with("http://localhost:8080/"));
    }
}
