//! Endpoints module and `PerPage` struct/impl
/* 
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * OpenAPI spec version: 1.1.4
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

pub const GITHUB_BASE_API_URL: &str = if cfg!(feature = "mock") {
    "http://localhost:8080"
} else {
    "https://api.github.com"
};

pub mod meta;
pub mod issues;
pub mod licenses;
pub mod reactions;
pub mod activity;
pub mod projects;
pub mod orgs;
pub mod users;
pub mod apps;
pub mod rate_limit;
pub mod repos;
pub mod secret_scanning;
pub mod security_advisories;
pub mod packages;
pub mod search;
pub mod classroom;
pub mod teams;
pub mod private_registries;
pub mod oidc;
pub mod markdown;
pub mod actions;
pub mod migrations;
pub mod code_security;
pub mod gists;
pub mod hosted_compute;
pub mod campaigns;
pub mod dependency_graph;
pub mod copilot;
pub mod dependabot;
pub mod codes_of_conduct;
pub mod pulls;
pub mod gitignore;
pub mod git;
pub mod code_scanning;
pub mod checks;
pub mod billing;
pub mod interactions;
pub mod codespaces;
pub mod emojis;

pub struct PerPage {
    per_page: u16,
    page: u16,
}

impl PerPage {
    pub fn new(per_page: u16) -> Self {
        PerPage { per_page, page: 0 }
    }

    pub fn page(&mut self, page: u16) -> &mut Self {
        self.page = page;
        self
    }
}

impl std::convert::AsRef<PerPage> for PerPage {
    fn as_ref(&self) -> &PerPage {
        self
    }
}
