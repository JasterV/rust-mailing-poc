use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials as LettreCredentials;
use lettre::Message;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

#[derive(Clone, Debug)]
pub struct Client {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    email: String,
}

pub struct Credentials {
    pub user: String,
    pub password: String,
    pub email: String,
    pub domain: String,
}

pub struct SendEmailRequest {
    pub to: String,
    pub body: String,
    pub subject: String,
}

impl Client {
    pub fn new(credentials: Credentials) -> Self {
        let creds = LettreCredentials::new(credentials.user, credentials.password);

        // Open a remote connection to gmail
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&credentials.domain)
                .unwrap()
                .credentials(creds)
                .build();

        Self {
            transport: mailer,
            email: credentials.email,
        }
    }

    pub async fn send_email(&self, data: SendEmailRequest) -> Result<(), String> {
        let email = Message::builder()
            .from(self.email.parse().unwrap())
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
