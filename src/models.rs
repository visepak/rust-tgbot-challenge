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

/// https://core.telegram.org/bots/api#message
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
}

/// https://core.telegram.org/bots/api#update
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Update {
    pub update_id: i64,
    pub message: Message,
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
