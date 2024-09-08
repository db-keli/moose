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