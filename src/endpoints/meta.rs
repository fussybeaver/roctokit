//! Method, error and parameter types for the Meta endpoint.
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

use crate::adapters::{AdapterError, Client, GitHubRequest, GitHubResponseExt};
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Meta<'api, C: Client> where AdapterError: From<<C as Client>::Err> {
    client: &'api C
}

pub fn new<C: Client>(client: &C) -> Meta<C> where AdapterError: From<<C as Client>::Err> {
    Meta { client }
}

/// Errors for the [Get GitHub meta information](Meta::get_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetError {
    #[error("Not modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<MetaGetError> for AdapterError {
    fn from(err: MetaGetError) -> Self {
        let (description, status_code) = match err {
            MetaGetError::Status304 => (String::from("Not modified"), 304),
            MetaGetError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get all API versions](Meta::get_all_versions_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetAllVersionsError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<MetaGetAllVersionsError> for AdapterError {
    fn from(err: MetaGetAllVersionsError) -> Self {
        let (description, status_code) = match err {
            MetaGetAllVersionsError::Status404(_) => (String::from("Resource not found"), 404),
            MetaGetAllVersionsError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get Octocat](Meta::get_octocat_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetOctocatError {
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<MetaGetOctocatError> for AdapterError {
    fn from(err: MetaGetOctocatError) -> Self {
        let (description, status_code) = match err {
            MetaGetOctocatError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get the Zen of GitHub](Meta::get_zen_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaGetZenError {
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<MetaGetZenError> for AdapterError {
    fn from(err: MetaGetZenError) -> Self {
        let (description, status_code) = match err {
            MetaGetZenError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [GitHub API Root](Meta::root_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum MetaRootError {
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<MetaRootError> for AdapterError {
    fn from(err: MetaRootError) -> Self {
        let (description, status_code) = match err {
            MetaRootError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}


/// Query parameters for the [Get Octocat](Meta::get_octocat_async()) endpoint.
#[derive(Default, Serialize)]
pub struct MetaGetOctocatParams<'req> {
    /// The words to show in Octocat's speech bubble
    s: Option<&'req str>
}

impl<'req> MetaGetOctocatParams<'req> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The words to show in Octocat's speech bubble
    pub fn s(self, s: &'req str) -> Self {
        Self {
            s: Some(s),
        }
    }
}


impl<'api, C: Client> Meta<'api, C> where AdapterError: From<<C as Client>::Err> {
    /// ---
    ///
    /// # Get GitHub meta information
    ///
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://docs.github.com/articles/about-github-s-ip-addresses/)."
    /// 
    /// The API's response also includes a list of GitHub's domain names.
    /// 
    /// The values shown in the documentation's response are example values. You must always query the API directly to get the latest values.
    /// 
    /// > [!NOTE]
    /// > This endpoint returns both IPv4 and IPv6 addresses. However, not all features support IPv6. You should refer to the specific documentation for each feature to determine if IPv6 is supported.
    ///
    /// [GitHub API docs for get](https://docs.github.com/rest/meta/meta#get-apiname-meta-information)
    ///
    /// ---
    pub async fn get_async(&self) -> Result<ApiOverview, AdapterError> {

        let request_uri = format!("{}/meta", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                304 => Err(MetaGetError::Status304.into()),
                code => Err(MetaGetError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub meta information
    ///
    /// Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://docs.github.com/articles/about-github-s-ip-addresses/)."
    /// 
    /// The API's response also includes a list of GitHub's domain names.
    /// 
    /// The values shown in the documentation's response are example values. You must always query the API directly to get the latest values.
    /// 
    /// > [!NOTE]
    /// > This endpoint returns both IPv4 and IPv6 addresses. However, not all features support IPv6. You should refer to the specific documentation for each feature to determine if IPv6 is supported.
    ///
    /// [GitHub API docs for get](https://docs.github.com/rest/meta/meta#get-apiname-meta-information)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&self) -> Result<ApiOverview, AdapterError> {

        let request_uri = format!("{}/meta", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                304 => Err(MetaGetError::Status304.into()),
                code => Err(MetaGetError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get all API versions
    ///
    /// Get all supported GitHub API versions.
    ///
    /// [GitHub API docs for get_all_versions](https://docs.github.com/rest/meta/meta#get-all-api-versions)
    ///
    /// ---
    pub async fn get_all_versions_async(&self) -> Result<Vec<chrono::DateTime<chrono::Utc>>, AdapterError> {

        let request_uri = format!("{}/versions", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                404 => Err(MetaGetAllVersionsError::Status404(github_response.to_json_async().await?).into()),
                code => Err(MetaGetAllVersionsError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get all API versions
    ///
    /// Get all supported GitHub API versions.
    ///
    /// [GitHub API docs for get_all_versions](https://docs.github.com/rest/meta/meta#get-all-api-versions)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_all_versions(&self) -> Result<Vec<chrono::DateTime<chrono::Utc>>, AdapterError> {

        let request_uri = format!("{}/versions", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                404 => Err(MetaGetAllVersionsError::Status404(github_response.to_json()?).into()),
                code => Err(MetaGetAllVersionsError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get Octocat
    ///
    /// Get the octocat as ASCII art
    ///
    /// [GitHub API docs for get_octocat](https://docs.github.com/rest/meta/meta#get-octocat)
    ///
    /// ---
    pub async fn get_octocat_async(&self, query_params: Option<impl Into<MetaGetOctocatParams<'api>>>) -> Result<String, AdapterError> {

        let mut request_uri = format!("{}/octocat", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            request_uri.push_str(&serde_urlencoded::to_string(params.into())?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                code => Err(MetaGetOctocatError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get Octocat
    ///
    /// Get the octocat as ASCII art
    ///
    /// [GitHub API docs for get_octocat](https://docs.github.com/rest/meta/meta#get-octocat)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_octocat(&self, query_params: Option<impl Into<MetaGetOctocatParams<'api>>>) -> Result<String, AdapterError> {

        let mut request_uri = format!("{}/octocat", super::GITHUB_BASE_API_URL);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: MetaGetOctocatParams = params.into();
            request_uri.push_str(&serde_urlencoded::to_string(qp)?);
        }

        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                code => Err(MetaGetOctocatError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get the Zen of GitHub
    ///
    /// Get a random sentence from the Zen of GitHub
    ///
    /// [GitHub API docs for get_zen](https://docs.github.com/rest/meta/meta#get-the-zen-of-github)
    ///
    /// ---
    pub async fn get_zen_async(&self) -> Result<String, AdapterError> {

        let request_uri = format!("{}/zen", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                code => Err(MetaGetZenError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get the Zen of GitHub
    ///
    /// Get a random sentence from the Zen of GitHub
    ///
    /// [GitHub API docs for get_zen](https://docs.github.com/rest/meta/meta#get-the-zen-of-github)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_zen(&self) -> Result<String, AdapterError> {

        let request_uri = format!("{}/zen", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                code => Err(MetaGetZenError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # GitHub API Root
    ///
    /// Get Hypermedia links to resources accessible in GitHub's REST API
    ///
    /// [GitHub API docs for root](https://docs.github.com/rest/meta/meta#github-api-root)
    ///
    /// ---
    pub async fn root_async(&self) -> Result<Root, AdapterError> {

        let request_uri = format!("{}/", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json_async().await?)
        } else {
            match github_response.status_code() {
                code => Err(MetaRootError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # GitHub API Root
    ///
    /// Get Hypermedia links to resources accessible in GitHub's REST API
    ///
    /// [GitHub API docs for root](https://docs.github.com/rest/meta/meta#github-api-root)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn root(&self) -> Result<Root, AdapterError> {

        let request_uri = format!("{}/", super::GITHUB_BASE_API_URL);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(github_response.to_json()?)
        } else {
            match github_response.status_code() {
                code => Err(MetaRootError::Generic { code }.into()),
            }
        }
    }

}
