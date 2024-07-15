#![allow(dead_code, unused)]
use crate::models::{CreateIssue, Issue, UpdateIssue};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use std::error::Error;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GithubClient {
    client: Client,
    token: String,
    owner: String,
    repo: String,
}

impl GithubClient {
    pub fn new(token: String, owner: String, repo: String) -> Self {
        let client = Client::new();
        GithubClient {
            client,
            token,
            owner,
            repo,
        }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("token {}", self.token)).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("rust-github-client"));
        headers.insert(
            ACCEPT,
            HeaderValue::from_str("application/vnd.github+json").unwrap(),
        );
        headers
    }

    pub fn list_issues(&self) -> Result<Vec<Issue>, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues",
            GITHUB_API_URL, self.owner, self.repo
        );
        let issues = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()?
            .json()?;
        Ok(issues)
    }

    pub fn create_issue(&self, issue: CreateIssue) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues",
            GITHUB_API_URL, self.owner, self.repo
        );
        let mut headers = self.build_headers();
        let created_issue = self
            .client
            .post(&url)
            .headers(headers)
            .json(&issue)
            .send()?;
        Ok(created_issue)
    }

    pub fn update_issue(
        &self,
        issue_number: u64,
        issue: UpdateIssue,
    ) -> Result<Issue, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            GITHUB_API_URL, self.owner, self.repo, issue_number
        );
        let updated_issue = self
            .client
            .patch(&url)
            .headers(self.build_headers())
            .json(&issue)
            .send()?
            .json()?;
        Ok(updated_issue)
    }

    pub fn close_issue(&self, issue_number: u64) -> Result<(), Box<dyn Error>> {
        self.update_issue(
            issue_number,
            UpdateIssue {
                title: None,
                body: None,
                state: Some("closed".into()),
            },
        )?;
        Ok(())
    }
}
