//! Method, error and parameter types for the Campaigns endpoint.
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

pub struct Campaigns<'api, C: Client> where AdapterError: From<<C as Client>::Err> {
    client: &'api C
}

pub fn new<C: Client>(client: &C) -> Campaigns<C> where AdapterError: From<<C as Client>::Err> {
    Campaigns { client }
}

/// Errors for the [Create a campaign for an organization](Campaigns::create_campaign_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CampaignsCreateCampaignError {
    #[error("Bad Request")]
    Status400(BasicError),
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Unprocessable Entity")]
    Status422(BasicError),
    #[error("Too Many Requests")]
    Status429,
    #[error("Service unavailable")]
    Status503(GetBillingGetGithubBillingUsageReportUserResponse503),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<CampaignsCreateCampaignError> for AdapterError {
    fn from(err: CampaignsCreateCampaignError) -> Self {
        let (description, status_code) = match err {
            CampaignsCreateCampaignError::Status400(_) => (String::from("Bad Request"), 400),
            CampaignsCreateCampaignError::Status404(_) => (String::from("Resource not found"), 404),
            CampaignsCreateCampaignError::Status422(_) => (String::from("Unprocessable Entity"), 422),
            CampaignsCreateCampaignError::Status429 => (String::from("Too Many Requests"), 429),
            CampaignsCreateCampaignError::Status503(_) => (String::from("Service unavailable"), 503),
            CampaignsCreateCampaignError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Delete a campaign for an organization](Campaigns::delete_campaign_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CampaignsDeleteCampaignError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Service unavailable")]
    Status503(GetBillingGetGithubBillingUsageReportUserResponse503),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<CampaignsDeleteCampaignError> for AdapterError {
    fn from(err: CampaignsDeleteCampaignError) -> Self {
        let (description, status_code) = match err {
            CampaignsDeleteCampaignError::Status404(_) => (String::from("Resource not found"), 404),
            CampaignsDeleteCampaignError::Status503(_) => (String::from("Service unavailable"), 503),
            CampaignsDeleteCampaignError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Get a campaign for an organization](Campaigns::get_campaign_summary_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CampaignsGetCampaignSummaryError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Unprocessable Entity")]
    Status422(BasicError),
    #[error("Service unavailable")]
    Status503(GetBillingGetGithubBillingUsageReportUserResponse503),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<CampaignsGetCampaignSummaryError> for AdapterError {
    fn from(err: CampaignsGetCampaignSummaryError) -> Self {
        let (description, status_code) = match err {
            CampaignsGetCampaignSummaryError::Status404(_) => (String::from("Resource not found"), 404),
            CampaignsGetCampaignSummaryError::Status422(_) => (String::from("Unprocessable Entity"), 422),
            CampaignsGetCampaignSummaryError::Status503(_) => (String::from("Service unavailable"), 503),
            CampaignsGetCampaignSummaryError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [List campaigns for an organization](Campaigns::list_org_campaigns_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CampaignsListOrgCampaignsError {
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Service unavailable")]
    Status503(GetBillingGetGithubBillingUsageReportUserResponse503),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<CampaignsListOrgCampaignsError> for AdapterError {
    fn from(err: CampaignsListOrgCampaignsError) -> Self {
        let (description, status_code) = match err {
            CampaignsListOrgCampaignsError::Status404(_) => (String::from("Resource not found"), 404),
            CampaignsListOrgCampaignsError::Status503(_) => (String::from("Service unavailable"), 503),
            CampaignsListOrgCampaignsError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}

/// Errors for the [Update a campaign](Campaigns::update_campaign_async()) endpoint.
#[derive(Debug, thiserror::Error)]
pub enum CampaignsUpdateCampaignError {
    #[error("Bad Request")]
    Status400(BasicError),
    #[error("Resource not found")]
    Status404(BasicError),
    #[error("Unprocessable Entity")]
    Status422(BasicError),
    #[error("Service unavailable")]
    Status503(GetBillingGetGithubBillingUsageReportUserResponse503),
    #[error("Status code: {}", code)]
    Generic { code: u16 },
}

impl From<CampaignsUpdateCampaignError> for AdapterError {
    fn from(err: CampaignsUpdateCampaignError) -> Self {
        let (description, status_code) = match err {
            CampaignsUpdateCampaignError::Status400(_) => (String::from("Bad Request"), 400),
            CampaignsUpdateCampaignError::Status404(_) => (String::from("Resource not found"), 404),
            CampaignsUpdateCampaignError::Status422(_) => (String::from("Unprocessable Entity"), 422),
            CampaignsUpdateCampaignError::Status503(_) => (String::from("Service unavailable"), 503),
            CampaignsUpdateCampaignError::Generic { code } => (String::from("Generic"), code)
        };

        Self::Endpoint {
            description,
            status_code,
            source: Some(Box::new(err))
        }
    }
}


/// Query parameters for the [List campaigns for an organization](Campaigns::list_org_campaigns_async()) endpoint.
#[derive(Default, Serialize)]
pub struct CampaignsListOrgCampaignsParams<'req> {
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    page: Option<u16>, 
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    per_page: Option<u16>, 
    /// The direction to sort the results by.
    direction: Option<&'req str>, 
    /// If specified, only campaigns with this state will be returned.
    state: Option<CampaignState>, 
    /// The property by which to sort the results.
    sort: Option<&'req str>
}

impl<'req> CampaignsListOrgCampaignsParams<'req> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn page(self, page: u16) -> Self {
        Self {
            page: Some(page),
            per_page: self.per_page, 
            direction: self.direction, 
            state: self.state, 
            sort: self.sort, 
        }
    }

    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub fn per_page(self, per_page: u16) -> Self {
        Self {
            page: self.page, 
            per_page: Some(per_page),
            direction: self.direction, 
            state: self.state, 
            sort: self.sort, 
        }
    }

    /// The direction to sort the results by.
    pub fn direction(self, direction: &'req str) -> Self {
        Self {
            page: self.page, 
            per_page: self.per_page, 
            direction: Some(direction),
            state: self.state, 
            sort: self.sort, 
        }
    }

    /// If specified, only campaigns with this state will be returned.
    pub fn state(self, state: CampaignState) -> Self {
        Self {
            page: self.page, 
            per_page: self.per_page, 
            direction: self.direction, 
            state: Some(state),
            sort: self.sort, 
        }
    }

    /// The property by which to sort the results.
    pub fn sort(self, sort: &'req str) -> Self {
        Self {
            page: self.page, 
            per_page: self.per_page, 
            direction: self.direction, 
            state: self.state, 
            sort: Some(sort),
        }
    }
}

impl<'enc> From<&'enc PerPage> for CampaignsListOrgCampaignsParams<'enc> {
    fn from(per_page: &'enc PerPage) -> Self {
        Self {
            per_page: Some(per_page.per_page),
            page: Some(per_page.page),
            ..Default::default()
        }
    }
}

impl<'api, C: Client> Campaigns<'api, C> where AdapterError: From<<C as Client>::Err> {
    /// ---
    ///
    /// # Create a campaign for an organization
    ///
    /// Create a campaign for an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    /// 
    /// Fine-grained tokens must have the "Code scanning alerts" repository permissions (read) on all repositories included
    /// in the campaign.
    ///
    /// [GitHub API docs for create_campaign](https://docs.github.com/rest/campaigns/campaigns#create-a-campaign-for-an-organization)
    ///
    /// ---
    pub async fn create_campaign_async(&self, org: &str, body: PostCampaignsCreateCampaign) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PostCampaignsCreateCampaign>(body)?),
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
                400 => Err(CampaignsCreateCampaignError::Status400(github_response.to_json_async().await?).into()),
                404 => Err(CampaignsCreateCampaignError::Status404(github_response.to_json_async().await?).into()),
                422 => Err(CampaignsCreateCampaignError::Status422(github_response.to_json_async().await?).into()),
                429 => Err(CampaignsCreateCampaignError::Status429.into()),
                503 => Err(CampaignsCreateCampaignError::Status503(github_response.to_json_async().await?).into()),
                code => Err(CampaignsCreateCampaignError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Create a campaign for an organization
    ///
    /// Create a campaign for an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    /// 
    /// Fine-grained tokens must have the "Code scanning alerts" repository permissions (read) on all repositories included
    /// in the campaign.
    ///
    /// [GitHub API docs for create_campaign](https://docs.github.com/rest/campaigns/campaigns#create-a-campaign-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_campaign(&self, org: &str, body: PostCampaignsCreateCampaign) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns", super::GITHUB_BASE_API_URL, org);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PostCampaignsCreateCampaign>(body)?),
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
                400 => Err(CampaignsCreateCampaignError::Status400(github_response.to_json()?).into()),
                404 => Err(CampaignsCreateCampaignError::Status404(github_response.to_json()?).into()),
                422 => Err(CampaignsCreateCampaignError::Status422(github_response.to_json()?).into()),
                429 => Err(CampaignsCreateCampaignError::Status429.into()),
                503 => Err(CampaignsCreateCampaignError::Status503(github_response.to_json()?).into()),
                code => Err(CampaignsCreateCampaignError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Delete a campaign for an organization
    ///
    /// Deletes a campaign in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for delete_campaign](https://docs.github.com/rest/campaigns/campaigns#delete-a-campaign-for-an-organization)
    ///
    /// ---
    pub async fn delete_campaign_async(&self, org: &str, campaign_number: i32) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


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
                404 => Err(CampaignsDeleteCampaignError::Status404(github_response.to_json_async().await?).into()),
                503 => Err(CampaignsDeleteCampaignError::Status503(github_response.to_json_async().await?).into()),
                code => Err(CampaignsDeleteCampaignError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Delete a campaign for an organization
    ///
    /// Deletes a campaign in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for delete_campaign](https://docs.github.com/rest/campaigns/campaigns#delete-a-campaign-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn delete_campaign(&self, org: &str, campaign_number: i32) -> Result<(), AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


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
                404 => Err(CampaignsDeleteCampaignError::Status404(github_response.to_json()?).into()),
                503 => Err(CampaignsDeleteCampaignError::Status503(github_response.to_json()?).into()),
                code => Err(CampaignsDeleteCampaignError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a campaign for an organization
    ///
    /// Gets a campaign for an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_campaign_summary](https://docs.github.com/rest/campaigns/campaigns#get-a-campaign-for-an-organization)
    ///
    /// ---
    pub async fn get_campaign_summary_async(&self, org: &str, campaign_number: i32) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


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
                404 => Err(CampaignsGetCampaignSummaryError::Status404(github_response.to_json_async().await?).into()),
                422 => Err(CampaignsGetCampaignSummaryError::Status422(github_response.to_json_async().await?).into()),
                503 => Err(CampaignsGetCampaignSummaryError::Status503(github_response.to_json_async().await?).into()),
                code => Err(CampaignsGetCampaignSummaryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Get a campaign for an organization
    ///
    /// Gets a campaign for an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for get_campaign_summary](https://docs.github.com/rest/campaigns/campaigns#get-a-campaign-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_campaign_summary(&self, org: &str, campaign_number: i32) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


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
                404 => Err(CampaignsGetCampaignSummaryError::Status404(github_response.to_json()?).into()),
                422 => Err(CampaignsGetCampaignSummaryError::Status422(github_response.to_json()?).into()),
                503 => Err(CampaignsGetCampaignSummaryError::Status503(github_response.to_json()?).into()),
                code => Err(CampaignsGetCampaignSummaryError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # List campaigns for an organization
    ///
    /// Lists campaigns in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for list_org_campaigns](https://docs.github.com/rest/campaigns/campaigns#list-campaigns-for-an-organization)
    ///
    /// ---
    pub async fn list_org_campaigns_async(&self, org: &str, query_params: Option<impl Into<CampaignsListOrgCampaignsParams<'api>>>) -> Result<Vec<CampaignSummary>, AdapterError> {

        let mut request_uri = format!("{}/orgs/{}/campaigns", super::GITHUB_BASE_API_URL, org);

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
                404 => Err(CampaignsListOrgCampaignsError::Status404(github_response.to_json_async().await?).into()),
                503 => Err(CampaignsListOrgCampaignsError::Status503(github_response.to_json_async().await?).into()),
                code => Err(CampaignsListOrgCampaignsError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # List campaigns for an organization
    ///
    /// Lists campaigns in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for list_org_campaigns](https://docs.github.com/rest/campaigns/campaigns#list-campaigns-for-an-organization)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn list_org_campaigns(&self, org: &str, query_params: Option<impl Into<CampaignsListOrgCampaignsParams<'api>>>) -> Result<Vec<CampaignSummary>, AdapterError> {

        let mut request_uri = format!("{}/orgs/{}/campaigns", super::GITHUB_BASE_API_URL, org);

        if let Some(params) = query_params {
            request_uri.push_str("?");
            let qp: CampaignsListOrgCampaignsParams = params.into();
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
                404 => Err(CampaignsListOrgCampaignsError::Status404(github_response.to_json()?).into()),
                503 => Err(CampaignsListOrgCampaignsError::Status503(github_response.to_json()?).into()),
                code => Err(CampaignsListOrgCampaignsError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Update a campaign
    ///
    /// Updates a campaign in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_campaign](https://docs.github.com/rest/campaigns/campaigns#update-a-campaign)
    ///
    /// ---
    pub async fn update_campaign_async(&self, org: &str, campaign_number: i32, body: PatchCampaignsUpdateCampaign) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PatchCampaignsUpdateCampaign>(body)?),
            method: "PATCH",
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
                400 => Err(CampaignsUpdateCampaignError::Status400(github_response.to_json_async().await?).into()),
                404 => Err(CampaignsUpdateCampaignError::Status404(github_response.to_json_async().await?).into()),
                422 => Err(CampaignsUpdateCampaignError::Status422(github_response.to_json_async().await?).into()),
                503 => Err(CampaignsUpdateCampaignError::Status503(github_response.to_json_async().await?).into()),
                code => Err(CampaignsUpdateCampaignError::Generic { code }.into()),
            }
        }
    }

    /// ---
    ///
    /// # Update a campaign
    ///
    /// Updates a campaign in an organization.
    /// 
    /// The authenticated user must be an owner or security manager for the organization to use this endpoint.
    /// 
    /// OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint.
    ///
    /// [GitHub API docs for update_campaign](https://docs.github.com/rest/campaigns/campaigns#update-a-campaign)
    ///
    /// ---
    #[cfg(not(target_arch = "wasm32"))]
    pub fn update_campaign(&self, org: &str, campaign_number: i32, body: PatchCampaignsUpdateCampaign) -> Result<CampaignSummary, AdapterError> {

        let request_uri = format!("{}/orgs/{}/campaigns/{}", super::GITHUB_BASE_API_URL, org, campaign_number);


        let req = GitHubRequest {
            uri: request_uri,
            body: Some(C::from_json::<PatchCampaignsUpdateCampaign>(body)?),
            method: "PATCH",
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
                400 => Err(CampaignsUpdateCampaignError::Status400(github_response.to_json()?).into()),
                404 => Err(CampaignsUpdateCampaignError::Status404(github_response.to_json()?).into()),
                422 => Err(CampaignsUpdateCampaignError::Status422(github_response.to_json()?).into()),
                503 => Err(CampaignsUpdateCampaignError::Status503(github_response.to_json()?).into()),
                code => Err(CampaignsUpdateCampaignError::Generic { code }.into()),
            }
        }
    }

}
