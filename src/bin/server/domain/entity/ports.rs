use std::future::Future;

use super::model::{Entity, EntityError};

pub trait EntityService: Clone + Send + Sync + 'static {
    /// Harvests the HTML content from the given URL using a browser.
    ///
    /// Saves markdown files to `{data}/{url-domain}/{page-slug}.md`.
    /// Return paths to the markdown files.
    fn get_entity(
        &self,
        url: &str,
        exclude_urls_by_mask: Option<&str>,
    ) -> impl Future<Output = Result<Vec<String>, EntityError>> + Send;
}

/// Persist state for markdown files
pub trait EntityRepository: Send + Sync + Clone + 'static {
    fn find_all(&self) -> impl Future<Output = Result<Vec<Entity>, sqlx::Error>> + Send;
}
