use config::{Config, File};
use std::convert::{Infallible, TryFrom};
use std::env;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub user: String,
    pub dbname: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(rename = "host")]
    pub host: [u8; 4],
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self, Infallible> {
        let cwd: PathBuf = env::current_dir().expect("Failed to get current directory");
        let config_path: PathBuf = cwd.join("config.toml");
        let settings: Config = Config::builder()
            .add_source(File::from(config_path))
            .build().unwrap();
        settings.try_into()
    }

    pub fn host_ip(&self) -> std::net::IpAddr {
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(self.server.host[0], self.server.host[1], self.server.host[2], self.server.host[3]))
    }
}

impl TryFrom<Config> for AppConfig {
    type Error = Infallible;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let app_config: AppConfig = config.try_deserialize().unwrap();
        Ok(app_config)
    }
}
