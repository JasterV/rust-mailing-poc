use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub imaps_port: u16,
    pub server_url: String,
    pub email_receiver_user: String,
    pub email_receiver_password: String,
    pub receiver_port: u16,
}

impl Config {
    pub fn build() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}
