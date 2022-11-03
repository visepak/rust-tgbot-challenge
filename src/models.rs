use serde::{Deserialize, Serialize};
use serde_json::Value;

/// https://core.telegram.org/bots/api#user
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UserGetMe {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: bool,
    pub can_read_all_group_messages: bool,
    pub supports_inline_queries: bool,
}

/// https://core.telegram.org/bots/api#chat
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Chat {
    pub id: i64,
}

/// https://core.telegram.org/bots/api#message
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
    pub chat: Chat,
}

/// https://core.telegram.org/bots/api#callbackquery
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CallbackQuery {
    pub id: String,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub data: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum NaviType {
    History,
    Back,
    Forward,
    Close,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum UpdateType {
    Save,
    Navi(NaviType)
}

/// https://core.telegram.org/bots/api#update
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
    pub update_type: Option<UpdateType>,
}

/// https://core.telegram.org/bots/api#making-requests
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Wrapper<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RedisData {
    pub history: Vec<Message>,
    pub view_idx: usize,
    pub chat_id: i64,
}

#[derive(Debug)]
pub struct ReplyMsg {
    pub chat_id: i64,
    pub text: Option<String>,
    pub msg_type: String,
    pub message_id: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    // pub from_handler: String,
    // pub response_type: String,
    // pub processed: bool,
    // /// if editing existing message
}

// pub enum MsgType {
//     editMessageText,
//     sendMessage,
// }

#[derive(Debug, Serialize)]
pub struct ReplyMsgBody {
    pub chat_id: i64,
    pub text: Option<String>,
    pub message_id: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: Option<String>,
}
