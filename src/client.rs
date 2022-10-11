use std::sync::Arc;

use futures::lock::Mutex;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};

pub struct Client;

const API_URL: &str = "https://hacker-news.firebaseio.com/v0/";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Story {
    pub text: Option<String>,
    pub by: String,
    pub descendants: Option<u64>,
    pub id: u64,
    pub kids: Option<Vec<u64>>,
    pub score: i64,
    pub time: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub story_type: String,
    pub url: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_story_by_id(&self, id: u64) -> Result<Story, reqwest::Error> {
        let url = format!("{}/item/{}.json", API_URL, id);
        reqwest::get(url).await?.json::<Story>().await
    }

    pub async fn get_stop_stories(&self) -> Result<Vec<Story>, reqwest::Error> {
        let url = format!("{}/{}", API_URL, "topstories.json");
        let stories_ids = reqwest::get(url).await?.json::<Vec<u64>>().await?;

        // Only fetch the first 8 top news for now
        let stories_ids = &stories_ids[0..8];

        let stories = Arc::new(Mutex::new(Vec::new()));

        {
            let stories = stories.clone();
            stream::iter(stories_ids)
                .for_each_concurrent(8, move |story_id| {
                    let stories = stories.clone();
                    async move {
                        stories
                            .lock()
                            .await
                            .push(self.get_story_by_id(*story_id).await.unwrap())
                    }
                })
                .await;
        }

        let stories = stories.lock().await;

        Ok(stories.to_vec())
    }
}
