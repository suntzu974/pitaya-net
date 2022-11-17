pub use ::config::ConfigError;
use serde::Deserialize;
use dotenv::dotenv;
#[derive(Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        let mut cfg = ::config::Config::new();
        cfg.merge(config::Environment::new().separator("_"))?;
        cfg.try_into()
    }
}
