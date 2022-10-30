use std::ops::DerefMut;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use mobc::Connection;
use mobc::Pool;
use mobc_redis::RedisConnectionManager;
use serde_json::json;

use crate::models::UserGetMe;

const REDIS_TIMEOUT: Duration = Duration::from_secs(2);

fn get_me_key(bot_id: i64) -> String {
    ["GET_ME", &bot_id.to_string()].join("_")
}

pub struct RedisService {
    pool: Pool<RedisConnectionManager>,
}

impl RedisService {
    pub fn new(redis_url: String) -> Self {
        let client = redis::Client::open(redis_url.as_str()).expect("Can not create redis client");
        let redis_manager = RedisConnectionManager::new(client);
        let pool = Pool::builder().max_open(3).max_idle(1).build(redis_manager);
        Self { pool }
    }

    async fn get_connection(&self) -> Result<Connection<RedisConnectionManager>> {
        let conn = self
            .pool
            .get_timeout(REDIS_TIMEOUT)
            .await
            .map_err(|e| anyhow!("cannot get redis connection from pool: {:#?}", e))?;
        Ok(conn)
    }

    pub async fn get_me(&self, bot_id: i64) -> Result<Option<UserGetMe>> {
        let mut con = self.get_connection().await?;
        redis::cmd("GET")
            .arg(get_me_key(bot_id))
            .query_async::<_, Option<String>>(&mut *con.deref_mut())
            .await
            .map_err(|e| anyhow!("cannot get data from redis: {}", e))?
            .map(|x| {
                serde_json::from_str::<UserGetMe>(&x)
                    .map_err(|e| anyhow!("cannot parse json {}: {:#?}", x, e))
            })
            .map_or(Ok(None), |v| Ok(Some(v?)))
    }

    pub async fn set_me(&self, bot_id: i64, data: &UserGetMe) -> Result<()> {
        let data = json!(data).to_string();
        // demo purpose
        let ttl_seconds = 5;
        // let ttl_seconds = 24 * 60 * 60;
        let key = get_me_key(bot_id);
        let mut con = self.get_connection().await?;
        redis::pipe()
            .set(&key, &data)
            .expire(key, ttl_seconds)
            .query_async(&mut *con.deref_mut())
            .await
            .map_err(|e| {
                anyhow!(
                    "cannot save bot's {} model {} to redis: {:#?}",
                    bot_id,
                    &data,
                    e
                )
            })?;
        Ok(())
    }
}
