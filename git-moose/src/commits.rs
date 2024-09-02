use crate::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::error::Error;
use crate::github_client::GithubClient;

const GITHUB_API_URL: &str = "https://api.github.com";

impl GithubClient {
}