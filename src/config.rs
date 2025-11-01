use base64::Engine;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use std::cell::LazyCell;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RedisConfig {
    pub url: String,
    #[serde(default = "RedisConfig::default_max_connections")]
    pub max_connections: u32,
}

impl RedisConfig {
    fn default_max_connections() -> u32 {
        6
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub url: String,
    #[serde(default = "DatabaseConfig::default_max_connections")]
    pub max_connections: u32,
}

impl DatabaseConfig {
    fn default_max_connections() -> u32 {
        6
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    pub(crate) redis: RedisConfig,
    pub(crate) database: DatabaseConfig,
    /// Host and port to listen on. Defaults to `0.0.0.0:3000`
    #[serde(default = "AppConfig::default_app_host")]
    pub(crate) app_host: String,
    /// Signup token. Regenerated on startup unless set.
    /// Use a config file or the `SIGNUP_TOKEN` env variable.
    #[serde(default = "AppConfig::default_signup_token")]
    pub(crate) signup_token: String,
}

impl AppConfig {
    fn default_app_host() -> String {
        "0.0.0.0:3000".to_string()
    }
    fn default_signup_token() -> String {
        let mut token = [0; 32];
        let mut rng = rand::prelude::StdRng::from_os_rng();
        rng.fill_bytes(&mut token);
        BASE64_STANDARD_NO_PAD.encode(&token)
    }
}

pub const CONFIG: LazyCell<AppConfig> = LazyCell::new(|| {
    let config = config::Config::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::default().separator("_"))
        .build()
        .expect("Failed to build config");

    config
        .try_deserialize::<AppConfig>()
        .expect("Failed to deserialize config")
});
