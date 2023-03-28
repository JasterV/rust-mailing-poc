use anyhow::Result;
use async_imap::{
    types::{Fetch, Flag},
    Session,
};
use async_native_tls::TlsStream;
use futures::TryStreamExt;
use serde::Serialize;
use std::str::Utf8Error;
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct SessionWrapper {
    session: Session<TlsStream<TcpStream>>,
}

#[derive(Serialize, Debug)]
pub struct Message {
    pub body: String,
    pub uid: u32,
    pub size: u32,
    pub flags: Vec<String>,
}

impl SessionWrapper {
    pub(crate) fn new(session: Session<TlsStream<TcpStream>>) -> Self {
        Self { session }
    }

    pub async fn clear(&mut self) -> Result<()> {
        self.session.examine("INBOX").await?;
        self.session.close().await?;
        Ok(())
    }

    pub async fn fetch_inbox(&mut self) -> Result<Vec<Message>> {
        // we want to fetch the first email in the INBOX mailbox
        self.session.examine("INBOX").await?;

        let query = "(RFC822 UID RFC822.SIZE RFC822.TEXT)";

        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let messages_stream = self.session.fetch("1:*", query).await?;

        let messages = messages_stream.try_collect::<Vec<Fetch>>().await?;

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

#[derive(Error, Debug)]
pub enum ImapError {
    #[error("Error: {0}")]
    InnerImapError(#[from] async_imap::error::Error),
    #[error("Connection not established")]
    NonExistingSession,
}
