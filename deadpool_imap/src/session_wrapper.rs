use anyhow::Result;
use async_imap::types::Flag as InternalFlag;
use async_imap::{types::Fetch, Session};
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

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Seen,
    Deleted,
    MyCustomFlag,
}

impl From<Flag> for InternalFlag<'_> {
    fn from(value: Flag) -> Self {
        match value {
            Flag::Seen => InternalFlag::Seen,
            Flag::Deleted => InternalFlag::Deleted,
            Flag::MyCustomFlag => InternalFlag::Custom("MyCustomFlag".into()),
        }
    }
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

    pub async fn set_flags(&mut self, folder: &str, uids: &[u32], flags: &[Flag]) -> Result<()> {
        self.session.select(folder).await?;

        let uids: Vec<String> = uids.iter().map(|x| x.to_string()).collect();
        let seq_set = uids.join(" ");
        let flags = flags
            .into_iter()
            .cloned()
            .map(From::from)
            .map(Self::internal_flag_to_str)
            .collect::<Vec<String>>()
            .join(" ");
        let query = format!("+FLAGS.SILENT ({})", flags);
        let _ = self.session.uid_store(seq_set, query).await?;
        Ok(())
    }

    pub async fn fetch(&mut self, folder: &str) -> Result<Vec<Message>> {
        // we want to fetch the first email in the INBOX mailbox
        self.session.examine(folder).await?;

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
                let flags = m
                    .flags()
                    .into_iter()
                    .map(Self::internal_flag_to_str)
                    .collect();
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

    fn internal_flag_to_str<'a>(flag: InternalFlag<'a>) -> String {
        match flag {
            InternalFlag::Seen => "\\Seen".into(),
            InternalFlag::Answered => "\\Answered".into(),
            InternalFlag::Flagged => "\\Flagged".into(),
            InternalFlag::Deleted => "\\Deleted".into(),
            InternalFlag::Draft => "\\Draft".into(),
            InternalFlag::Recent => "\\Recent".into(),
            InternalFlag::MayCreate => "\\*".into(),
            InternalFlag::Custom(custom) => custom.into(),
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
