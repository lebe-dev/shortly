use log::{error, info};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::url::ports::{UrlRepository, UrlService};

const DEFAULT_TTL: u32 = 604_800; // 7 days in seconds
const MAX_RETRIES: usize = 100;

#[derive(Debug, Clone)]
pub struct UrlServiceImpl<R>
where
    R: UrlRepository,
{
    base_url: String,
    repo: R,
}

impl<R> UrlServiceImpl<R>
where
    R: UrlRepository,
{
    pub fn new(base_url: &str, repo: R) -> Self {
        Self {
            base_url: base_url.to_string(),
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
                        ttl: DEFAULT_TTL,
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
