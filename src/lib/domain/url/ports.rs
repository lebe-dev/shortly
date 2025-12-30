use std::future::Future;

use crate::domain::url::model::FindUrlError;

use super::audit::{AuditEventType, AuditEventWithUser, UrlAuditEvent};
use super::model::{CheckCustomNameError, DeleteUrlError, ShortUrlGenerationError, Url};

pub trait UrlService: Clone + Send + Sync + 'static {
    fn is_url_valid(&self, url: &str) -> impl Future<Output = bool> + Send;

    fn register_url(
        &self,
        url: &str,
        user_id: Option<i64>,
        custom_name: Option<String>,
    ) -> impl Future<Output = Result<Url, ShortUrlGenerationError>> + Send;

    fn generate_short_url(&self, url: &Url) -> impl Future<Output = String> + Send;

    fn find_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Url>, FindUrlError>> + Send;

    fn check_custom_name_available(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<bool, CheckCustomNameError>> + Send;

    fn cleanup_expired_urls(&self) -> impl Future<Output = Result<(), FindUrlError>> + Send;

    fn list_user_urls(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<Url>, FindUrlError>> + Send;

    fn delete_url(
        &self,
        url_id: &str,
        user_id: i64,
        is_admin: bool,
    ) -> impl Future<Output = Result<(), DeleteUrlError>> + Send;

    fn count_named_urls_by_user(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<i64, FindUrlError>> + Send;

    fn count_named_urls_by_user_since(
        &self,
        user_id: i64,
        since_timestamp: i64,
    ) -> impl Future<Output = Result<i64, FindUrlError>> + Send;

    fn list_all_urls(&self) -> impl Future<Output = Result<Vec<Url>, FindUrlError>> + Send;

    fn record_audit_event(
        &self,
        event: &UrlAuditEvent,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;

    fn find_audit_events(
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
    ) -> impl Future<Output = Result<(Vec<AuditEventWithUser>, i64), FindUrlError>> + Send;
}

pub trait UrlRepository: Send + Sync + Clone + 'static {
    fn save(&self, url: &Url) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn find_by_id(&self, id: &str)
    -> impl Future<Output = Result<Option<Url>, sqlx::Error>> + Send;

    fn find_by_custom_name(
        &self,
        custom_name: &str,
    ) -> impl Future<Output = Result<Option<Url>, sqlx::Error>> + Send;

    fn delete_expired(&self) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn find_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<Vec<Url>, sqlx::Error>> + Send;

    fn delete_by_id(&self, id: &str) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn count_by_user_id(
        &self,
        user_id: i64,
    ) -> impl Future<Output = Result<i64, sqlx::Error>> + Send;

    fn count_by_user_id_since(
        &self,
        user_id: i64,
        since_timestamp: i64,
    ) -> impl Future<Output = Result<i64, sqlx::Error>> + Send;

    fn record_audit_event(
        &self,
        event: &UrlAuditEvent,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn find_all(&self) -> impl Future<Output = Result<Vec<Url>, sqlx::Error>> + Send;

    fn find_audit_events(
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
    ) -> impl Future<Output = Result<(Vec<AuditEventWithUser>, i64), sqlx::Error>> + Send;
}
