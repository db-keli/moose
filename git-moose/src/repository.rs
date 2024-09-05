use crate::github_client::GithubClient;
use crate::models::{
    CreateIssue, CreateIssueComment, Issue, IssueComment, PullRequest, UpdateIssue, UserRepos,
};
use crate::GITHUB_API_URL;
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::collections::HashMap;
use std::error::Error;

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

    pub fn get_repo_pr(&self, repo: &str) -> Result<Vec<PullRequest>, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/pulls",
            GITHUB_API_URL,
            self.owner,
            repo.to_string()
        );
        let pull_requests = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()?
            .json()?;
        Ok(pull_requests)
    }
}
