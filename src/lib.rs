use std::sync::Arc;

// use anyhow::Result;
// use futures_util::StreamExt;
use crate::bot_navi::BotNavi;
use crate::models::Update;
use crate::bot_save::BotSave;

use crate::bot_update::BotUpdate;
use crate::redis_service::RedisService;
use crate::settings::Config;
use crate::tg_service::TgClient;

mod models;
mod redis_service;
mod settings;
mod tg_service;
mod bot_update;
mod bot_save;
mod bot_navi;

pub async fn start_server() {
    let config = Config::new();

    let tg_client = Arc::new(TgClient::new(config.tg.clone()));
    let redis_service = Arc::new(RedisService::new(config.redis_url.clone()));

    let (tx_save, rx_save) = tokio::sync::mpsc::channel::<Update>(200);
    let (tx_navi, rx_navi) = tokio::sync::mpsc::channel::<Update>(200);

    // start a separate thread of infinite lifetime - for every independent task

    // thread for get updates and send to save/navi channels
    tokio::spawn({
        let tg_client = tg_client.clone();
        let mut bot_update = BotUpdate::new(tg_client, tx_save, tx_navi);
        async move { bot_update.get_updates().await }
    });

    // thread for save mesages to redis
    tokio::spawn({
        let redis_service = redis_service.clone();
        let mut bot_save = BotSave::new(redis_service, rx_save);
        async move { bot_save.save_msg().await }
    });

    // thread for open history and navi
    tokio::spawn({
        let tg_client = tg_client.clone();
        let redis_service = redis_service.clone();
        let mut bot_navi = BotNavi::new(tg_client, redis_service, rx_navi);
        async move { bot_navi.navi_msg().await }
    });

    // random shit for demonstration purpose below

    // tokio::spawn({
    //     let tg_client = tg_client.clone();
    //     let redis_service = redis_service.clone();
    //     async move {
    //         loop {
    //             if let Err(e) = set_me(config.tg.bot_id, &tg_client, &redis_service).await {
    //                 eprintln!("falied on set_me: {:#?}", e)
    //             }
    //             // demo purpose: not to spam console
    //             tokio::time::sleep(tokio::time::Duration::from_secs(7)).await;
    //         }
    //     }
    // });

    // tokio::spawn({
    //     let redis_service = redis_service.clone();
    //     async move {
    //         loop {
    //             got_me_from_redis(config.tg.bot_id, &redis_service).await;
    //             // demo purpose: not to spam console
    //             tokio::time::sleep(tokio::time::Duration::from_secs(9)).await;
    //         }
    //     }
    // });
}

// async fn get_offset()-> i64 {
//     let offset = match _redis_service.get::<Offset>("offset").await {
//         Ok(x) => x.unwrap_or(Offset { update_id: 0}).update_id,
//         Err(e) => 0,
//     };
//     println!("got offset {}", offset);
//     offset
// }

// async fn set_offset(offset: i64)-> () {
//
//     let offset: Offset = Offset {
//         update_id: 3
//     };
//     match redis_service().set::<Offset>("offset",&offset).await {
//         Ok(x) => {
//             println!("{:#?}", x);
//             assert!(true);
//         }
//         Err(e) => {
//             eprintln!("{:#?}", e);
//             assert!(false);
//         }
//     };
//
//     let offset = match _redis_service.get::<Offset>("offset").await {
//         Ok(x) => x.unwrap_or(Offset { update_id: 0}).update_id,
//         Err(e) => 0,
//     };
//     println!("got offset {}", offset);
//     offset
// }

// error propagated
// async fn set_me(bot_id: i64, tg_client: &TgClient, redis_service: &RedisService) -> Result<()> {
//     let data = tg_client.get_me().await?;
//     redis_service.set_me(bot_id, &data).await?;
//     println!("had set me");
//     Ok(())
// }

// error log hidden
// async fn got_me_from_redis(bot_id: i64, redis_service: &RedisService) -> () {
//     if let Some(data) = redis_service
//         .get_me(bot_id)
//         .await
//         .map_err(|e| eprintln!("failed to get me {:#?}", e))
//         .ok()
//         .flatten()
//     {
//         println!("got_me {:#?}", data)
//     } else {
//         println!("got_me nothing");
//     };
// }
