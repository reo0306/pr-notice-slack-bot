use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub html_url: String,
    pub number: u32,
    pub state: String,
    pub title: String,
    pub user: User,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reviewers {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    pub user: User,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
    pub html_url: String,
}

#[derive(Serialize, Debug)]
pub struct TextLine<'a> {
    pub repo: &'a Repository,
    pub pull: &'a PullRequest,
    pub reviewers: Reviewers,
    pub reviews: Vec<Review>,
}

impl<'a> TextLine<'a> {
    pub fn new(repo: &'a Repository, pull: &'a PullRequest, reviewers: Reviewers, reviews: Vec<Review>) -> Self {
        Self { repo, pull, reviewers, reviews }
    }

    pub fn title(&self) -> String {
        format!(
            "*{} - <{}|{}#{}>*",
            &self.pull.title,
            &self.pull.html_url,
            &self.repo.full_name,
            &self.pull.number
        )
    }

    pub fn unapproved_reviewers(&self) -> String {
        let mut users: Vec<String> = Vec::new();

        let _ = &self.reviewers.users.clone()
            .into_iter()
            .for_each(|reviewer| {
                if &self.reviews.len() == &0_usize {
                    users.push((&reviewer.login).to_string());
                }

                let _ = &self.reviews.clone()
                    .into_iter()
                    .filter(|review| {
                        &review.user.login.len() > &0_usize && &reviewer.login == &review.user.login && review.state != "APPROVED".to_string()
                    })
                    .for_each(|_| {
                        users.push((&reviewer.login).to_string());
                    });
            });

        if users.len() == 0 {
            format!("")
        } else {
            format!("unapproved reviewers - {}", users.join(" "))
        }
    }

    pub fn state(&self) -> String {
        format!(
            "*{}* - Created by <{}|{}> on {}",
            &self.pull.state,
            &self.pull.user.html_url,
            &self.pull.user.login,
            NaiveDateTime::parse_from_str(&self.pull.created_at, "%Y-%m-%dT%H:%M:%SZ").unwrap(),
        )
    }

    pub fn message(&self) -> String {
        [
            self.title(),
            self.unapproved_reviewers(),
            self.state()
        ].join("\n")
    }
}