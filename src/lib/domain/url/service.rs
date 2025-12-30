use log::{error, info};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::url::{
    audit::{AuditEventType, AuditEventWithUser, UrlAuditEvent},
    model::{CleanupExpiredUrlsError, FindUrlError},
    ports::{UrlRepository, UrlService},
};

const MAX_RETRIES: usize = 100;

/// Base reserved names that are always protected regardless of configuration
pub const BASE_RESERVED_NAMES: &[&str] = &[
    "links", "api", "login", "logout", "assets", "static", "health", "metrics", "auth", "admin",
];

#[derive(Debug, Clone)]
pub struct UrlServiceImpl<R>
where
    R: UrlRepository,
{
    base_url: String,
    /// TTL in seconds
    ttl: u32,
    /// Maximum URL length in characters
    max_url_length: usize,
    repo: R,
    // Named URLs configuration
    named_urls_enabled: bool,
    named_url_min_length: usize,
    named_url_max_length: usize,
    reserved_names: Vec<String>,
    // Rate limiting configuration
    max_urls_per_user: u32,
    max_urls_per_day: u32,
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
    /// * `max_url_length` - Maximum URL length in characters
    /// * `repo` - URL repository implementation
    /// * `named_urls_enabled` - Whether named URLs feature is enabled
    /// * `named_url_min_length` - Minimum length for custom names
    /// * `named_url_max_length` - Maximum length for custom names
    /// * `reserved_names` - Additional reserved names from configuration (will be merged with BASE_RESERVED_NAMES)
    /// * `max_urls_per_user` - Maximum total URLs per user
    /// * `max_urls_per_day` - Maximum URLs per user per day
    pub fn new(
        base_url: &str,
        ttl_hours: u32,
        max_url_length: usize,
        repo: R,
        named_urls_enabled: bool,
        named_url_min_length: usize,
        named_url_max_length: usize,
        reserved_names: Vec<String>,
        max_urls_per_user: u32,
        max_urls_per_day: u32,
    ) -> Self {
        // Merge base reserved names with config reserved names
        let mut merged_reserved_names: Vec<String> =
            BASE_RESERVED_NAMES.iter().map(|s| s.to_string()).collect();

        // Add config reserved names, avoiding duplicates
        for name in reserved_names {
            let name_lower = name.to_lowercase();
            if !merged_reserved_names
                .iter()
                .any(|r| r.to_lowercase() == name_lower)
            {
                merged_reserved_names.push(name);
            }
        }

        Self {
            base_url: base_url.to_string(),
            ttl: ttl_hours * 3600, // Convert hours to seconds
            max_url_length,
            repo,
            named_urls_enabled,
            named_url_min_length,
            named_url_max_length,
            reserved_names: merged_reserved_names,
            max_urls_per_user,
            max_urls_per_day,
        }
    }

    /// Validate custom name format and restrictions
    fn validate_custom_name(
        &self,
        name: &str,
    ) -> Result<(), super::model::ShortUrlGenerationError> {
        if !self.named_urls_enabled {
            return Err(super::model::ShortUrlGenerationError::InvalidCustomName(
                "Named URLs feature is disabled".to_string(),
            ));
        }

        if name.len() < self.named_url_min_length {
            return Err(super::model::ShortUrlGenerationError::InvalidCustomName(
                format!(
                    "Name must be at least {} characters",
                    self.named_url_min_length
                ),
            ));
        }

        if name.len() > self.named_url_max_length {
            return Err(super::model::ShortUrlGenerationError::InvalidCustomName(
                format!(
                    "Name must be at most {} characters",
                    self.named_url_max_length
                ),
            ));
        }

        let is_valid_chars = name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

        if !is_valid_chars {
            return Err(super::model::ShortUrlGenerationError::InvalidCustomName(
                "Name can only contain letters, numbers, hyphens, and underscores".to_string(),
            ));
        }

        let name_lower = name.to_lowercase();
        if self
            .reserved_names
            .iter()
            .any(|r| r.to_lowercase() == name_lower)
        {
            return Err(super::model::ShortUrlGenerationError::CustomNameReserved);
        }

        Ok(())
    }

    /// Check rate limits for URL creation (applies to all URLs for authenticated users)
    async fn check_rate_limits(
        &self,
        user_id: i64,
    ) -> Result<(), super::model::ShortUrlGenerationError> {
        // Total count
        let total_count = self.repo.count_by_user_id(user_id).await?;
        if total_count >= self.max_urls_per_user as i64 {
            return Err(super::model::ShortUrlGenerationError::UserLimitExceeded);
        }

        // Daily limit
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                super::model::ShortUrlGenerationError::from(anyhow::anyhow!("Time error: {}", e))
            })?
            .as_secs() as i64;

        let day_ago = timestamp - 86400;
        let daily_count = self.repo.count_by_user_id_since(user_id, day_ago).await?;

        if daily_count >= self.max_urls_per_day as i64 {
            return Err(super::model::ShortUrlGenerationError::RateLimitExceeded);
        }

        Ok(())
    }
}

impl<R> UrlService for UrlServiceImpl<R>
where
    R: UrlRepository,
{
    async fn is_url_valid(&self, url: &str) -> bool {
        // Trim whitespace before validation
        let url = url.trim();

        if url.is_empty() {
            return false;
        }

        if url.len() > self.max_url_length {
            return false;
        }

        if !url::Url::parse(url).is_ok() {
            return false;
        }

        let normalized_url = url.to_lowercase();
        let normalized_base_url = self.base_url.to_lowercase();

        let base_url_without_slash = normalized_base_url.trim_end_matches('/');

        if normalized_url == base_url_without_slash
            || normalized_url.starts_with(&format!("{}/", base_url_without_slash))
            || normalized_url.starts_with(&format!("{}?", base_url_without_slash))
            || normalized_url.starts_with(&format!("{}#", base_url_without_slash))
        {
            return false;
        }

        true
    }

    async fn register_url(
        &self,
        url: &str,
        user_id: Option<i64>,
        custom_name: Option<String>,
    ) -> Result<super::model::Url, super::model::ShortUrlGenerationError> {
        let url = url.trim();

        info!(
            "registering url '{}' with custom_name: {:?}",
            url, custom_name
        );

        if !self.is_url_valid(url).await {
            return Err(super::model::ShortUrlGenerationError::InvalidOriginalUrl);
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                super::model::ShortUrlGenerationError::from(anyhow::anyhow!(
                    "failed to get timestamp: {}",
                    e
                ))
            })?
            .as_secs() as i64;

        if let Some(ref name) = custom_name {
            self.validate_custom_name(name)?;

            if self.repo.find_by_custom_name(name).await?.is_some() {
                return Err(super::model::ShortUrlGenerationError::CustomNameExists);
            }

            let uid = user_id.ok_or_else(|| {
                super::model::ShortUrlGenerationError::InvalidCustomName(
                    "Authentication required for named URLs".to_string(),
                )
            })?;

            self.check_rate_limits(uid).await?;

            let hash_id = loop {
                let random_number: u64 = rand::thread_rng().gen_range(916_132_832..56_800_235_584);
                let short_id = base62::encode(random_number);
                if self.repo.find_by_id(&short_id).await?.is_none() {
                    break short_id;
                }
            };

            let new_url = super::model::Url {
                id: hash_id,
                original_url: url.to_string(),
                ttl: 0, // No TTL for named URLs
                created: timestamp,
                user_id: Some(uid),
                custom_name: Some(name.clone()),
            };

            self.repo.save(&new_url).await?;

            let audit_event = UrlAuditEvent {
                id: None,
                event_type: AuditEventType::CreateUrl,
                actor_user_id: uid,
                target_user_id: uid,
                url_name: Some(name.clone()),
                created_at: timestamp,
            };
            self.repo.record_audit_event(&audit_event).await?;

            info!(
                "registered named url '{}' -> '{}' with hash '{}'",
                url, name, new_url.id
            );
            return Ok(new_url);
        }

        if let Some(uid) = user_id {
            self.check_rate_limits(uid).await?;
        }

        for attempt in 0..MAX_RETRIES {
            let random_number: u64 = rand::thread_rng().gen_range(916_132_832..56_800_235_584);
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
                    let new_url = super::model::Url {
                        id: short_id.clone(),
                        original_url: url.to_string(),
                        ttl: self.ttl,
                        created: timestamp,
                        user_id,
                        custom_name: None,
                    };

                    self.repo.save(&new_url).await?;

                    if let Some(uid) = user_id {
                        let audit_event = UrlAuditEvent {
                            id: None,
                            event_type: AuditEventType::CreateUrl,
                            actor_user_id: uid,
                            target_user_id: uid,
                            url_name: Some(short_id.clone()),
                            created_at: timestamp,
                        };
                        self.repo.record_audit_event(&audit_event).await?;
                    }

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
        info!("finding url by id or name '{}'", id);

        if let Some(url) = self.repo.find_by_id(id).await? {
            return Ok(Some(url));
        }

        if let Some(url) = self.repo.find_by_custom_name(id).await? {
            return Ok(Some(url));
        }

        info!("url wasn't found by id or name '{}'", id);
        Ok(None)
    }

    async fn check_custom_name_available(
        &self,
        name: &str,
    ) -> Result<bool, super::model::CheckCustomNameError> {
        if let Err(_) = self.validate_custom_name(name) {
            return Ok(false); // Invalid = not available
        }

        match self.repo.find_by_custom_name(name).await {
            Ok(Some(_)) => Ok(false), // Taken
            Ok(None) => Ok(true),     // Available
            Err(e) => Err(super::model::CheckCustomNameError::from(e)),
        }
    }

    async fn generate_short_url(&self, url: &super::model::Url) -> String {
        let path = url.custom_name.as_ref().unwrap_or(&url.id);
        format!("{}/{}", self.base_url, path)
    }

    async fn cleanup_expired_urls(&self) -> Result<(), CleanupExpiredUrlsError> {
        self.repo.delete_expired().await?;

        info!("all expired urls have been removed");

        Ok(())
    }

    async fn list_user_urls(
        &self,
        user_id: i64,
    ) -> Result<Vec<super::model::Url>, super::model::FindUrlError> {
        info!("listing urls for user {}", user_id);
        self.repo.find_by_user_id(user_id).await.map_err(Into::into)
    }

    async fn delete_url(
        &self,
        url_id: &str,
        user_id: i64,
        is_admin: bool,
    ) -> Result<(), super::model::DeleteUrlError> {
        info!(
            "attempting to delete url '{}' by user {} (admin: {})",
            url_id, user_id, is_admin
        );

        match self.repo.find_by_id(url_id).await? {
            Some(url) => {
                if is_admin || url.user_id == Some(user_id) {
                    let url_name_for_audit =
                        url.custom_name.clone().or_else(|| Some(url.id.clone()));

                    self.repo.delete_by_id(url_id).await?;
                    info!("url '{}' deleted successfully", url_id);

                    let timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|e| {
                            super::model::DeleteUrlError::from(anyhow::anyhow!("Time error: {}", e))
                        })?
                        .as_secs() as i64;

                    let audit_event = UrlAuditEvent {
                        id: None,
                        event_type: AuditEventType::DeleteUrl,
                        actor_user_id: user_id,
                        target_user_id: user_id,
                        url_name: url_name_for_audit,
                        created_at: timestamp,
                    };

                    if let Err(e) = self.repo.record_audit_event(&audit_event).await {
                        error!("Failed to record delete audit event: {:?}", e);
                    }

                    Ok(())
                } else {
                    error!(
                        "unauthorized delete attempt: url '{}' does not belong to user {}",
                        url_id, user_id
                    );
                    Err(super::model::DeleteUrlError::Unauthorized)
                }
            }
            None => {
                error!("url '{}' not found", url_id);
                Err(super::model::DeleteUrlError::NotFound)
            }
        }
    }

    async fn count_named_urls_by_user(&self, user_id: i64) -> Result<i64, FindUrlError> {
        self.repo
            .count_by_user_id(user_id)
            .await
            .map_err(Into::into)
    }

    async fn count_named_urls_by_user_since(
        &self,
        user_id: i64,
        since_timestamp: i64,
    ) -> Result<i64, FindUrlError> {
        self.repo
            .count_by_user_id_since(user_id, since_timestamp)
            .await
            .map_err(Into::into)
    }

    async fn list_all_urls(&self) -> Result<Vec<super::model::Url>, super::model::FindUrlError> {
        info!("listing all urls");
        self.repo.find_all().await.map_err(Into::into)
    }

    async fn record_audit_event(
        &self,
        event: &UrlAuditEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.repo
            .record_audit_event(event)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    async fn find_audit_events(
        &self,
        event_type: Option<AuditEventType>,
        actor_user_id: Option<i64>,
        target_user_id: Option<i64>,
        url_name: Option<String>,
        username: Option<String>,
        date_from: Option<i64>,
        date_to: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AuditEventWithUser>, i64), FindUrlError> {
        self.repo
            .find_audit_events(
                event_type,
                actor_user_id,
                target_user_id,
                url_name,
                username,
                date_from,
                date_to,
                limit,
                offset,
            )
            .await
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::url::model::ShortUrlGenerationError;
    use crate::tests::database::get_in_memory_db;

    // Helper function to create a test user in the database
    async fn create_test_user(db: &crate::outbound::sqlite::init::Sqlite, user_id: i64) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        sqlx::query(
            "INSERT INTO users (id, gitlab_id, username, email, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(user_id)
        .bind(user_id * 1000) // gitlab_id (just make it unique)
        .bind(format!("testuser{}", user_id))
        .bind(format!("test{}@example.com", user_id))
        .bind(current_time)
        .bind(current_time)
        .execute(db.get_pool())
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_register_url_success() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        let result = service
            .register_url("https://example.com", None, None)
            .await;

        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.original_url, "https://example.com");
        assert_eq!(url.ttl, 3600); // Should be 3600 seconds
        assert!(!url.id.is_empty());
    }

    #[tokio::test]
    async fn test_register_url_generates_unique_id() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        let url1 = service
            .register_url("https://example.com", None, None)
            .await
            .unwrap();
        let url2 = service
            .register_url("https://google.com", None, None)
            .await
            .unwrap();

        assert_ne!(url1.id, url2.id);
    }

    #[tokio::test]
    async fn test_find_by_id_existing_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        let registered_url = service
            .register_url("https://example.com", None, None)
            .await
            .unwrap();
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
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        let result = service.find_by_id("nonexistent").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_generate_short_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        let registered_url = service
            .register_url("https://example.com", None, None)
            .await
            .unwrap();
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
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

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
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

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
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

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
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(!service.is_url_valid("").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_whitespace_only() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(!service.is_url_valid("   ").await);
        assert!(!service.is_url_valid("\t").await);
        assert!(!service.is_url_valid("\n").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_missing_scheme() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(!service.is_url_valid("example.com").await);
        assert!(!service.is_url_valid("www.example.com").await);
        assert!(!service.is_url_valid("example.com/path").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_relative_paths() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(!service.is_url_valid("/path/to/resource").await);
        assert!(!service.is_url_valid("../relative/path").await);
        assert!(!service.is_url_valid("./current/path").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_invalid_format() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(!service.is_url_valid("not a url at all").await);
        assert!(!service.is_url_valid("http://").await);
        assert!(!service.is_url_valid("://missing-scheme").await);
        assert!(!service.is_url_valid("ht!tp://invalid-scheme.com").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_with_special_characters() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

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
        let service = UrlServiceImpl::new(
            "http://localhost:8080",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        assert!(service.is_url_valid("http://192.168.1.1").await);
        assert!(service.is_url_valid("http://127.0.0.1:8080/path").await);
        assert!(service.is_url_valid("http://[::1]/ipv6").await);
        assert!(service.is_url_valid("http://[2001:db8::1]:8080/").await);
    }

    #[tokio::test]
    async fn test_register_url_generates_six_character_id() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        ); // 1 hour

        // Generate multiple URLs to test consistency
        for i in 0..50 {
            let url = format!("https://example{}.com", i);
            let result = service.register_url(&url, None, None).await;

            assert!(result.is_ok());
            let registered_url = result.unwrap();
            assert_eq!(
                registered_url.id.len(),
                6,
                "ID should be exactly 6 characters: '{}'",
                registered_url.id
            );
        }
    }

    // Tests for self-referencing URL prevention

    #[tokio::test]
    async fn test_is_url_valid_rejects_self_referencing() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(!service.is_url_valid("https://short.ly").await);
        assert!(!service.is_url_valid("https://short.ly/").await);
        assert!(!service.is_url_valid("https://short.ly/abc123").await);
        assert!(!service.is_url_valid("https://short.ly?query").await);
        assert!(!service.is_url_valid("https://short.ly#fragment").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_case_insensitive_self_referencing() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(!service.is_url_valid("HTTPS://SHORT.LY/abc").await);
        assert!(!service.is_url_valid("https://Short.Ly/test").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_handles_trailing_slash_in_base_url() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(!service.is_url_valid("https://short.ly/abc").await);
        assert!(!service.is_url_valid("https://short.ly").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_allows_different_domains() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(service.is_url_valid("https://example.com").await);
        assert!(service.is_url_valid("https://short.ly.evil.com").await);
    }

    // Tests for trim functionality

    #[tokio::test]
    async fn test_is_url_valid_trims_whitespace() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(service.is_url_valid("  https://example.com  ").await);
        assert!(service.is_url_valid("\thttps://example.com\t").await);
        assert!(service.is_url_valid("\nhttps://example.com\n").await);
        assert!(service.is_url_valid("   https://example.com").await);
        assert!(service.is_url_valid("https://example.com   ").await);
    }

    #[tokio::test]
    async fn test_is_url_valid_rejects_self_referencing_with_whitespace() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "https://short.ly",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        assert!(!service.is_url_valid("  https://short.ly  ").await);
        assert!(!service.is_url_valid("  https://short.ly/abc  ").await);
        assert!(!service.is_url_valid("\thttps://short.ly\t").await);
    }

    #[tokio::test]
    async fn test_register_url_trims_whitespace() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false,
            3,
            20,
            vec![],
            100,
            10,
        );

        let result = service
            .register_url("  https://example.com  ", None, None)
            .await;

        assert!(result.is_ok());
        let url = result.unwrap();
        // The stored URL should be trimmed
        assert_eq!(url.original_url, "https://example.com");
        assert!(!url.original_url.starts_with(' '));
        assert!(!url.original_url.ends_with(' '));
    }

    // Tests for validate_custom_name

    #[tokio::test]
    async fn test_validate_custom_name_hardcoded_reserved_names() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![], // No additional reserved names
            100,
            10,
        );

        // Test all hardcoded reserved names
        for reserved_name in BASE_RESERVED_NAMES {
            let result = service.validate_custom_name(reserved_name);
            assert!(
                result.is_err(),
                "Expected '{}' to be reserved",
                reserved_name
            );
            if let Err(e) = result {
                assert!(
                    matches!(e, ShortUrlGenerationError::CustomNameReserved),
                    "Expected CustomNameReserved error for '{}'",
                    reserved_name
                );
            }
        }
    }

    #[tokio::test]
    async fn test_validate_custom_name_hardcoded_reserved_names_case_insensitive() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            100,
            10,
        );

        // Test case variations of hardcoded reserved names
        assert!(service.validate_custom_name("API").is_err());
        assert!(service.validate_custom_name("Api").is_err());
        assert!(service.validate_custom_name("aPi").is_err());
        assert!(service.validate_custom_name("LOGIN").is_err());
        assert!(service.validate_custom_name("Login").is_err());
        assert!(service.validate_custom_name("ADMIN").is_err());
        assert!(service.validate_custom_name("Admin").is_err());
    }

    #[tokio::test]
    async fn test_validate_custom_name_config_reserved_names() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec!["custom".to_string(), "reserved".to_string()],
            100,
            10,
        );

        // Test config reserved names
        assert!(service.validate_custom_name("custom").is_err());
        assert!(service.validate_custom_name("reserved").is_err());
        assert!(service.validate_custom_name("CUSTOM").is_err());
        assert!(service.validate_custom_name("Reserved").is_err());
    }

    #[tokio::test]
    async fn test_validate_custom_name_min_length() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,    // Min length
            20,
            vec![],
            100,
            10,
        );

        // Too short
        assert!(service.validate_custom_name("ab").is_err());
        assert!(service.validate_custom_name("a").is_err());

        // Exactly min length - should be OK
        assert!(service.validate_custom_name("abc").is_ok());

        // Longer than min - should be OK
        assert!(service.validate_custom_name("abcd").is_ok());
    }

    #[tokio::test]
    async fn test_validate_custom_name_max_length() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            10, // Max length
            vec![],
            100,
            10,
        );

        // Too long
        assert!(service.validate_custom_name("12345678901").is_err());
        assert!(
            service
                .validate_custom_name("verylongnamethatexceedslimit")
                .is_err()
        );

        // Exactly max length - should be OK
        assert!(service.validate_custom_name("1234567890").is_ok());

        // Shorter than max - should be OK
        assert!(service.validate_custom_name("short").is_ok());
    }

    #[tokio::test]
    async fn test_validate_custom_name_valid_characters() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            100,
            10,
        );

        // Valid characters: a-z, A-Z, 0-9, -, _
        assert!(service.validate_custom_name("abc123").is_ok());
        assert!(service.validate_custom_name("my-url").is_ok());
        assert!(service.validate_custom_name("my_url").is_ok());
        assert!(service.validate_custom_name("My-URL_123").is_ok());
    }

    #[tokio::test]
    async fn test_validate_custom_name_invalid_characters() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            100,
            10,
        );

        // Invalid characters
        assert!(service.validate_custom_name("my url").is_err()); // space
        assert!(service.validate_custom_name("my.url").is_err()); // dot
        assert!(service.validate_custom_name("my/url").is_err()); // slash
        assert!(service.validate_custom_name("my@url").is_err()); // at
        assert!(service.validate_custom_name("my#url").is_err()); // hash
        assert!(service.validate_custom_name("my!url").is_err()); // exclamation
    }

    #[tokio::test]
    async fn test_validate_custom_name_feature_disabled() {
        let db = get_in_memory_db().await;
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            false, // Named URLs DISABLED
            3,
            20,
            vec![],
            100,
            10,
        );

        // Should reject any name when feature is disabled
        let result = service.validate_custom_name("validname");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, ShortUrlGenerationError::InvalidCustomName(_)));
        }
    }

    #[tokio::test]
    async fn test_validate_custom_name_no_duplicate_in_merged_list() {
        let db = get_in_memory_db().await;
        // Config contains a name that's already in BASE_RESERVED_NAMES
        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true,
            3,
            20,
            vec!["api".to_string(), "custom".to_string()], // "api" is already in BASE_RESERVED_NAMES
            100,
            10,
        );

        // Both should be reserved
        assert!(service.validate_custom_name("api").is_err());
        assert!(service.validate_custom_name("custom").is_err());

        // Check the merged list doesn't have duplicates
        // Count how many times "api" appears in reserved_names
        let api_count = service
            .reserved_names
            .iter()
            .filter(|name| name.to_lowercase() == "api")
            .count();
        assert_eq!(api_count, 1, "Reserved names should not contain duplicates");
    }

    // Tests for rate limiting (max_urls_per_user and max_urls_per_day)

    #[tokio::test]
    async fn test_register_url_max_urls_per_user_limit() {
        let db = get_in_memory_db().await;

        let user_id = 1;
        create_test_user(&db, user_id).await;

        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            3,  // max_urls_per_user = 3
            10, // max_urls_per_day = 10
        );

        // Create 3 URLs - should all succeed
        for i in 1..=3 {
            let result = service
                .register_url(
                    &format!("https://example{}.com", i),
                    Some(user_id),
                    Some(format!("mylink{}", i)),
                )
                .await;
            if let Err(ref e) = result {
                panic!("Should be able to create URL {} of 3, error: {:?}", i, e);
            }
            assert!(result.is_ok());
        }

        // Try to create 4th URL - should fail with UserLimitExceeded
        let result = service
            .register_url(
                "https://example4.com",
                Some(user_id),
                Some("mylink4".to_string()),
            )
            .await;

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, ShortUrlGenerationError::UserLimitExceeded),
                "Expected UserLimitExceeded error, got: {:?}",
                e
            );
        }
    }

    #[tokio::test]
    async fn test_register_url_max_urls_per_day_limit() {
        let db = get_in_memory_db().await;

        let user_id = 1;
        create_test_user(&db, user_id).await;

        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            100, // max_urls_per_user = 100 (high enough to not interfere)
            2,   // max_urls_per_day = 2
        );

        // Create 2 URLs - should both succeed
        for i in 1..=2 {
            let result = service
                .register_url(
                    &format!("https://example{}.com", i),
                    Some(user_id),
                    Some(format!("daily{}", i)),
                )
                .await;
            assert!(result.is_ok(), "Should be able to create URL {} of 2", i);
        }

        // Try to create 3rd URL - should fail with RateLimitExceeded
        let result = service
            .register_url(
                "https://example3.com",
                Some(user_id),
                Some("daily3".to_string()),
            )
            .await;

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, ShortUrlGenerationError::RateLimitExceeded),
                "Expected RateLimitExceeded error, got: {:?}",
                e
            );
        }
    }

    #[tokio::test]
    async fn test_register_url_within_limits_succeeds() {
        let db = get_in_memory_db().await;

        let user_id = 1;
        create_test_user(&db, user_id).await;

        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            5, // max_urls_per_user = 5
            3, // max_urls_per_day = 3
        );

        // Create 2 URLs - both should succeed (under both limits)
        let result1 = service
            .register_url(
                "https://example1.com",
                Some(user_id),
                Some("within1".to_string()),
            )
            .await;
        assert!(result1.is_ok());

        let result2 = service
            .register_url(
                "https://example2.com",
                Some(user_id),
                Some("within2".to_string()),
            )
            .await;
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_register_url_rate_limits_only_apply_to_named_urls() {
        let db = get_in_memory_db().await;

        let user_id = 1;
        create_test_user(&db, user_id).await;

        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            2, // max_urls_per_user = 2 (very low)
            1, // max_urls_per_day = 1 (very low)
        );

        // Create one named URL
        let result = service
            .register_url(
                "https://example1.com",
                Some(user_id),
                Some("named".to_string()),
            )
            .await;
        assert!(result.is_ok());

        // Try to create another named URL - should fail due to daily limit
        let result = service
            .register_url(
                "https://example2.com",
                Some(user_id),
                Some("named2".to_string()),
            )
            .await;
        assert!(result.is_err());

        // Regular URLs should now also be subject to rate limits for authenticated users
        let result = service
            .register_url("https://example3.com", Some(user_id), None)
            .await;
        assert!(
            result.is_err(),
            "Regular URLs should also be subject to rate limits for authenticated users"
        );

        // But anonymous URLs should still work
        let result = service
            .register_url("https://example3.com", None, None)
            .await;
        assert!(
            result.is_ok(),
            "Anonymous URLs should not be subject to rate limits"
        );
    }

    #[tokio::test]
    async fn test_register_url_different_users_have_separate_limits() {
        let db = get_in_memory_db().await;

        create_test_user(&db, 1).await;
        create_test_user(&db, 2).await;

        let service = UrlServiceImpl::new(
            "http://localhost:8080/",
            1,
            2048,
            db,
            true, // Named URLs enabled
            3,
            20,
            vec![],
            2, // max_urls_per_user = 2
            2, // max_urls_per_day = 2
        );

        // User 1 creates 2 URLs (hits limit)
        for i in 1..=2 {
            let result = service
                .register_url(
                    &format!("https://user1-{}.com", i),
                    Some(1),
                    Some(format!("user1link{}", i)),
                )
                .await;
            assert!(result.is_ok());
        }

        // User 1 is now at limit
        let result = service
            .register_url(
                "https://user1-3.com",
                Some(1),
                Some("user1link3".to_string()),
            )
            .await;
        assert!(result.is_err());

        // User 2 should still be able to create URLs (separate limits)
        let result = service
            .register_url(
                "https://user2-1.com",
                Some(2),
                Some("user2link1".to_string()),
            )
            .await;
        assert!(
            result.is_ok(),
            "User 2 should have their own separate limits"
        );
    }
}
