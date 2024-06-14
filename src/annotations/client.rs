use std::fmt;

use crate::{
    annotations::model::{Annotation, AnnotationRequest},
    error::Result,
    http,
};
use tracing::instrument;

use super::ListRequest;

/// Provides methods to work with Axiom annotations.
#[derive(Debug, Clone)]
pub struct Client<'client> {
    http_client: &'client http::Client,
}

impl<'client> Client<'client> {
    pub(crate) fn new(http_client: &'client http::Client) -> Self {
        Self { http_client }
    }

    /// Creates an annotation
    ///
    /// # Errors
    /// If the API call fails
    #[instrument(skip(self))]
    pub async fn create(&self, req: AnnotationRequest) -> Result<Annotation> {
        self.http_client
            .post("/v2/annotations", req)
            .await?
            .json()
            .await
    }

    /// Gets an annotation
    ///
    /// # Errors
    /// If the API call fails
    #[instrument(skip(self))]
    pub async fn get(&self, id: impl fmt::Display + fmt::Debug) -> Result<Annotation> {
        self.http_client
            .get(format!("/v2/annotations/{id}"))
            .await?
            .json()
            .await
    }

    /// Lists annotations
    ///
    /// # Errors
    /// If the API call fails
    #[instrument(skip(self))]
    pub async fn list(&self, req: ListRequest) -> Result<Vec<Annotation>> {
        let query_params = serde_qs::to_string(&req)?;
        self.http_client
            .get(format!("/v2/annotations?{query_params}"))
            .await?
            .json()
            .await
    }

    /// Updates an annotation
    ///
    /// # Errors
    /// If the API call fails
    #[instrument(skip(self))]
    pub async fn update(
        &self,
        id: impl fmt::Display + fmt::Debug,
        req: AnnotationRequest,
    ) -> Result<Annotation> {
        self.http_client
            .put(format!("/v2/annotations/{id}"), req)
            .await?
            .json()
            .await
    }
    /// Delets an annotation
    ///
    /// # Errors
    /// If the API call fails
    #[instrument(skip(self))]
    pub async fn delete(&self, id: impl fmt::Display + fmt::Debug) -> Result<()> {
        self.http_client
            .delete(format!("/v2/annotations/{id}"))
            .await
    }
}
