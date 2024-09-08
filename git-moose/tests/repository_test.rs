use moose::GITHUB_API_URL;
use moose::github_client::GithubClient;
use moose::models::{CreateIssue, CreateIssueComment, Issue, IssueComment, UpdateIssue};
use std::error::Error;
use std::env;
use dotenv::dotenv;

#[test]
fn test_get_repos() {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    let client = GithubClient::new(token, owner);
    let repos = client.get_repositories(Some("db-keli")).unwrap();

    let status = repos.0;
    assert_eq!(status, 200);
}

#[test]
fn test_get_repo_pr() {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    let client = GithubClient::new(token, owner);
    let prs = client.get_repo_pr("moose").unwrap();
    let status = prs.0;
    assert_eq!(status, 200);
}