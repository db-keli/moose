mod github_client;
mod models;

use chrono::{DateTime, Utc};
use github_client::GithubClient;
use models::{CreateIssue, UpdateIssue};
use std::collections::HashSet;
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO not set");

    let client = GithubClient::new(token, owner, repo);

    let mut seen_issues: HashSet<u64> = HashSet::new();

    loop {
        let issues = client.list_issues()?;

        for issue in issues {
            if !seen_issues.contains(&issue.number) {
                println!("New issue found: {:?}", issue);
                seen_issues.insert(issue.number);
            }
        }

        thread::sleep(Duration::from_secs(60)); 
    }
}
