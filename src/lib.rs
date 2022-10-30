use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;

use crate::redis_service::RedisService;
use crate::settings::Config;
use crate::tg_service::TgClient;

mod models;
mod redis_service;
mod settings;
mod tg_service;

pub async fn start_server() {
    let config = Config::new();

    let tg_client = Arc::new(TgClient::new(config.tg.clone()));
    let redis_service = Arc::new(RedisService::new(config.redis_url.clone()));

    // start a separate thread of infinite lifetime - for every independent task

    tokio::spawn({
        let tg_client = tg_client.clone();
        let redis_service = redis_service.clone();
        async move { process_updates(&tg_client, &redis_service).await }
    });

    // random shit for demonstration purpose below

    tokio::spawn({
        let tg_client = tg_client.clone();
        let redis_service = redis_service.clone();
        async move {
            loop {
                if let Err(e) = set_me(config.tg.bot_id, &tg_client, &redis_service).await {
                    eprintln!("falied on set_me: {:#?}", e)
                }
                // demo purpose: not to spam console
                tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
            }
        }
    });

    tokio::spawn({
        let redis_service = redis_service.clone();
        async move {
            loop {
                got_me_from_redis(config.tg.bot_id, &redis_service).await;
                // demo purpose: not to spam console
                tokio::time::sleep(tokio::time::Duration::from_secs(9)).await;
            }
        }
    });
}

async fn process_updates(tg_client: &TgClient, _redis_service: &RedisService) {
    while let Some(upd) = tg_client.get_updates().await.next().await {
        println!("got update {:#?}", upd);
        // demo purpose: not to spam console
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    }
}

// error propagated
async fn set_me(bot_id: i64, tg_client: &TgClient, redis_service: &RedisService) -> Result<()> {
    let data = tg_client.get_me().await?;
    redis_service.set_me(bot_id, &data).await?;
    println!("had set me");
    Ok(())
}

// error log hidden
async fn got_me_from_redis(bot_id: i64, redis_service: &RedisService) -> () {
    if let Some(data) = redis_service
        .get_me(bot_id)
        .await
        .map_err(|e| eprintln!("failed to get me {:#?}", e))
        .ok()
        .flatten()
    {
        println!("got_me {:#?}", data)
    } else {
        println!("got_me nothing");
    };
}
