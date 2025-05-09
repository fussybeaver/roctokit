//! Method, error and parameter types for the PrivateRegistries endpoint.
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

use roctokit::adapters::{AdapterError, Client, GitHubRequest, GitHubResponseExt};
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct PrivateRegistries<'api, C: Client> where AdapterError: From<<C as Client>::Err> {
    client: &'api C
}

pub fn new<C: Client>(client: &C) -> PrivateRegistries<C> where AdapterError: From<<C as Client>::Err> {
    PrivateRegistries { client }
}

/// Errors for the [Create a private registry for an organization](PrivateRegistries::create_org_private_registry_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesCreateOrgPrivateRegistryError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Validation failed, or the endpoint has been spammed.")]
    Status422(ValidationError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesCreateOrgPrivateRegistryError> for AdapterError {
    fn from(err: PrivateRegistriesCreateOrgPrivateRegistryError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesCreateOrgPrivateRegistryError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesCreateOrgPrivateRegistryError::Status422(_) => (String::from("Validation failed, or the endpoint has been spammed."), 422),
            PrivateRegistriesCreateOrgPrivateRegistryError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Delete a private registry for an organization](PrivateRegistries::delete_org_private_registry_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesDeleteOrgPrivateRegistryError {
    #[error("Bad Request")]
    Status400(BasicError),
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesDeleteOrgPrivateRegistryError> for AdapterError {
    fn from(err: PrivateRegistriesDeleteOrgPrivateRegistryError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesDeleteOrgPrivateRegistryError::Status400(_) => (String::from("Bad Request"), 400),
            PrivateRegistriesDeleteOrgPrivateRegistryError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesDeleteOrgPrivateRegistryError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get a private registry for an organization](PrivateRegistries::get_org_private_registry_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesGetOrgPrivateRegistryError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesGetOrgPrivateRegistryError> for AdapterError {
    fn from(err: PrivateRegistriesGetOrgPrivateRegistryError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesGetOrgPrivateRegistryError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesGetOrgPrivateRegistryError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get private registries public key for an organization](PrivateRegistries::get_org_public_key_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesGetOrgPublicKeyError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesGetOrgPublicKeyError> for AdapterError {
    fn from(err: PrivateRegistriesGetOrgPublicKeyError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesGetOrgPublicKeyError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesGetOrgPublicKeyError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [List private registries for an organization](PrivateRegistries::list_org_private_registries_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesListOrgPrivateRegistriesError {
    #[error("Bad Request")]
    Status400(BasicError),
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesListOrgPrivateRegistriesError> for AdapterError {
    fn from(err: PrivateRegistriesListOrgPrivateRegistriesError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesListOrgPrivateRegistriesError::Status400(_) => (String::from("Bad Request"), 400),
            PrivateRegistriesListOrgPrivateRegistriesError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesListOrgPrivateRegistriesError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Update a private registry for an organization](PrivateRegistries::update_org_private_registry_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum PrivateRegistriesUpdateOrgPrivateRegistryError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Validation failed, or the endpoint has been spammed.")]
    Status422(ValidationError),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<PrivateRegistriesUpdateOrgPrivateRegistryError> for AdapterError {
    fn from(err: PrivateRegistriesUpdateOrgPrivateRegistryError) -> Self {
        let (description, status_code) = match err {
            PrivateRegistriesUpdateOrgPrivateRegistryError::Status404(_) => (String::from("Resource not found"), 404),
            PrivateRegistriesUpdateOrgPrivateRegistryError::Status422(_) => (String::from("Validation failed, or the endpoint has been spammed."), 422),
            PrivateRegistriesUpdateOrgPrivateRegistryError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}


/// Query parameters for the [List private registries for an organization](PrivateRegistries::list_org_private_registries_async()) endpoint.
#[derive(Default, Serialize)]
pub struct PrivateRegistriesListOrgPrivateRegistriesParams {
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    per_page: Option<u16>, 
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    page: Option<u16>
}

impl PrivateRegistriesListOrgPrivateRegistriesParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn per_page(self, per_page: u16) -> Self {
        Self {
            per_page: Some(per_page),
            page: self.page, 
        }
    }

    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn page(self, page: u16) -> Self {
        Self {
            per_page: self.per_page, 
            page: Some(page),
        }
    }
}

impl<'enc> From<&'enc PerPage> for PrivateRegistriesListOrgPrivateRegistriesParams {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}

impl<'api, C: Client> PrivateRegistries<'api, C> where AdapterError: From<<C as Client>::Err> {
    /// ---
    ///
    /// # Create a private registry for an organization
    ///
    /// 
    /// Creates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for create_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#create-a-private-registry-for-an-organization)
    ///
    /// ---
    pub async fn create_org_private_registry_async(&self, org: &str, body: PostPrivateRegistriesCreateOrgPrivateRegistry) -> Result<OrgPrivateRegistryConfigurationWithSelectedRepositories, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PostPrivateRegistriesCreateOrgPrivateRegistry>(body)?),
            method: "POST",
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
                404 => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Status404(github_response.to_json_async().await?).into()),
                422 => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Status422(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Create a private registry for an organization
    ///
    /// 
    /// Creates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for create_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#create-a-private-registry-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_org_private_registry(&self, org: &str, body: PostPrivateRegistriesCreateOrgPrivateRegistry) -> Result<OrgPrivateRegistryConfigurationWithSelectedRepositories, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PostPrivateRegistriesCreateOrgPrivateRegistry>(body)?),
            method: "POST",
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
                404 => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Status404(github_response.to_json()?).into()),
                422 => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Status422(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesCreateOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Delete a private registry for an organization
    ///
    /// 
    /// Delete a private registry configuration at the organization-level.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for delete_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#delete-a-private-registry-for-an-organization)
    ///
    /// ---
    pub async fn delete_org_private_registry_async(&self, org: &str, secret_name: &str) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


        let req = GitHubRequest {
            uri: request_uri,
            body: None::<C::Body>,
            method: "DELETE",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(())
        } else {
            match github_response.status_code() {
                400 => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Status400(github_response.to_json_async().await?).into()),
                404 => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Status404(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Delete a private registry for an organization
    ///
    /// 
    /// Delete a private registry configuration at the organization-level.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for delete_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#delete-a-private-registry-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_org_private_registry(&self, org: &str, secret_name: &str) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "DELETE",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(())
        } else {
            match github_response.status_code() {
                400 => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Status400(github_response.to_json()?).into()),
                404 => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Status404(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesDeleteOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a private registry for an organization
    ///
    /// 
    /// Get the configuration of a single private registry defined for an organization, omitting its encrypted value.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#get-a-private-registry-for-an-organization)
    ///
    /// ---
    pub async fn get_org_private_registry_async(&self, org: &str, secret_name: &str) -> Result<OrgPrivateRegistryConfiguration, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


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
                404 => Err(PrivateRegistriesGetOrgPrivateRegistryError::Status404(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesGetOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a private registry for an organization
    ///
    /// 
    /// Get the configuration of a single private registry defined for an organization, omitting its encrypted value.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#get-a-private-registry-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_org_private_registry(&self, org: &str, secret_name: &str) -> Result<OrgPrivateRegistryConfiguration, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


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
                404 => Err(PrivateRegistriesGetOrgPrivateRegistryError::Status404(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesGetOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get private registries public key for an organization
    ///
    /// 
    /// Gets the org public key, which is needed to encrypt private registry secrets. You need to encrypt a secret before you can create or update secrets.
    /// 
    /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_org_public_key](https://docs.github.com/rest/private-registries/organization-configurations#get-private-registries-public-key-for-an-organization)
    ///
    /// ---
    pub async fn get_org_public_key_async(&self, org: &str) -> Result<GetPrivateRegistriesGetOrgPublicKeyResponse200, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/public-key", super::GITHUB_BASE_API_URL, org);


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
                404 => Err(PrivateRegistriesGetOrgPublicKeyError::Status404(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesGetOrgPublicKeyError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get private registries public key for an organization
    ///
    /// 
    /// Gets the org public key, which is needed to encrypt private registry secrets. You need to encrypt a secret before you can create or update secrets.
    /// 
    /// OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_org_public_key](https://docs.github.com/rest/private-registries/organization-configurations#get-private-registries-public-key-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_org_public_key(&self, org: &str) -> Result<GetPrivateRegistriesGetOrgPublicKeyResponse200, AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/public-key", super::GITHUB_BASE_API_URL, org);


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
                404 => Err(PrivateRegistriesGetOrgPublicKeyError::Status404(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesGetOrgPublicKeyError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # List private registries for an organization
    ///
    /// 
    /// Lists all private registry configurations available at the organization-level without revealing their encrypted
    /// values.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for list_org_private_registries](https://docs.github.com/rest/private-registries/organization-configurations#list-private-registries-for-an-organization)
    ///
    /// ---
    pub async fn list_org_private_registries_async(&self, org: &str, query_params: Option<impl Into<PrivateRegistriesListOrgPrivateRegistriesParams>>) -> Result<GetPrivateRegistriesListOrgPrivateRegistriesResponse200, AdapterError> {

        let mut request_uri = format!("{}/orgs/{}/private-registries", super::GITHUB_BASE_API_URL, org);

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
                400 => Err(PrivateRegistriesListOrgPrivateRegistriesError::Status400(github_response.to_json_async().await?).into()),
                404 => Err(PrivateRegistriesListOrgPrivateRegistriesError::Status404(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesListOrgPrivateRegistriesError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # List private registries for an organization
    ///
    /// 
    /// Lists all private registry configurations available at the organization-level without revealing their encrypted
    /// values.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for list_org_private_registries](https://docs.github.com/rest/private-registries/organization-configurations#list-private-registries-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_org_private_registries(&self, org: &str, query_params: Option<impl Into<PrivateRegistriesListOrgPrivateRegistriesParams>>) -> Result<GetPrivateRegistriesListOrgPrivateRegistriesResponse200, AdapterError> {

        let mut request_uri = format!("{}/orgs/{}/private-registries", super::GITHUB_BASE_API_URL, org);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: PrivateRegistriesListOrgPrivateRegistriesParams = params.into();
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
                400 => Err(PrivateRegistriesListOrgPrivateRegistriesError::Status400(github_response.to_json()?).into()),
                404 => Err(PrivateRegistriesListOrgPrivateRegistriesError::Status404(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesListOrgPrivateRegistriesError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Update a private registry for an organization
    ///
    /// 
    /// Updates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#update-a-private-registry-for-an-organization)
    ///
    /// ---
    pub async fn update_org_private_registry_async(&self, org: &str, secret_name: &str, body: PatchPrivateRegistriesUpdateOrgPrivateRegistry) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PatchPrivateRegistriesUpdateOrgPrivateRegistry>(body)?),
            method: "PATCH",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(())
        } else {
            match github_response.status_code() {
                404 => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Status404(github_response.to_json_async().await?).into()),
                422 => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Status422(github_response.to_json_async().await?).into()),
                code => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Update a private registry for an organization
    ///
    /// 
    /// Updates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_org_private_registry](https://docs.github.com/rest/private-registries/organization-configurations#update-a-private-registry-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_org_private_registry(&self, org: &str, secret_name: &str, body: PatchPrivateRegistriesUpdateOrgPrivateRegistry) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/private-registries/{}", super::GITHUB_BASE_API_URL, org, secret_name);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PatchPrivateRegistriesUpdateOrgPrivateRegistry>(body)?),
            method: "PATCH",
            headers: vec![]
        };

        let request = self.client.build(req)?;

        // --

        let github_response = self.client.fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(())
        } else {
            match github_response.status_code() {
                404 => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Status404(github_response.to_json()?).into()),
                422 => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Status422(github_response.to_json()?).into()),
                code => Err(PrivateRegistriesUpdateOrgPrivateRegistryError::Generic { code }.into()),
            }
        }
    }

}
