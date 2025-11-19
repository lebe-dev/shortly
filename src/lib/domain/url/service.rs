use log::{error, info};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::url::{
    model::CleanupExpiredUrlsError,
    ports::{UrlRepository, UrlService},
};

const MAX_RETRIES: usize = 100;

#[derive(Debug, Clone)]
pub struct UrlServiceImpl<R>
where
    R: UrlRepository,
{
    base_url: String,
    /// TTL in seconds
    ttl: u32,
    repo: R,
}

impl<R> UrlServiceImpl<R>
where
    R: UrlRepository,
{
    /// Creates a new UrlServiceImpl
    ///
    /// # Arguments
    /// * `base_url` - Base URL for generating short URLs
    /// * `ttl_hours` - TTL in hours (will be converted to seconds)
    /// * `repo` - URL repository implementation
    pub fn new(base_url: &str, ttl_hours: u32, repo: R) -> Self {
        Self {
            base_url: base_url.to_string(),
            ttl: ttl_hours * 3600, // Convert hours to seconds
            repo,
        }
    }
}

impl<R> UrlService for UrlServiceImpl<R>
where
    R: UrlRepository,
{
    async fn is_url_valid(&self, url: &str) -> bool {
        if url.trim().is_empty() {
            return false;
        }

        url::Url::parse(url).is_ok()
    }

    async fn register_url(
        &self,
        url: &str,
    ) -> Result<super::model::Url, super::model::ShortUrlGenerationError> {
        info!("registering url '{}'..", url);

        if !self.is_url_valid(url).await {
            return Err(super::model::ShortUrlGenerationError::InvalidOriginalUrl);
        }

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
        format!("{}/{}", self.base_url, url.id)
    }

    async fn cleanup_expired_urls(&self) -> Result<(), CleanupExpiredUrlsError> {
        self.repo.delete_expired().await?;

        info!("all expired urls have been removed");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::database::get_in_memory_db;

    #[tokio::test]
    async fn test_register_url_success() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 1, db); // 1 hour

        let result = service.register_url("https://example.com").await;

        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.original_url, "https://example.com");
        assert_eq!(url.ttl, 3600); // Should be 3600 seconds
        assert!(!url.id.is_empty());
    }

    #[tokio::test]
    async fn test_register_url_generates_unique_id() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 1, db); // 1 hour

        let url1 = service.register_url("https://example.com").await.unwrap();
        let url2 = service.register_url("https://google.com").await.unwrap();

        assert_ne!(url1.id, url2.id);
    }

    #[tokio::test]
    async fn test_find_by_id_existing_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 1, db); // 1 hour

        let registered_url = service.register_url("https://example.com").await.unwrap();
        let found_url = service.find_by_id(&registered_url.id).await.unwrap();

        assert!(found_url.is_some());
        let found = found_url.unwrap();
        assert_eq!(found.id, registered_url.id);
        assert_eq!(found.original_url, "https://example.com");
        assert_eq!(found.ttl, 3600); // Should be 3600 seconds
    }

    #[tokio::test]
    async fn test_find_by_id_non_existing_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080/", 1, db); // 1 hour

        let result = service.find_by_id("nonexistent").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_generate_short_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        let registered_url = service.register_url("https://example.com").await.unwrap();
        let short_url = service.generate_short_url(&registered_url).await;

        assert_eq!(
            short_url,
            format!("http://localhost:8080/{}", registered_url.id)
        );
        assert!(short_url.starts_with("http://localhost:8080/"));
    }

    // Tests for is_url_valid

    #[tokio::test]
    async fn test_is_url_valid_with_http() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(service.is_url_valid("http://example.com").await);
        assert!(service.is_url_valid("http://example.com/path").await);
        assert!(
            service
                .is_url_valid("http://example.com:8080/path?query=value")
                .await
        );
    }

    #[tokio::test]
    async fn test_is_url_valid_with_https() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(service.is_url_valid("https://example.com").await);
        assert!(
            service
                .is_url_valid("https://www.example.com/path/to/resource")
                .await
        );
        assert!(
            service
                .is_url_valid("https://subdomain.example.com:443/")
                .await
        );
    }

    #[tokio::test]
    async fn test_is_url_valid_with_various_schemes() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        // FTP
        assert!(service.is_url_valid("ftp://ftp.example.com/file.txt").await);

        // WebSocket
        assert!(service.is_url_valid("ws://websocket.example.com").await);
        assert!(
            service
                .is_url_valid("wss://secure.websocket.example.com")
                .await
        );

        // File
        assert!(service.is_url_valid("file:///path/to/file.txt").await);

        // Mailto
        assert!(service.is_url_valid("mailto:test@example.com").await);

        // Custom schemes
        assert!(service.is_url_valid("custom://example.com/resource").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_empty_string() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(!service.is_url_valid("").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_whitespace_only() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(!service.is_url_valid("   ").await);
        assert!(!service.is_url_valid("\t").await);
        assert!(!service.is_url_valid("\n").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_missing_scheme() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(!service.is_url_valid("example.com").await);
        assert!(!service.is_url_valid("www.example.com").await);
        assert!(!service.is_url_valid("example.com/path").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_relative_paths() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(!service.is_url_valid("/path/to/resource").await);
        assert!(!service.is_url_valid("../relative/path").await);
        assert!(!service.is_url_valid("./current/path").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_invalid_format() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(!service.is_url_valid("not a url at all").await);
        assert!(!service.is_url_valid("http://").await);
        assert!(!service.is_url_valid("://missing-scheme").await);
        assert!(!service.is_url_valid("ht!tp://invalid-scheme.com").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_with_special_characters() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        // Valid URLs with encoded characters
        assert!(
            service
                .is_url_valid("https://example.com/path%20with%20spaces")
                .await
        );
        assert!(
            service
                .is_url_valid("https://example.com/search?q=hello+world")
                .await
        );
        assert!(service.is_url_valid("https://user:pass@example.com/").await);

        // Fragment identifiers
        assert!(
            service
                .is_url_valid("https://example.com/page#section")
                .await
        );
    }

    #[tokio::test]
    async fn test_is_url_valid_with_ip_addresses() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new("http://localhost:8080", 1, db); // 1 hour

        assert!(service.is_url_valid("http://192.168.1.1").await);
        assert!(service.is_url_valid("http://127.0.0.1:8080/path").await);
        assert!(service.is_url_valid("http://[::1]/ipv6").await);
        assert!(service.is_url_valid("http://[2001:db8::1]:8080/").await);
    }
}
