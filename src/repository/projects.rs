use axum::async_trait;
use http::StatusCode;
use reqwest::{Body, Client, IntoUrl, Url};
use tracing::error;

use crate::constants::USER_ID_HEADER;

use super::PutError;

#[async_trait]
pub trait ProjectRepository {
    async fn put_doc_content(
        &self,
        user_id: i32,
        project_id: i32,
        content: impl AsRef<str> + Into<Body> + Send + Sync,
    ) -> Result<(), PutError>;
}

#[derive(Clone)]
pub struct HttpProjectRepository {
    manager_projects_url: Url,
    client: Client,
}

impl HttpProjectRepository {
    pub fn new(url: impl IntoUrl) -> Self {
        Self {
            manager_projects_url: url.into_url().unwrap(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ProjectRepository for HttpProjectRepository {
    #[tracing::instrument(skip(self, content))]
    async fn put_doc_content(
        &self,
        user_id: i32,
        project_id: i32,
        content: impl AsRef<str> + Into<Body> + Send + Sync,
    ) -> Result<(), PutError> {
        let mut url = self.manager_projects_url.clone();

        match url.path_segments_mut() {
            Err(_) => {
                error!(
                    "Bad Resource Management Projects URL! {}",
                    self.manager_projects_url
                );
                return Err(PutError::Unknown);
            }
            Ok(mut path) => path.push(project_id.to_string().as_str()),
        };

        let req = self
            .client
            .put(url)
            .header(USER_ID_HEADER.as_str(), user_id)
            .body(content)
            .send();

        let res = match req.await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(PutError::Unknown);
            }
        };

        match res.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::NOT_FOUND => Err(PutError::Missing),
            StatusCode::FORBIDDEN => Err(PutError::NoAccess),
            _ => Err(PutError::Unknown),
        }
    }
}
