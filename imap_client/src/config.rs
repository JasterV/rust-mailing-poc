use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(alias = "imaps_server_port")]
    pub server_port: u16,
    pub server_url: String,
    #[serde(alias = "imap_user")]
    pub user: String,
    #[serde(alias = "imap_password")]
    pub password: String,
    #[serde(alias = "imap_client_port")]
    pub port: u16,
}

impl Config {
    pub fn build() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}
