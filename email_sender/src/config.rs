use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp_port: u16,
    pub server_url: String,
    pub email_user: String,
    pub email_password: String,
}

impl Config {
    pub fn build() -> Self {
        envy::from_env::<Config>().unwrap()
    }
}
