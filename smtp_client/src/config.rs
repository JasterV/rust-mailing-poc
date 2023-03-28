use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(alias = "smtp_server_port")]
    pub server_port: u16,
    pub server_url: String,
    #[serde(alias = "smtp_user")]
    pub user: String,
    #[serde(alias = "smtp_password")]
    pub password: String,
    #[serde(alias = "imap_user")]
    pub recipient_user: String,
    #[serde(alias = "smtp_client_port")]
    pub port: u16,
}

impl Config {
    pub fn build() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}
