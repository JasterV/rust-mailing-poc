use super::Flag;
use anyhow::Result;
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
    pub flags: Vec<Flag>,
}

pub struct SetFlagCommand {
    pub folder: String,
    pub uids: Vec<u32>,
    pub flags: Vec<Flag>,
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

    pub async fn set_flags(&mut self, data: SetFlagCommand) -> Result<()> {
        self.session.select(data.folder).await?;

        let uids: Vec<String> = data.uids.iter().map(|x| x.to_string()).collect();
        let seq_set = uids.join(",");
        let flags = data
            .flags
            .into_iter()
            .map(From::from)
            .collect::<Vec<String>>()
            .join(" ");

        let query = format!("FLAGS ({})", flags);
        let updates_stream = self.session.uid_store(seq_set, query).await?;
        let _updates = updates_stream.try_collect::<Vec<Fetch>>().await?;

        Ok(())
    }

    pub async fn fetch(&mut self, folder: &str) -> Result<Vec<Message>> {
        // we want to fetch the first email in the INBOX mailbox
        self.session.examine(folder).await?;

        let query = "(RFC822 UID RFC822.SIZE RFC822.TEXT FLAGS)";

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
                let flags = m.flags().into_iter().map(From::from).collect();
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
}

#[derive(Error, Debug)]
pub enum ImapError {
    #[error("Error: {0}")]
    InnerImapError(#[from] async_imap::error::Error),
}
