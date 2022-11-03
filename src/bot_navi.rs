use crate::models::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMsg, Update, UpdateType};
use crate::redis_service::RedisService;
use crate::tg_service::TgClient;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver};
use crate::models::NaviType::{Back, Close, Forward, History};


pub struct BotNavi {
    tg_client: Arc<TgClient>,
    redis_service: Arc<RedisService>,
    rx_navi: Receiver<Update>
}

impl BotNavi {
    pub fn new(tg_client: Arc<TgClient>, redis_service: Arc<RedisService>, rx_navi: Receiver<Update> ) -> Self {
        Self {
            tg_client,
            redis_service,
            rx_navi
        }
    }

    pub async fn navi_msg(&mut self) -> () {
        loop {
            if let Some(upd) = self.rx_navi.recv().await {
                match &upd.update_type {
                    Some(UpdateType::Navi(History)) => self.send_history(&upd).await,
                    Some(UpdateType::Navi(Back)) => self.send_back(&upd).await,
                    Some(UpdateType::Navi(Forward)) => self.send_forward(&upd).await,
                    Some(UpdateType::Navi(Close)) => self.send_close(&upd).await,
                    _ => eprintln!("Error matching navi update!")
                }
            } else {
                println!("Error receive update navi from channel!");
            }
            // Pause to prevent rate-limit
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }

    }

    fn get_reply_markup() -> InlineKeyboardMarkup {
        InlineKeyboardMarkup {
            inline_keyboard: vec![vec![
                InlineKeyboardButton {
                    text: "<<<".to_string(),
                    callback_data: Some("/back".to_string()),
                },
                InlineKeyboardButton {
                    text: "close".to_string(),
                    callback_data: Some("/close".to_string()),
                },
                InlineKeyboardButton {
                    text: ">>>".to_string(),
                    callback_data: Some("/forward".to_string()),
                },
            ]],
        }
    }

    fn get_msg_id(upd: &Update) -> i64 {
        if let Some(cbq) = &upd.callback_query {
            if let Some(msg) = &cbq.message {
                return msg.message_id;
            }
        }
        return 0;
    }

    async fn send_close(&self, upd: &Update) -> () {
        let reply_msg = ReplyMsg {
            chat_id: RedisService::get_chat_id(&upd),
            text: None,
            msg_type: "deleteMessage".to_string(),
            message_id: Some(BotNavi::get_msg_id(&upd)),
            reply_markup: None,
        };
        self.tg_client
            .send_msg(&reply_msg)
            .await
            .expect("Send_msg Error");
    }

    async fn send_back(&self, upd: &Update) -> () {
        let mut redis_data = self.redis_service.get_history(&upd).await;
        if redis_data.history.len() > 0 {
            if redis_data.view_idx > 1 {
                redis_data.view_idx -= 1;
                let back_message = redis_data.history[redis_data.view_idx].clone();
                let reply_msg = ReplyMsg {
                    chat_id: RedisService::get_chat_id(&upd),
                    text: Some(back_message.text.unwrap()),
                    msg_type: "editMessageText".to_string(),
                    message_id: Some(BotNavi::get_msg_id(&upd)),
                    reply_markup: Some(BotNavi::get_reply_markup()),
                };
                self.tg_client
                    .send_msg(&reply_msg)
                    .await
                    .expect("Send_msg Error");
            }
            self.redis_service.update_redis_data(&redis_data).await;
        }
    }

    async fn send_forward(&self, upd: &Update) -> () {
        let mut redis_data = self.redis_service.get_history(&upd).await;
        if redis_data.history.len() > 0 {
            if redis_data.view_idx < redis_data.history.len() - 1 {
                redis_data.view_idx += 1;
                let forward_message = redis_data.history[redis_data.view_idx].clone();
                let reply_msg = ReplyMsg {
                    chat_id: RedisService::get_chat_id(&upd),
                    text: Some(forward_message.text.unwrap()),
                    msg_type: "editMessageText".to_string(),
                    message_id: Some(BotNavi::get_msg_id(&upd)),
                    reply_markup: Some(BotNavi::get_reply_markup()),
                };
                self.tg_client
                    .send_msg(&reply_msg)
                    .await
                    .expect("Send_msg Error");
            }
            self.redis_service.update_redis_data(&redis_data).await;
        }
    }

    async fn send_history(&self, upd: &Update) -> () {
        let redis_data = self.redis_service.get_history(&upd).await;
        if redis_data.history.len() > 0 {
            let last_message = redis_data.history[redis_data.view_idx].clone();
            // println!("History message: {:?}", &last_message);
            let reply_msg = ReplyMsg {
                chat_id: upd.message.as_ref().unwrap().chat.id.clone(),
                text: Some(last_message.text.unwrap()),
                msg_type: "sendMessage".to_string(),
                message_id: None,
                reply_markup: Some(BotNavi::get_reply_markup()),
            };
            self.tg_client
                .send_msg(&reply_msg)
                .await
                .expect("Send_msg Error");
        }
    }

}
