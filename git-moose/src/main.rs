#![allow(dead_code, unused_imports)]

mod database_models;
mod github_client;
mod issues;
mod models;
mod repository;
mod schema;

use self::schema::admin::dsl::*;
use chrono::{DateTime, Utc};
use database_models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use github_client::GithubClient;
use models::{CreateIssue, CreateIssueComment, IssueComment, UpdateIssue};
use moose::crud::create_admin;
use moose::establish_connection;
use schema::admin::all_columns;
use std::collections::HashSet;
use std::io::{stdin, Read};
use std::time::Duration;
use std::{env, io, thread};

pub const GITHUB_API_URL: &str = "https://api.github.com";

fn main() -> () {
    // dotenv().ok();
    // let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    // let owner = env::var("GITHUB_OWNER").expect("GITHUB_OWNER not set");
    // let repo = env::var("GITHUB_REPO").expect("GITHUB_REPO not set");

    // let client = GithubClient::new(token, owner, repo);

    // let mut body = String::new();
    // let mut title = String::new();

    // io::stdin().read_line(&mut title);
    // io::stdin().read_line(&mut body);

    // let issue = CreateIssue { title, body };
    // let created_issue = client.create_issue(issue).unwrap();
    // println!("{}", created_issue.text().unwrap());

    // let number = 4;
    // let message = "testing moose: creating a comment from terminal".to_string();

    // // creating an issue comment interface
    // let comment = CreateIssueComment { body: message };

    // // Comment on issue
    // let status = client.comment_issue(number, &comment);
    // println!("{:?}", status.unwrap().status());

    // let mut seen_issues: HashSet<u64> = HashSet::new();

    // let issues = client.list_issues().unwrap();

    // for issue in issues {
    //     if !seen_issues.contains(&issue.number) {
    //         println!("New issue found: {:?}", issue);
    //         let issue_comments = client.list_issue_comments(issue.number).unwrap();
    //         println!("Issue comments: {:?}", issue_comments);
    //         seen_issues.insert(issue.number);
    //     }
    // }

    // let user_repositories = client.get_repositories(None).unwrap();
    // for repo in user_repositories {
    //     println!("{:?}", repo);
    // }

    // let connection = &mut establish_connection();

    // let mut username = String::new();
    // let mut token = String::new();

    // println!("Enter your username: ");
    // stdin().read_line(&mut username).unwrap();
    // let username = &username[..(username.len() - 1)];

    // println!("Enter token: press {} when finished", EOF);
    // stdin().read_to_string(&mut token).unwrap();
    // let token = &token[..(token.len() - 1)];

    // let admin = create_admin(connection, username.to_string(), token.to_string());
    // println!("\n Saved draft with {}", admin.username);

    let connection = &mut establish_connection();
    let results = admin
        .limit(5)
        .select(Admin::as_select())
        .load(connection)
        .expect("Failed to load database");

    for result in results {
        println!("{}: {}", result.username, result.token);
    }
}

#[cfg(not(windows))]
const EOF: &str = "Ctrl-D";

#[cfg(windows)]
const EOF: &str = "Ctrl-Z";
