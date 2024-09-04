#![allow(dead_code, unused_imports)]
use crate::github_client::GithubClient;
use crate::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue};
use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::error::Error;

const GITHUB_API_URL: &str = "https://api.github.com";

impl GithubClient {
    pub fn list_issues(&self, repo: &str) -> Result<Vec<Issue>, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues",
            GITHUB_API_URL, self.owner, repo
        );
        let issues = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()?
            .json()?;
        Ok(issues)
    }

    pub fn create_issue(&self, issue: &CreateIssue, repo: &str) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues",
            GITHUB_API_URL, self.owner, repo
        );
        let headers = self.build_headers();
        let created_issue = self.client.post(&url).headers(headers).json(issue).send()?;
        Ok(created_issue)
    }

    pub fn update_issue(
        &self,
        issue_number: u64,
        issue: &UpdateIssue,
        repo: &str
    ) -> Result<Issue, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            GITHUB_API_URL, self.owner, repo, issue_number
        );
        let updated_issue = self
            .client
            .patch(&url)
            .headers(self.build_headers())
            .json(issue)
            .send()?
            .json()?;
        Ok(updated_issue)
    }

    pub fn close_issue(&self, issue_number: u64, repo: &str) -> Result<(), Box<dyn Error>> {
        self.update_issue(
            issue_number,
            &UpdateIssue {
                title: None,
                body: None,
                state: Some("closed".into()),
            },
            repo
        )?;
        Ok(())
    }

    pub fn comment_issue(
        &self,
        number: u64,
        comment: &CreateIssueComment,
        repo: &str
    ) -> Result<Response, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            GITHUB_API_URL, self.owner, repo, number
        );
        let mut headers = self.build_headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let comment = self
            .client
            .post(&url)
            .headers(headers)
            .json(comment)
            .send()?;

        Ok(comment)
    }

    pub fn list_issue_comments(&self, number: u64, repo: &str) -> Result<Vec<IssueComment>, Box<dyn Error>> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/comments",
            GITHUB_API_URL, self.owner, repo, number
        );
        let comments = self
            .client
            .get(&url)
            .headers(self.build_headers())
            .send()?
            .json()?;
        Ok(comments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_test() {
        let client =
            GithubClient::new("owner".to_string(), "token".to_string());
    }
}
