use crate::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::error::Error;

const GITHUB_API_URL: &str = "https://api.github.com";

pub struct GithubClient {
    pub client: Client,
    pub token: String,
    pub owner: String,
    pub repo: String,
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

    pub fn build_headers(&self) -> HeaderMap {
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
        headers.insert(
            "x-Github-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );
        headers
    }

}
