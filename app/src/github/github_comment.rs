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
use llmchain::DatabendLLM;
use llmchain::DocumentLoader;
use llmchain::DocumentPath;
use llmchain::DocumentSplitter;
use llmchain::GithubPRDiffSplitter;
use llmchain::GithubPRLoader;
use llmchain::GithubPRSummary;
use llmchain::Summarize;
use log::error;
use log::info;
use octocrab::params::State;
use octocrab::Octocrab;
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
        let keywords = "askbend:summary";
        let conf = self.conf.clone();
        tokio::spawn(async move {
            let mut now = Utc::now();
            let mut scan_map: HashMap<String, DateTime<Utc>> = HashMap::new();
            loop {
                if let Some(repos) = &conf.github.repos {
                    info!("scan repos: {:?}", repos);

                    for repo in repos.clone() {
                        let cloned_conf = conf.clone();
                        let cloned_repo = repo.clone();
                        let cloned_keywords = keywords.to_string().clone();
                        let task_now = now;
                        let map_clone = scan_map.clone();
                        let task = tokio::spawn(async move {
                            info!("Scan repo: {} at {}", cloned_repo, task_now);

                            let (owner, repo_name) = match Self::parse_github_repo(&cloned_repo) {
                                Ok(val) => val,
                                Err(e) => {
                                    error!("Failed to parse github repo: {:?}", e);
                                    return;
                                }
                            };

                            let pull_requests = match Self::get_octo(&cloned_conf)
                                .pulls(&owner, &repo_name)
                                .list()
                                .page(1u32)
                                .per_page(100)
                                .state(State::Open)
                                .send()
                                .await
                            {
                                Ok(val) => val,
                                Err(e) => {
                                    error!("Failed to get pull requests: {:?}", e);
                                    return;
                                }
                            };

                            let since = *map_clone.get(&repo_name).unwrap_or(&task_now);

                            for pr in pull_requests {
                                info!(
                                    "Pr html_url:{:?}, title: {:?}, create_at:{:?}",
                                    pr.html_url, pr.title, pr.created_at
                                );
                                let pr_comments = match Self::get_octo(&cloned_conf)
                                    .issues(&owner, &repo_name)
                                    .list_comments(pr.number)
                                    .page(1u32)
                                    .per_page(100)
                                    .since(since)
                                    .send()
                                    .await
                                {
                                    Ok(val) => val,
                                    Err(e) => {
                                        error!("Failed to list comments: {:?}", e);
                                        return;
                                    }
                                };

                                let mut comments = pr_comments.items.clone();
                                comments.sort_by(|a, b| b.created_at.cmp(&a.created_at));

                                for comment in comments {
                                    info!(
                                        "Pr number:{}, Comment ID: {}, Body: {:?}, create_at:{:?}, url:{:?}",
                                        pr.number,
                                        comment.id,
                                        comment.body,
                                        comment.created_at,
                                        comment.issue_url
                                    );

                                    if comment.body == Some(cloned_keywords.clone()) {
                                        match Self::get_octo(&cloned_conf)
                                            .issues(&owner, &repo_name)
                                            .create_comment_reaction(
                                                comment.id,
                                                octocrab::models::reactions::ReactionContent::PlusOne,
                                            )
                                            .await
                                        {
                                            Ok(_) => {},
                                            Err(e) => {
                                                error!("Failed to create comment reaction: {:?}", e);
                                                return;
                                            }
                                        };

                                        match Self::get_summary(
                                            &cloned_conf,
                                            &owner,
                                            &repo_name,
                                            pr.number,
                                        )
                                        .await
                                        {
                                            Ok(summary) => {
                                                let final_summary = format!(
                                                    "## PR Summary(By [llmchain.rs](https://github.com/shafishlabs/llmchain.rs)):\n{}",
                                                    summary
                                                );

                                                match Self::get_octo(&cloned_conf)
                                                    .issues(&owner, &repo_name)
                                                    .create_comment(pr.number, final_summary)
                                                    .await
                                                {
                                                    Ok(_) => {}
                                                    Err(e) => {
                                                        error!("Failed to create comment: {:?}", e);
                                                        return;
                                                    }
                                                };
                                            }
                                            Err(e) => {
                                                error!("Failed to get summary: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        });

                        if let Err(e) = task.await {
                            error!("Task panicked with error: {:?}", e);
                        }

                        now = Utc::now();
                        scan_map.insert(repo.clone(), now);
                    }
                }
                sleep(Duration::from_secs(conf.github.check_in_secs as u64)).await;
            }
        });
    }

    fn get_octo(conf: &Config) -> Octocrab {
        Octocrab::builder()
            .personal_token(conf.github.github_token.clone())
            .build()
            .unwrap()
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

    async fn get_summary(conf: &Config, owner: &str, repo: &str, pull_id: u64) -> Result<String> {
        info!("get summary for {}/{}#{}", owner, repo, pull_id);
        let github_token = conf.github.github_token.clone();
        let databend_dsn = conf.github.databend_dsn.clone();

        let documents = GithubPRLoader::create(owner, repo, &github_token)
            .load(DocumentPath::from_list(vec![pull_id as usize]))
            .await?;

        let documents = GithubPRDiffSplitter::create()
            .with_chunk_size(8000)
            .split_documents(&documents)
            .unwrap();

        if documents.tokens() > conf.github.llm_max_tokens {
            return Ok(format!(
                "The PR is too large to summarize, tokens: {}, max tokens: {}",
                documents.tokens(),
                conf.github.llm_max_tokens
            ));
        }

        let databend_llm = DatabendLLM::create(&databend_dsn);
        let summary = GithubPRSummary::create(databend_llm);
        summary.add_documents(&documents).await?;
        let pr_summary = summary.final_summary().await?;
        info!("Tokens: {}, Summary: {}", summary.tokens(), pr_summary);

        Ok(pr_summary)
    }
}
