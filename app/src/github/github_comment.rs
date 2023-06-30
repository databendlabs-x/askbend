// Copyright 2023 Databend Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use log::info;
use octocrab::params::State;
use octocrab::Octocrab;
use tokio::spawn;
use tokio::time::sleep;
use url::Url;

use crate::Config;

pub struct GithubComment {
    conf: Config,
}

impl GithubComment {
    pub fn create(conf: &Config) -> Self {
        GithubComment { conf: conf.clone() }
    }

    pub fn start(&self) {
        let conf = self.conf.clone();
        spawn(async move {
            let octo = octocrab::initialise(
                Octocrab::builder()
                    .personal_token(conf.github.github_token)
                    .build()
                    .unwrap(),
            );

            let mut scan_map: HashMap<String, DateTime<Utc>> = HashMap::new();
            loop {
                if let Some(repos) = &conf.github.repos {
                    info!("scan repos: {:?}", repos);

                    for repo in repos {
                        let now = chrono::Utc::now();
                        info!("Scan repo:{} at {}", repo, now);
                        let (owner, repo) = Self::parse_github_repo(repo).unwrap();
                        let pull_requests = octo
                            .pulls(&owner, &repo)
                            .list()
                            .page(1u32)
                            .per_page(100)
                            .state(State::Open)
                            .send()
                            .await
                            .unwrap();

                        let since = scan_map.get(&repo).unwrap_or(&now);
                        for pr in pull_requests {
                            let pr_comments = octo
                                .issues(&owner, &repo)
                                .list_comments(pr.number)
                                .page(1u32)
                                .per_page(100)
                                .since(*since)
                                .send()
                                .await
                                .unwrap();

                            for comment in pr_comments.items {
                                println!(
                                    "Pr number:{}, Comment ID: {}, Body: {:?}, time:{:?}",
                                    pr.number, comment.id, comment.body, comment.updated_at
                                );
                            }
                        }
                        scan_map.insert(repo.clone(), now);
                    }
                }

                sleep(Duration::from_secs(20)).await;
            }
        });
    }

    fn parse_github_repo(url: &str) -> Result<(String, String)> {
        let parsed_url = Url::parse(url)?;

        let mut segments = parsed_url.path_segments().expect("path segments");

        let owner = segments.next().expect("owner").to_string();
        info!("owner: {}", owner);
        let repo = segments.next().expect("repo").to_string();
        info!("repo: {}", repo);

        Ok((owner, repo))
    }
}
