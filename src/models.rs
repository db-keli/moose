use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: Option<String>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssue {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIssue {
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIssueComment {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueComment {
    pub body: String,
    pub user: Option<User>,
}
