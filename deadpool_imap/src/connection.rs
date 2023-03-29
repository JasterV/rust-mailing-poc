use super::session::SessionWrapper;
use anyhow::Result;
use async_native_tls::TlsConnector;

pub struct Credentials {
    pub user: String,
    pub password: String,
}

pub struct ConnectionConfig {
    pub credentials: Credentials,
    pub domain: String,
    pub port: u16,
}

pub(crate) async fn connect(
    config: &ConnectionConfig,
    tls: TlsConnector,
) -> Result<SessionWrapper> {
    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = async_imap::connect(
        (config.domain.clone(), config.port),
        config.domain.clone(),
        tls,
    )
    .await?;

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let session = client
        .login(
            config.credentials.user.clone(),
            config.credentials.password.clone(),
        )
        .await
        .map_err(|(err, _)| err)?;

    Ok(SessionWrapper::new(session))
}
