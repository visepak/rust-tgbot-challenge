use crate::models::{ Update};
use crate::redis_service::RedisService;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver};

pub struct BotSave {
    redis_service: Arc<RedisService>,
    rx_save: Receiver<Update>
}

impl BotSave {
    pub fn new(redis_service: Arc<RedisService>, rx_save: Receiver<Update> ) -> Self {
        Self {
            redis_service,
             rx_save
        }
    }

    pub async fn save_msg(&mut self) -> () {
        loop {
            if let Some(upd) = self.rx_save.recv().await {
                let mut redis_data = self.redis_service.get_history(&upd).await;
                if let Some(msg) = upd.message.clone() {
                    redis_data.history.push(msg);
                    redis_data.view_idx = &redis_data.history.len() - 1;
                    self.redis_service.update_redis_data(&redis_data).await;
                }
            } else {
                println!("Error receive update save from channel!");
            }
        }
    }

}
