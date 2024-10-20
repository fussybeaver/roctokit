//! Method, error and parameter types for the Gitignore endpoint.
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

pub struct Gitignore<'api, C: Client> where AdapterError: From<<C as Client>::Err> {
    client: &'api C
}

pub fn new<C: Client>(client: &C) -> Gitignore<C> where AdapterError: From<<C as Client>::Err> {
    Gitignore { client }
}

/// Errors for the [Get all gitignore templates](Gitignore::get_all_templates_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum GitignoreGetAllTemplatesError {
    #[error("Not modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<GitignoreGetAllTemplatesError> for AdapterError {
    fn from(err: GitignoreGetAllTemplatesError) -> Self {
        let (description, status_code) = match err {
            GitignoreGetAllTemplatesError::Status304 => (String::from("Not modified"), 304),
            GitignoreGetAllTemplatesError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get a gitignore template](Gitignore::get_template_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum GitignoreGetTemplateError {
    #[error("Not modified")]
    Status304,
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<GitignoreGetTemplateError> for AdapterError {
    fn from(err: GitignoreGetTemplateError) -> Self {
        let (description, status_code) = match err {
            GitignoreGetTemplateError::Status304 => (String::from("Not modified"), 304),
            GitignoreGetTemplateError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}



impl<'api, C: Client> Gitignore<'api, C> where AdapterError: From<<C as Client>::Err> {
    /// ---
    ///
    /// # Get all gitignore templates
    ///
    /// List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user).
    ///
    /// [GitHub API docs for get_all_templates](https://docs.github.com/rest/gitignore/gitignore#get-all-gitignore-templates)
    ///
    /// ---
    pub async fn get_all_templates_async(&self) -> Result<Vec<String>, AdapterError> {

        let request_uri = format!("{}/gitignore/templates", super::GITHUB_BASE_API_URL);


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
                304 => Err(GitignoreGetAllTemplatesError::Status304.into()),
                code => Err(GitignoreGetAllTemplatesError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get all gitignore templates
    ///
    /// List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user).
    ///
    /// [GitHub API docs for get_all_templates](https://docs.github.com/rest/gitignore/gitignore#get-all-gitignore-templates)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_all_templates(&self) -> Result<Vec<String>, AdapterError> {

        let request_uri = format!("{}/gitignore/templates", super::GITHUB_BASE_API_URL);


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
                304 => Err(GitignoreGetAllTemplatesError::Status304.into()),
                code => Err(GitignoreGetAllTemplatesError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a gitignore template
    ///
    /// Get the content of a gitignore template.
    /// 
    /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
    /// 
    /// - **`application/vnd.github.raw+json`**: Returns the raw .gitignore contents.
    ///
    /// [GitHub API docs for get_template](https://docs.github.com/rest/gitignore/gitignore#get-a-gitignore-template)
    ///
    /// ---
    pub async fn get_template_async(&self, name: &str) -> Result<GitignoreTemplate, AdapterError> {

        let request_uri = format!("{}/gitignore/templates/{}", super::GITHUB_BASE_API_URL, name);


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
                304 => Err(GitignoreGetTemplateError::Status304.into()),
                code => Err(GitignoreGetTemplateError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a gitignore template
    ///
    /// Get the content of a gitignore template.
    /// 
    /// This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
    /// 
    /// - **`application/vnd.github.raw+json`**: Returns the raw .gitignore contents.
    ///
    /// [GitHub API docs for get_template](https://docs.github.com/rest/gitignore/gitignore#get-a-gitignore-template)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_template(&self, name: &str) -> Result<GitignoreTemplate, AdapterError> {

        let request_uri = format!("{}/gitignore/templates/{}", super::GITHUB_BASE_API_URL, name);


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
                304 => Err(GitignoreGetTemplateError::Status304.into()),
                code => Err(GitignoreGetTemplateError::Generic { code }.into()),
            }
        }
    }

}
