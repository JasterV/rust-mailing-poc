use async_imap::{types::Fetch, Session};
use async_native_tls::{TlsConnector, TlsStream};
use futures::TryStreamExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    imap_session: Session<TlsStream<TcpStream>>,
}

// impl Drop for Client {
//     fn drop(&mut self) {
//         // Create the runtime
//         let rt = tokio::runtime::Runtime::new().unwrap();
//         // Execute the future, blocking the current thread until completion
//         rt.block_on(async {
//             self.imap_session.logout().await.unwrap();
//         });
//     }
// }

#[derive(Debug, Clone)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

pub type ServerAddress = (String, u16);

impl Client {
    pub async fn new((domain, port): ServerAddress, credentials: Credentials) -> Self {
        let tls = TlsConnector::new().danger_accept_invalid_certs(true);

        // we pass in the domain twice to check that the server's TLS
        // certificate is valid for the domain we're connecting to.
        let client = async_imap::connect((domain.clone(), port), domain, tls)
            .await
            .expect("Can't connect to the server");

        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        let imap_session = client
            .login(credentials.user, credentials.password)
            .await
            .expect("Can't log into the imap server");

        Client { imap_session }
    }

    pub async fn fetch_inbox(&mut self, threshold: u16) -> Result<Vec<String>, String> {
        // we want to fetch the first email in the INBOX mailbox
        self.imap_session
            .select("INBOX")
            .await
            .map_err(|e| e.to_string())?;

        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let messages_stream = self
            .imap_session
            .fetch(format!("{threshold}"), "RFC822")
            .await
            .map_err(|io_err| io_err.to_string())?;
        let messages = messages_stream
            .try_collect::<Vec<Fetch>>()
            .await
            .map_err(|io_err| io_err.to_string())?;

        let messages: Vec<String> = messages
            .iter()
            .map(|m| {
                // extract the message's body
                let body = m.body().expect("message did not have a body!");
                std::str::from_utf8(body)
                    .expect("message was not valid utf-8")
                    .to_string()
            })
            .collect();

        Ok(messages)
    }
}
