use serde::Deserialize;

use crate::tg_service::TgClientConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub redis_url: String,
    pub tg: TgClientConfig,
}

impl Config {
    pub fn new() -> Config {
        let mut s = config::Config::new();

        if std::path::Path::new("Settings.toml").exists() {
            s.merge(config::File::with_name("./Settings.toml")).unwrap();
        } else {
            panic!("No config file found");
        }

        let conf = s
            .try_into::<Config>()
            .unwrap_or_else(|e| panic!("Error parsing config: {}", e));

        conf
    }
}
