use vym_fyi_model::models::errors::AppResult;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LinkResponse {
    pub slug: String,
    pub target_url: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct LinkError {
    pub message: String,
}

#[derive(Clone)]
pub struct RedirectApp {
    crud_api_url: String,
    redirect_service_key: Option<String>,
    http_client: Client,
}

impl RedirectApp {
    pub fn short_link_repository(&self) -> () {
        ()
    }
}

pub struct RedirectAppBuilder {
    crud_api_url: String,
    redirect_service_key: Option<String>,
    max_connections: u32,
}

impl RedirectAppBuilder {
    pub fn from_env() -> AppResult<Self> {
        let crud_api_url = std::env::var("CRUD_API_URL").map_err(|_| {
            vym_fyi_model::models::errors::AppError::Config("CRUD_API_URL not set".into())
        })?;

        let redirect_service_key = std::env::var("REDIRECT_SERVICE_KEY").ok();

        Ok(Self {
            crud_api_url,
            redirect_service_key,
            max_connections: 5,
        })
    }

    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub async fn build(self) -> AppResult<RedirectApp> {
        let http_client = Client::builder()
            .pool_max_idle_per_host(self.max_connections as usize)
            .build()?;

        Ok(RedirectApp {
            crud_api_url: self.crud_api_url,
            redirect_service_key: self.redirect_service_key,
            http_client,
        })
    }
}

impl RedirectApp {
    pub async fn resolve_slug(&self, slug: &str) -> AppResult<Option<String>> {
        let url = format!("{}/api/links/{}", self.crud_api_url.trim_end_matches('/'), slug);
        
        let mut request = self.http_client.get(&url);
        
        if let Some(ref key) = self.redirect_service_key {
            request = request.header("X-Service-Key", key);
        }

        let response = request.send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(vym_fyi_model::models::errors::AppError::Server(
                format!("CRUD API returned status: {}", response.status())
            ));
        }

        let link: LinkResponse = response.json().await?;
        
        if link.active {
            Ok(Some(link.target_url))
        } else {
            Ok(None)
        }
    }
}
