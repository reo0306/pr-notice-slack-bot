use anyhow::Result;

use crate::domain::model::github::{
    Repository,
    PullRequest,
    Reviewers,
    Review,
    TextLine
};
use crate::adapter::{
    github::GithubApi,
    slack::SlackApi,
};

const GITHUB_API_URI: &str = "https://api.github.com";

pub struct Batch {
    github_api: GithubApi,
    message: Vec<String>,
}

impl Batch {
    pub fn new() -> Self {
        Self {
            github_api: GithubApi::new(),
            message: Vec::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let repositories = self.github_api.fetch::<Repository>(&format!("{}/user/repos?page=3", GITHUB_API_URI)).await?;

        self.create_slack_message(repositories).await?;

        self.slack_api().await?;

        Ok(())
    }

    async fn create_slack_message(&mut self, repositories: Vec<Repository>) -> Result<()> {
        self.message.push("*Open Pull Request*".to_string());

        for repo in &repositories {
            let pulls = self.github_api.fetch::<PullRequest>(&format!("{}/pulls?state=open", repo.url)).await?;

            for pull in pulls {
                let requested_reviewers = self.github_api
                                            .find::<Reviewers>(&format!("{}/pulls/{}/requested_reviewers", &repo.url, pull.number))
                                            .await?;

                if requested_reviewers.users.len() == 0 {
                    let review = Vec::new();
                    let text_line = TextLine::new(repo, &pull, requested_reviewers.clone(), review);
                    self.message.push(text_line.message());
                    continue;
                }

                let reviews = self.github_api.fetch::<Review>(&format!("{}/pulls/{}/reviews", &repo.url, pull.number)).await?;

                let text_line = TextLine::new(repo, &pull, requested_reviewers.clone(), reviews);

                self.message.push(text_line.message());
            }
        }

        Ok(())
    }

    async fn slack_api(&self) -> Result<()> {
        let slack = SlackApi::construct_slack_message(&self.message);

        SlackApi::send_message(&slack).await?;

        Ok(()) 
    }
}