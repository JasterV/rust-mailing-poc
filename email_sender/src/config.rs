use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub smtp_port: u16,
    pub server_url: String,
    pub email_sender_user: String,
    pub email_sender_password: String,
    pub email_receiver_user: String,
    pub sender_port: u16,
}

impl Config {
    pub fn build() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}
