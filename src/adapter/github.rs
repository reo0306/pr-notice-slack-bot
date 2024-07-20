use std::env;
use serde::Deserialize;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, USER_AGENT, HeaderMap, HeaderValue}
};
use anyhow::Result;

pub struct GithubApi {
    client: Client,
    headers: HeaderMap,
}

impl GithubApi {
    pub fn new() -> Self {
        let token = env::var("GITHUB_TOKEN").unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("request"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token)).unwrap());
        headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));

        let client = reqwest::Client::new();

        Self { client, headers }
    }

    pub async fn fetch<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<Vec<T>> {
        let response = self.client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {
                let repos = response.json::<Vec<T>>().await?;
                Ok(repos)
            },
            Err(e) => {
                Err(e.into())
            }
        }
    }

    pub async fn find<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let response = self.client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?;

        match response.error_for_status_ref() {
            Ok(_) => {
                let repos = response.json::<T>().await?;
                Ok(repos)
            },
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod github_api_test {
    use rstest::{rstest, fixture};

    use super::*;
    use crate::domain::model::github::{
        PullRequest,
        Repository,
        Reviewers,
        Review,
        User
    };

    #[fixture]
    fn github_api() -> GithubApi {
        dotenvy::dotenv().unwrap();
        GithubApi::new()
    }

    #[rstest]
    #[case(
        Repository {
            name: String::from("polymer-cli"),
            full_name: String::from("reo0306/polymer-cli"),
            url: String::from("https://api.github.com/repos/reo0306/polymer-cli"),
        },
        "/?page=1"
    )]
    #[tokio::test]
    async fn it_repositories(
        github_api: GithubApi,
        #[case] data: Repository,
        #[case] path: &str
    ) {
        let server = mockito::Server::new_async().await;

        let response = vec![&data];
        let body = serde_json::to_string(&response).unwrap();
        let (server, mock) = mock_server(server, path, body).await;

        let repos = github_api.fetch::<Repository>(&format!("{}{}", server.url(), path)).await.unwrap();

        for repo in repos {
            assert_eq!(&data.name, &repo.name);
            assert_eq!(&data.full_name, &repo.full_name);
            assert_eq!(&data.url, &repo.url);
        }

        mock.assert_async().await;
    }

    #[rstest]
    #[case(
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
        "/repos/reo0306/gospo/pulls",
    )]
    #[tokio::test]
    async fn it_pullrequests(
        github_api: GithubApi,
        #[case] data: PullRequest,
        #[case] path: &str
    ) {
        let server = mockito::Server::new_async().await;

        let response = vec![&data];
        let body = serde_json::to_string(&response).unwrap();
        let (server, mock) = mock_server(server, path, body).await;

        let pulls = github_api.fetch::<PullRequest>(&format!("{}{}", server.url(), path)).await.unwrap();

        for pull in pulls {
            assert_eq!(&data.html_url, &pull.html_url);
            assert_eq!(&data.number, &pull.number);
            assert_eq!(&data.state, &pull.state);
            assert_eq!(&data.title, &pull.title);
            assert_eq!(&data.user.login, &pull.user.login);
            assert_eq!(&data.user.html_url, &pull.user.html_url);
            assert_eq!(&data.created_at, &pull.created_at);
        }

        mock.assert_async().await;
    }

    #[rstest]
    #[case(
        Reviewers {
            users: vec![
                User {
                    login: "test".to_string(),
                    html_url: "https://github.com/reo0306".to_string(),
                },
            ],
        },
        "/repos/reo0306/gospo/pulls/1/requested_reviewers"
    )]
    #[tokio::test]
    async fn it_reviewers(
        github_api: GithubApi,
        #[case] response: Reviewers,
        #[case] path: &str
    ) {
        let server = mockito::Server::new_async().await;

        let body = serde_json::to_string(&response).unwrap();
        let (server, mock) = mock_server(server, path, body).await;

        let reviewers = github_api.find::<Reviewers>(&format!("{}{}", server.url(), path)).await.unwrap();

        for reviewer in &reviewers.users {
            assert_eq!("test".to_string(), reviewer.login);
        }

        mock.assert_async().await;
    }

    #[rstest]
    #[case(
        Review {
            user: User {
                login: "test".to_string(),
                html_url: "https://github.com/reo0306".to_string(),
            },
            state: "APPROVED".to_string(),
        },
        "/repos/reo0306/gospo/pulls/1/reviews"
    )]
    #[tokio::test]
    async fn it_reviews(
        github_api: GithubApi,
        #[case] data: Review,
        #[case] path: &str
    ) {
        let server = mockito::Server::new_async().await;

        let response = vec![&data];
        let body = serde_json::to_string(&response).unwrap();
        let (server, mock) = mock_server(server, path, body).await;

        let reviews = github_api.fetch::<Review>(&format!("{}{}", server.url(), path)).await.unwrap();

        for review in reviews {
            assert_eq!(&data.user.login, &review.user.login);
            assert_eq!(&data.state, &review.state);
        }

        mock.assert_async().await;
    }

    async fn mock_server(mut server: mockito::ServerGuard, path: &str, body: String) -> (mockito::ServerGuard, mockito::Mock) {
        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("User-Agent", "request")
            .with_header("Accept", "application/vnd.github.v3+json")
            .with_header("Authorization", "token aaaaaaaa")
            .with_header("X-GitHub-Api-Version", "2022-11-28")
            .with_body(body)
            .create_async()
            .await;

       (server, mock) 
    }
}