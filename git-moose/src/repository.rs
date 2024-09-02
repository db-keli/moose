use crate::github_client::GithubClient;
use crate::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue, UserRepos};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::collections::HashMap;
use std::error::Error;

const GITHUB_API_URL: &str = "https://api.github.com";

impl GithubClient {
    pub fn get_repositories(
        &self,
        username: Option<&str>,
    ) -> Result<Vec<UserRepos>, Box<dyn Error>> {
        if username.is_some() {
            let url = format!("{}/{}/repos", GITHUB_API_URL, username.unwrap().to_string());
            let repositories = self
                .client
                .get(&url)
                .headers(self.build_headers())
                .send()?
                .json()?;
            return Ok(repositories);
        } else {
            let url = format!("{}/user/repos", GITHUB_API_URL);
            let repositories = self
                .client
                .get(&url)
                .headers(self.build_headers())
                .send()?
                .json()?;
            Ok(repositories)
        }
    }
}
