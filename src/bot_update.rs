use crate::models::{Update, UpdateType};
use crate::tg_service::TgClient;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use futures_util::StreamExt;
use crate::models::NaviType::{Back, Close, Forward, History};

pub struct BotUpdate {
    offset: i64,
    tg_client: Arc<TgClient>,
    tx_save: Sender<Update>,
    tx_navi: Sender<Update>,
}

impl BotUpdate {
    pub fn new(tg_client: Arc<TgClient>,  tx_save: Sender<Update>, tx_navi: Sender<Update> ) -> Self {
        Self {
            offset: 0,
            tg_client,
            tx_save,
            tx_navi,
        }
    }

    pub async fn get_updates(&mut self) {
        while let Some(upd) = self.tg_client
            .get_updates(self.offset)
            .await
            .next()
            .await
        {
            println!("Got updates {:?}", upd);
            match upd {
                Ok(upd) => {
                    self.offset=upd.update_id + 1;
                    let update_type = BotUpdate::get_update_type(&upd);
                    match &update_type {
                        Some(UpdateType::Save) => {
                            println!("Send update for save {:#?}", upd);
                            if let Err(err) = self.tx_save.send(Update {update_type, ..upd}).await {
                                eprintln!("Error send to save channel! {:?}", err);
                            }
                        }
                        Some(UpdateType::Navi(..)) => {
                            println!("Send update for navi {:#?}", upd);
                            if let Err(err) = self.tx_navi.send(Update {update_type, ..upd}).await {
                                eprintln!("Error send to navi channel! {:?}", err);
                            }
                        }
                        _ =>  eprintln!("Error matching update type!")
                    }
                }
                Err(err) => eprintln!("Error get_updates: {:?}", err),
            }
            // Pause to prevent rate-limit
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    fn get_update_type(upd: &Update) -> Option<UpdateType> {
        // Check message field (only for new message and /history)
        if let Some(msg) = &upd.message {
            if let Some(txt) = &msg.text {
                return if txt == "/history" {
                    println!("Incoming UpdateType::History");
                    Some(UpdateType::Navi(History))
                } else {
                    println!("Incoming UpdateType::Text");
                    Some(UpdateType::Save)
                };
            }
        } else {
            if let Some(cbq) = &upd.callback_query {
                return match cbq.data.as_ref().map(String::as_ref) {
                    Some("/back") => {
                        println!("Incoming UpdateType::Back");
                        Some(UpdateType::Navi(Back))
                    }
                    Some("/forward") => {
                        println!("Incoming UpdateType::Forward");
                        Some(UpdateType::Navi(Forward))
                    }
                    Some("/close") => {
                        println!("Incoming UpdateType::Close");
                        Some(UpdateType::Navi(Close))
                    }
                    _ => {
                        eprintln!("Incoming Unknown type");
                        None
                    }
                };
            }
        }
        println!("Update without message!");
        None
    }
}
