use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials as LettreCredentials;
use lettre::Message;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

#[derive(Clone, Debug)]
pub struct Client {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    user: String,
}

type ServerAddress = (String, u16);

#[derive(Debug)]
pub struct Credentials {
    pub user: String,
    pub password: String,
}

pub struct SendEmailRequest {
    pub to: String,
    pub body: String,
    pub subject: String,
}

impl Client {
    pub async fn new((domain, port): ServerAddress, credentials: Credentials) -> Self {
        let creds = LettreCredentials::new(credentials.user.clone(), credentials.password);

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(domain)
                .credentials(creds)
                .port(port)
                .build();

        mailer
            .test_connection()
            .await
            .expect("Couldn't connect to SMTP server");

        Self {
            transport: mailer,
            user: credentials.user,
        }
    }

    pub async fn send_email(&self, data: SendEmailRequest) -> Result<(), String> {
        let from = format!("{0}@localhost", self.user.clone());

        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(data.to.parse().unwrap())
            .subject(data.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(data.body)
            .unwrap();

        // Send the email
        match self.transport.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Could not send email: {e:?}")),
        }
    }
}
