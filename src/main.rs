#![allow(dead_code, unused)]

mod github_client;
mod models;

use chrono::{DateTime, Utc};
use dotenv::dotenv;
use github_client::GithubClient;
use models::{CreateIssue, IssueComment, UpdateIssue, CreateIssueComment};
use std::collections::HashSet;
use std::io::Read;
use std::time::Duration;
use std::{env, io, thread};

fn main() -> () {
    dotenv().ok();
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO not set");

    let client = GithubClient::new(token, owner, repo);

    // let mut body = String::new();
    // let mut title = String::new();

    // io::stdin().read_line(&mut title);
    // io::stdin().read_line(&mut body);

    // let issue = CreateIssue { title, body };
    // let created_issue = client.create_issue(issue).unwrap();
    // println!("{}", created_issue.text().unwrap());

    let number = 4;
    let message = "testing moose: creating a comment from terminal".to_string();

    // creating an issue comment interface
    let comment = CreateIssueComment { body: message };

    // Comment on issue
    let status = client.comment_issue(number, comment);
    println!("{:?}", status.unwrap().status());

    let mut seen_issues: HashSet<u64> = HashSet::new();

    loop {
        let issues = client.list_issues().unwrap();

        for issue in issues {
            if !seen_issues.contains(&issue.number) {
                println!("New issue found: {:?}", issue);
                let issue_comments = client.list_issue_comments(issue.number).unwrap();
                println!("Issue comments: {:?}", issue_comments);
                seen_issues.insert(issue.number);
            }
        }
    }

    let close_status = client.close_issue(4).unwrap();
    println!("{:?}", close_status);
}
