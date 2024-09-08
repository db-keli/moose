use moose::github_client::GithubClient;
use moose::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue};
use moose::GITHUB_API_URL;
use std::error::Error;
use dotenv::dotenv;
use std::env;

#[test]
fn test_get_commits() {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");

    let client = GithubClient::new(token, owner);

    let issues = client.list_issues("moose").unwrap();
    let status = issues.0;
    assert_eq!(status, 200);
}

#[test]
fn test_create_issue() {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    let title = "Test issue";
    let body = "This is a test issue.";
    let client = GithubClient::new(token, owner);
    let issue = CreateIssue {
        title: title.to_string(),
        body: body.to_string(),
    };

    let response = client.create_issue(&issue, "moose").unwrap();
    let status = response.status();
    assert_eq!(status, 201);
}