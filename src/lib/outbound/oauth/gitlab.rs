use reqwest::Client;
use serde::Deserialize;

use crate::domain::auth::{
    model::{AuthError, GitlabUserInfo},
    ports::OAuthClient,
};

#[derive(Clone)]
pub struct GitlabOAuthClient {
    client: Client,
    base_url: String,
    application_id: String,
    secret: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GitlabApiUser {
    id: i64,
    username: String,
    email: Option<String>,
    avatar_url: Option<String>,
}

impl GitlabOAuthClient {
    pub fn new(
        base_url: String,
        application_id: String,
        secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            client: Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("failed to initialize GitLab HTTP client"),
            base_url,
            application_id,
            secret,
            redirect_uri,
        }
    }
}

impl OAuthClient for GitlabOAuthClient {
    async fn exchange_code(&self, code: &str) -> Result<String, AuthError> {
        let token_url = format!("{}/oauth/token", self.base_url);

        let response = self
            .client
            .post(&token_url)
            .form(&[
                ("client_id", self.application_id.as_str()),
                ("client_secret", self.secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .map_err(|_| AuthError::OAuthFailed("Token request failed".into()))?;

        if !response.status().is_success() {
            return Err(AuthError::OAuthFailed(format!(
                "Token request returned HTTP {}",
                response.status().as_u16()
            )));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|_| AuthError::OAuthFailed("Invalid token response".into()))?;
        Ok(token_response.access_token)
    }

    async fn fetch_user_info(&self, access_token: &str) -> Result<GitlabUserInfo, AuthError> {
        let user_url = format!("{}/api/v4/user", self.base_url);

        let response = self
            .client
            .get(&user_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| AuthError::UserInfoFailed)?;

        if !response.status().is_success() {
            return Err(AuthError::UserInfoFailed);
        }

        let gitlab_user: GitlabApiUser = response
            .json()
            .await
            .map_err(|_| AuthError::UserInfoFailed)?;

        Ok(GitlabUserInfo {
            id: gitlab_user.id,
            username: gitlab_user.username,
            email: gitlab_user.email,
            avatar_url: gitlab_user.avatar_url,
        })
    }
}
