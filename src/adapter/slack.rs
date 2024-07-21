use std::env;

use crate::domain::model::slack::{Slack, Message, Text};

pub struct SlackApi;

impl SlackApi {
    pub fn construct_slack_message(message: &Vec<String>) -> Slack {
        let text_lines = message.join("\n");

        Slack {
            blocks: vec![
                Message {
                    r#type: "section".to_string(),
                    text: Text {
                        r#type: "mrkdwn".to_string(),
                        text: text_lines.to_string(),
                    },
                }
            ]
        }
    }

    pub async fn send_message(slack: &Slack) -> Result<(), reqwest::Error>{
        let client = reqwest::Client::new();

        let webhook_uri = env::var("WEBHOOK_URI").unwrap();

        let message = serde_json::json!(slack);

        let response = client
            .post(webhook_uri)
            .json(&message)
            .send()
            .await?;

        if response.status().is_success() == false {
            println!("Failed to send notification: {:?}", response.text().await?);
        }

        Ok(())
    }
}

#[cfg(test)]
mod slack_api_test{
    use rstest::rstest;

    use super::*;
    use crate::domain::model::{
        slack::{
            Message,
            Text,
        },
        github::{
            PullRequest,
            Repository,
            Reviewers,
            Review,
            TextLine,
            User
       }
    };

    #[rstest]
    #[case(
        Repository {
            name: "gospo".to_string(),
            full_name: "reo0306/gospo".to_string(),
            url: "https://api.github.com/repos/reo0306/gospo".to_string(),
        },
        PullRequest {
            html_url: "https://github.com/reo0306/gospo/pull/1".to_string(),
            number: 1,
            state: "open".to_string(),
            title: "Test".to_string(),
            user: User {
                login: "test".to_string(),
                html_url: "https://github.com/reo0306".to_string(),
            },
            created_at: "2024-07-16T20:09:31Z".to_string(),
        },
        Reviewers {
            users: vec![
                User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                User {
                    login: "test2".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
            ],
        },
        vec![
            Review {
                user: User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                state: "APPROVED".to_string(),
            },
            Review {
                user: User {
                    login: "test2".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                state: "COMMENTED".to_string(),
            },
        ],
        "*Test - <https://github.com/reo0306/gospo/pull/1|reo0306/gospo#1>*\nunapproved reviewers - test2\n*open* - Created by <https://github.com/reo0306|test> on 2024-07-16 20:09:31\n*Test - <https://github.com/reo0306/gospo/pull/1|reo0306/gospo#1>*\nunapproved reviewers - test2\n*open* - Created by <https://github.com/reo0306|test> on 2024-07-16 20:09:31".to_string(),
    )]
    #[case(
        Repository {
            name: "gospo".to_string(),
            full_name: "reo0306/gospo".to_string(),
            url: "https://api.github.com/repos/reo0306/gospo".to_string(),
        },
        PullRequest {
            html_url: "https://github.com/reo0306/gospo/pull/1".to_string(),
            number: 1,
            state: "open".to_string(),
            title: "Test".to_string(),
            user: User {
                login: "test".to_string(),
                html_url: "https://github.com/reo0306".to_string(),
            },
            created_at: "2024-07-16T20:09:31Z".to_string(),
        },
        Reviewers {
            users: vec![
                User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                User {
                    login: "test2".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
            ],
        },
        Vec::new(),
        "*Test - <https://github.com/reo0306/gospo/pull/1|reo0306/gospo#1>*\nunapproved reviewers - test test2\n*open* - Created by <https://github.com/reo0306|test> on 2024-07-16 20:09:31\n*Test - <https://github.com/reo0306/gospo/pull/1|reo0306/gospo#1>*\nunapproved reviewers - test test2\n*open* - Created by <https://github.com/reo0306|test> on 2024-07-16 20:09:31".to_string(),
    )]
    fn test_slack_message_text_lines(
        #[case] repo: Repository,
        #[case] pull: PullRequest,
        #[case] reviewers: Reviewers,
        #[case] reviews: Vec<Review>,
        #[case] result: String,
    ) {
        let text_line = TextLine::new(&repo, &pull, reviewers, reviews);

        let mut message = Vec::new();
        message.push(text_line.message());
        message.push(text_line.message());

        let text_lines = message.join("\n");

        assert_eq!(result, text_lines);
    }

    #[rstest]
    #[case(
        Repository {
            name: "gospo".to_string(),
            full_name: "reo0306/gospo".to_string(),
            url: "https://api.github.com/repos/reo0306/gospo".to_string(),
        },
        PullRequest {
            html_url: "https://github.com/reo0306/gospo/pull/1".to_string(),
            number: 1,
            state: "open".to_string(),
            title: "Test".to_string(),
            user: User {
                login: "test".to_string(),
                html_url: "https://github.com/reo0306".to_string(),
            },
            created_at: "2024-07-16T20:09:31Z".to_string(),
        },
        Reviewers {
            users: vec![
                User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                User {
                    login: "test2".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
            ],
        },
        vec![
            Review {
                user: User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                state: "APPROVED".to_string(),
            },
            Review {
                user: User {
                    login: "test2".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
                state: "COMMENTED".to_string(),
            },
        ],
    )]
    fn test_slack(
        #[case] repo: Repository,
        #[case] pull: PullRequest,
        #[case] reviewers: Reviewers,
        #[case] reviews: Vec<Review>,
    ) {
        let text_lines = TextLine::new(&repo, &pull, reviewers, reviews);

        let mut message = Vec::new();
        message.push(text_lines.message());

        let slack = SlackApi::construct_slack_message(&message);

        assert_eq!(
            vec![
                Message {
                    r#type: "section".to_string(),
                    text: Text {
                        r#type: "mrkdwn".to_string(),
                        text: text_lines.message().to_string(),
                    }
                }
            ],
            slack.blocks
        );
    }
}