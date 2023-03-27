use std::str::Utf8Error;

use async_imap::{
    types::{Fetch, Flag},
    Session,
};
use async_native_tls::{TlsConnector, TlsStream};
use futures::TryStreamExt;
use serde::Serialize;
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Client {
    address: ServerAddress,
    credentials: Credentials,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub body: String,
    pub uid: u32,
    pub size: u32,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

pub type ServerAddress = (String, u16);

impl Client {
    pub async fn new(server_address: ServerAddress, credentials: Credentials) -> Self {
        Client {
            address: server_address,
            credentials,
        }
    }

    async fn connect(&self) -> Session<TlsStream<TcpStream>> {
        let tls = TlsConnector::new().danger_accept_invalid_certs(true);

        // we pass in the domain twice to check that the server's TLS
        // certificate is valid for the domain we're connecting to.
        let client = async_imap::connect(
            (self.address.0.clone(), self.address.1),
            self.address.0.clone(),
            tls,
        )
        .await
        .expect("Can't connect to the server");

        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        client
            .login(
                self.credentials.user.clone(),
                self.credentials.password.clone(),
            )
            .await
            .expect("Can't log into the imap server")
    }

    async fn logout(mut session: Session<TlsStream<TcpStream>>) -> Result<(), String> {
        session.logout().await.map_err(|e| e.to_string())
    }

    pub async fn fetch_inbox(&mut self) -> Result<Vec<Message>, String> {
        // we want to fetch the first email in the INBOX mailbox
        let mut imap_session = self.connect().await;

        imap_session
            .examine("INBOX")
            .await
            .map_err(|e| e.to_string())?;

        let query = "(RFC822 UID RFC822.SIZE RFC822.TEXT)";

        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let messages_stream = imap_session
            .fetch("1:*", query)
            .await
            .map_err(|io_err| io_err.to_string())?;

        let messages = messages_stream
            .try_collect::<Vec<Fetch>>()
            .await
            .map_err(|io_err| io_err.to_string())?;

        let messages: Vec<Message> = messages
            .iter()
            .map(|m| {
                // extract the message's body
                let body: &[u8] = m.text().unwrap();
                let body: String = std::str::from_utf8(body)?.to_string();
                let uid = m.uid.unwrap();
                let size = m.size.unwrap();
                let flags = m.flags().into_iter().map(Self::flag_to_str).collect();
                let message = Message {
                    body,
                    size,
                    uid,
                    flags,
                };
                Ok(message)
            })
            .filter_map(|res: Result<Message, Utf8Error>| res.ok())
            .collect();

        Self::logout(imap_session).await?;

        Ok(messages)
    }

    fn flag_to_str<'a>(flag: Flag<'a>) -> String {
        match flag {
            Flag::Seen => "seen".into(),
            Flag::Answered => "answered".into(),
            Flag::Flagged => "flagged".into(),
            Flag::Deleted => "deleted".into(),
            Flag::Draft => "draft".into(),
            Flag::Recent => "recent".into(),
            Flag::MayCreate => "may_create".into(),
            Flag::Custom(custom) => custom.into(),
        }
    }
}
