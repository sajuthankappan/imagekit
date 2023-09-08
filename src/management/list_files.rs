use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::StatusCode;

use crate::{client::FILES_ENDPOINT, upload::types::Response, ErrorResponse, ImageKit};

/// Options for list/search files.
///
/// Refer: https://docs.imagekit.io/api-reference/media-api/list-and-search-files
#[derive(Default)]
pub struct Options {
    search_query: Option<String>,
    path: Option<String>,
    tags: Option<String>,
    skip: Option<u32>,
    limit: Option<u32>,
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_query<T: ToString>(mut self, val: T) -> Self {
        self.search_query = Some(val.to_string());
        self
    }

    pub fn path<T: ToString>(mut self, val: T) -> Self {
        self.path = Some(val.to_string());
        self
    }

    pub fn tags<T: ToString>(mut self, val: T) -> Self {
        self.tags = Some(val.to_string());
        self
    }

    pub fn skip(mut self, val: u32) -> Self {
        self.skip = Some(val);
        self
    }

    pub fn limit(mut self, val: u32) -> Self {
        self.limit = Some(val);
        self
    }
}

#[async_trait]
pub trait ListFiles {
    /// list and search files
    async fn list_files(&self, opts: Options) -> Result<Vec<Response>>;
}

#[async_trait]
impl ListFiles for ImageKit {
    async fn list_files(&self, opts: Options) -> Result<Vec<Response>> {
        let mut query_params = Vec::new();
        if let Some(search_query) = opts.search_query {
            query_params.push(("searchQuery", search_query))
        }
        if let Some(path) = opts.path {
            query_params.push(("path", path))
        }
        if let Some(tags) = opts.tags {
            query_params.push(("tags", tags))
        }
        if let Some(skip) = opts.skip {
            query_params.push(("skip", skip.to_string()))
        }
        if let Some(limit) = opts.limit {
            query_params.push(("limit", limit.to_string()))
        };

        let response = self
            .client
            .get(FILES_ENDPOINT.to_string())
            .query(&query_params)
            .send()
            .await?;

        if matches!(response.status(), StatusCode::OK) {
            let result = response.json::<Vec<Response>>().await?;
            return Ok(result);
        }

        let result = response.json::<ErrorResponse>().await?;

        bail!(result.message);
    }
}