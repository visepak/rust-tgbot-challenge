use std::time::Duration;

use anyhow::{anyhow, Result};
use async_stream::try_stream;
use futures_core::stream::Stream;
use futures_util::stream;
use futures_util::TryStreamExt;
use serde::Deserialize;

use crate::models::{Update, UserGetMe, Wrapper};

const CONSUMER_INTERVAL: u64 = 2;

#[derive(Debug, Deserialize, Clone)]
pub struct TgClientConfig {
    pub api_url: String,
    pub bot_id: i64,
    pub bot_secret: String,
}

pub struct TgClient {
    url: String,
    client: reqwest::Client,
}

impl TgClient {
    pub fn new(config: TgClientConfig) -> Self {
        let url = format!(
            "{}/bot{}:{}/",
            config.api_url, config.bot_id, config.bot_secret
        );
        let client = reqwest::Client::builder()
            .build()
            .expect("failed to create http client");
        Self { url, client }
    }

    pub async fn get_me(&self) -> Result<UserGetMe> {
        // We have to re-wrap Result so that Error types matches.
        // "anyhow" crate automatically wraps errors, propagated by "?", into anyhow::Error.
        self.client
            .get(format!("{}getMe", &self.url))
            .send()
            .await?
            .json::<Wrapper<UserGetMe>>()
            .await? // unwraps std::Result<_, reqwest::Error>
            .result
            .ok_or(anyhow!("no result, there is an an error somewhere")) // wraps into std::Result<_, anyhow::Error>
    }

    // Unfortunately, no easy way to store such a stream in a struct's field.
    // This is not necessary anyway.
    // But its ok to assign to a variable without hand-written type (i.e. let it be auto-derived)
    // and leave OOP behind.
    pub async fn get_updates(&self) -> impl Stream<Item = Result<Update>> + '_ {
        let url = format!("{}getUpdates?timeout={}", self.url, CONSUMER_INTERVAL);
        let consumer = &self.client;
        // we need to find a way to reconnect during long polling
        let a = match consumer
            .get(url.clone())
            .timeout(Duration::from_secs(CONSUMER_INTERVAL))
            .send()
            .await
        {
            Ok(rs) if rs.status().is_success() => {
                let val = rs
                    .json::<Wrapper<Vec<Update>>>()
                    .await
                    .map(|w|
                        // bad unwrap - we loose debug info
                        w.result.unwrap_or(Vec::new()))
                    .map(|v| v.into_iter().map(|u| Ok(u)).collect::<Vec<_>>())
                    .map_err(anyhow::Error::from)
                    .unwrap_or_else(|e| vec![Err(e)]);
                val
            }
            bad_res => {
                vec![Err(anyhow!(
                    "reconnect after failure on polling tg updates: {:#?}",
                    bad_res // have to extract valuable debug info, e.g. on wrong secret
                ))]
            }
        };
        stream::iter(a)
    }
}

#[cfg(test)]
mod test {
    use crate::Config;
    use futures_util::stream::StreamExt;

    use super::*;

    fn tg_client() -> TgClient {
        TgClient::new(Config::new().tg)
    }

    #[test]
    fn synchronous_test_example() {
        tg_client();
        assert!(true);
    }

    #[tokio::test]
    async fn async_test_example() {
        match tg_client().get_me().await {
            Ok(x) => {
                println!("{:#?}", x);
                assert!(true);
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                assert!(false);
            }
        };
    }

    #[tokio::test]
    async fn get_first_update() {
        let tg_client = tg_client();
        let mut stream = tg_client.get_updates().await;

        // if you wrap this in loop as it is, you will get no results into console.
        // you'll have to break surrounding loop.
        match stream.next().await {
            Some(Ok(upd)) => println!("{:#?}", upd),
            _ => (), // ???
        }

        assert!(true);
    }
}
