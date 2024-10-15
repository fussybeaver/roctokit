//! Method, error and parameter types for the RateLimit endpoint.
#![allow(
    clippy::all
)]
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde::Deserialize;

use crate::adapters::{AdapterError, Client, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct RateLimit<'api, C: Client<Req = crate::adapters::Req>> {
    client: &'api C
}

pub fn new<C: Client<Req = crate::adapters::Req>>(client: &C) -> RateLimit<C> {
    RateLimit { client }
}

/// Errors for the [Get rate limit status for the authenticated user](RateLimit::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum RateLimitGetError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Not modified")]
    Status304,
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api, C: Client<Req = crate::adapters::Req>> RateLimit<'api, C> {
    /// ---
    ///
    /// # Get rate limit status for the authenticated user
    ///
    /// > [!NOTE]
    /// > Accessing this endpoint does not count against your REST API rate limit.
    /// 
    /// Some categories of endpoints have custom rate limits that are separate from the rate limit governing the other REST API endpoints. For this reason, the API response categorizes your rate limit. Under `resources`, you'll see objects relating to different categories:
    /// * The `core` object provides your rate limit status for all non-search-related resources in the REST API.
    /// * The `search` object provides your rate limit status for the REST API for searching (excluding code searches). For more information, see "[Search](https://docs.github.com/rest/search/search)."
    /// * The `code_search` object provides your rate limit status for the REST API for searching code. For more information, see "[Search code](https://docs.github.com/rest/search/search#search-code)."
    /// * The `graphql` object provides your rate limit status for the GraphQL API. For more information, see "[Resource limitations](https://docs.github.com/graphql/overview/resource-limitations#rate-limit)."
    /// * The `integration_manifest` object provides your rate limit status for the `POST /app-manifests/{code}/conversions` operation. For more information, see "[Creating a GitHub App from a manifest](https://docs.github.com/apps/creating-github-apps/setting-up-a-github-app/creating-a-github-app-from-a-manifest#3-you-exchange-the-temporary-code-to-retrieve-the-app-configuration)."
    /// * The `dependency_snapshots` object provides your rate limit status for submitting snapshots to the dependency graph. For more information, see "[Dependency graph](https://docs.github.com/rest/dependency-graph)."
    /// * The `code_scanning_upload` object provides your rate limit status for uploading SARIF results to code scanning. For more information, see "[Uploading a SARIF file to GitHub](https://docs.github.com/code-security/code-scanning/integrating-with-code-scanning/uploading-a-sarif-file-to-github)."
    /// * The `actions_runner_registration` object provides your rate limit status for registering self-hosted runners in GitHub Actions. For more information, see "[Self-hosted runners](https://docs.github.com/rest/actions/self-hosted-runners)."
    /// * The `source_import` object is no longer in use for any API endpoints, and it will be removed in the next API version. For more information about API versions, see "[API Versions](https://docs.github.com/rest/about-the-rest-api/api-versions)."
    /// 
    /// > [!NOTE]
    /// > The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    ///
    /// [GitHub API docs for get](https://docs.github.com/rest/rate-limit/rate-limit#get-rate-limit-status-for-the-authenticated-user)
    ///
    /// ---
    pub async fn get_async(&self) -> Result<RateLimitOverview, RateLimitGetError> {

        let request_uri = format!("{}/rate_limit", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.client)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                304 => Err(RateLimitGetError::Status304),
                404 => Err(RateLimitGetError::Status404(github_response.to_json_async().await?)),
                code => Err(RateLimitGetError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get rate limit status for the authenticated user
    ///
    /// > [!NOTE]
    /// > Accessing this endpoint does not count against your REST API rate limit.
    /// 
    /// Some categories of endpoints have custom rate limits that are separate from the rate limit governing the other REST API endpoints. For this reason, the API response categorizes your rate limit. Under `resources`, you'll see objects relating to different categories:
    /// * The `core` object provides your rate limit status for all non-search-related resources in the REST API.
    /// * The `search` object provides your rate limit status for the REST API for searching (excluding code searches). For more information, see "[Search](https://docs.github.com/rest/search/search)."
    /// * The `code_search` object provides your rate limit status for the REST API for searching code. For more information, see "[Search code](https://docs.github.com/rest/search/search#search-code)."
    /// * The `graphql` object provides your rate limit status for the GraphQL API. For more information, see "[Resource limitations](https://docs.github.com/graphql/overview/resource-limitations#rate-limit)."
    /// * The `integration_manifest` object provides your rate limit status for the `POST /app-manifests/{code}/conversions` operation. For more information, see "[Creating a GitHub App from a manifest](https://docs.github.com/apps/creating-github-apps/setting-up-a-github-app/creating-a-github-app-from-a-manifest#3-you-exchange-the-temporary-code-to-retrieve-the-app-configuration)."
    /// * The `dependency_snapshots` object provides your rate limit status for submitting snapshots to the dependency graph. For more information, see "[Dependency graph](https://docs.github.com/rest/dependency-graph)."
    /// * The `code_scanning_upload` object provides your rate limit status for uploading SARIF results to code scanning. For more information, see "[Uploading a SARIF file to GitHub](https://docs.github.com/code-security/code-scanning/integrating-with-code-scanning/uploading-a-sarif-file-to-github)."
    /// * The `actions_runner_registration` object provides your rate limit status for registering self-hosted runners in GitHub Actions. For more information, see "[Self-hosted runners](https://docs.github.com/rest/actions/self-hosted-runners)."
    /// * The `source_import` object is no longer in use for any API endpoints, and it will be removed in the next API version. For more information about API versions, see "[API Versions](https://docs.github.com/rest/about-the-rest-api/api-versions)."
    /// 
    /// > [!NOTE]
    /// > The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    ///
    /// [GitHub API docs for get](https://docs.github.com/rest/rate-limit/rate-limit#get-rate-limit-status-for-the-authenticated-user)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self) -> Result<RateLimitOverview, RateLimitGetError> {

        let request_uri = format!("{}/rate_limit", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.client)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(RateLimitGetError::Status304),
                404 => Err(RateLimitGetError::Status404(github_response.to_json()?)),
                code => Err(RateLimitGetError::Generic { code }),
            }
        }
    }

}
