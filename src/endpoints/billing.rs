//! Method, error and parameter types for the Billing endpoint.
#![allow(
    unused_imports,
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

use crate::adapters::{AdapterError, FromJson, GitHubRequest, GitHubRequestBuilder, GitHubResponseExt};
use crate::auth::Auth;
use crate::models::*;

use super::PerPage;

use std::collections::HashMap;
use serde_json::value::Value;

pub struct Billing<'api> {
    auth: &'api Auth
}

pub fn new(auth: &Auth) -> Billing {
    Billing { auth }
}

/// Errors for the [Get GitHub Actions billing for an organization](Billing::get_github_actions_billing_org_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetGithubActionsBillingOrgError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get GitHub Actions billing for a user](Billing::get_github_actions_billing_user_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetGithubActionsBillingUserError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get GitHub Packages billing for an organization](Billing::get_github_packages_billing_org_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetGithubPackagesBillingOrgError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get GitHub Packages billing for a user](Billing::get_github_packages_billing_user_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetGithubPackagesBillingUserError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get shared storage billing for an organization](Billing::get_shared_storage_billing_org_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetSharedStorageBillingOrgError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

/// Errors for the [Get shared storage billing for a user](Billing::get_shared_storage_billing_user_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum BillingGetSharedStorageBillingUserError {
    #[error(transparent)]
    AdapterError(#[from] AdapterError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrl(#[from] serde_urlencoded::ser::Error),


    // -- endpoint errors

    #[error("Status code: {}", code)]
    Generic { code: u16 },
}



impl<'api> Billing<'api> {
    /// ---
    ///
    /// # Get GitHub Actions billing for an organization
    ///
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    /// 
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_actions_billing_org](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-an-organization)
    ///
    /// ---
    pub async fn get_github_actions_billing_org_async(&self, org: &str) -> Result<ActionsBillingUsage, BillingGetGithubActionsBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/actions", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubActionsBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Actions billing for an organization
    ///
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    /// 
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_actions_billing_org](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_github_actions_billing_org(&self, org: &str) -> Result<ActionsBillingUsage, BillingGetGithubActionsBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/actions", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubActionsBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Actions billing for a user
    ///
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    /// 
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_actions_billing_user](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-a-user)
    ///
    /// ---
    pub async fn get_github_actions_billing_user_async(&self, username: &str) -> Result<ActionsBillingUsage, BillingGetGithubActionsBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/actions", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubActionsBillingUserError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Actions billing for a user
    ///
    /// Gets the summary of the free and paid GitHub Actions minutes used.
    /// 
    /// Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_actions_billing_user](https://docs.github.com/rest/billing/billing#get-github-actions-billing-for-a-user)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_github_actions_billing_user(&self, username: &str) -> Result<ActionsBillingUsage, BillingGetGithubActionsBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/actions", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubActionsBillingUserError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Packages billing for an organization
    ///
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_packages_billing_org](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-an-organization)
    ///
    /// ---
    pub async fn get_github_packages_billing_org_async(&self, org: &str) -> Result<PackagesBillingUsage, BillingGetGithubPackagesBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/packages", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubPackagesBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Packages billing for an organization
    ///
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_packages_billing_org](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_github_packages_billing_org(&self, org: &str) -> Result<PackagesBillingUsage, BillingGetGithubPackagesBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/packages", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubPackagesBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Packages billing for a user
    ///
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_packages_billing_user](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-a-user)
    ///
    /// ---
    pub async fn get_github_packages_billing_user_async(&self, username: &str) -> Result<PackagesBillingUsage, BillingGetGithubPackagesBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/packages", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubPackagesBillingUserError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get GitHub Packages billing for a user
    ///
    /// Gets the free and paid storage used for GitHub Packages in gigabytes.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_github_packages_billing_user](https://docs.github.com/rest/billing/billing#get-github-packages-billing-for-a-user)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_github_packages_billing_user(&self, username: &str) -> Result<PackagesBillingUsage, BillingGetGithubPackagesBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/packages", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetGithubPackagesBillingUserError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get shared storage billing for an organization
    ///
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_shared_storage_billing_org](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-an-organization)
    ///
    /// ---
    pub async fn get_shared_storage_billing_org_async(&self, org: &str) -> Result<CombinedBillingUsage, BillingGetSharedStorageBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/shared-storage", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetSharedStorageBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get shared storage billing for an organization
    ///
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_shared_storage_billing_org](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_shared_storage_billing_org(&self, org: &str) -> Result<CombinedBillingUsage, BillingGetSharedStorageBillingOrgError> {

        let request_uri = format!("{}/orgs/{}/settings/billing/shared-storage", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetSharedStorageBillingOrgError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get shared storage billing for a user
    ///
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_shared_storage_billing_user](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-a-user)
    ///
    /// ---
    pub async fn get_shared_storage_billing_user_async(&self, username: &str) -> Result<CombinedBillingUsage, BillingGetSharedStorageBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/shared-storage", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch_async(request).await?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json_async(github_response).await?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetSharedStorageBillingUserError::Generic { code }),
            }
        }
    }

    /// ---
    ///
    /// # Get shared storage billing for a user
    ///
    /// Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.
    /// 
    /// Paid minutes only apply to packages stored for private repositories. For more information, see "[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages)."
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
    /// 
    /// [GitHub API docs for get_shared_storage_billing_user](https://docs.github.com/rest/billing/billing#get-shared-storage-billing-for-a-user)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_shared_storage_billing_user(&self, username: &str) -> Result<CombinedBillingUsage, BillingGetSharedStorageBillingUserError> {

        let request_uri = format!("{}/users/{}/settings/billing/shared-storage", super::GITHUB_BASE_API_URL, username);


        let req = GitHubRequest {
            uri: request_uri,
            body: None,
            method: "GET",
            headers: vec![]
        };

        let request = GitHubRequestBuilder::build(req, self.auth)?;

        // --

        let github_response = crate::adapters::fetch(request)?;

        // --

        if github_response.is_success() {
            Ok(crate::adapters::to_json(github_response)?)
        } else {
            match github_response.status_code() {
                code => Err(BillingGetSharedStorageBillingUserError::Generic { code }),
            }
        }
    }

}
