mod config;
mod smtp_client;

use config::Config;
use smtp_client::{Client as SmtpClient, Credentials, SendEmailRequest};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

async fn send_email_handler(client: SmtpClient, receiver: String) -> Result<impl Reply, Rejection> {
    let mock_req = SendEmailRequest {
        to: receiver,
        subject: "test".into(),
        body: "This is a test".into(),
    };

    match client.send_email(mock_req).await {
        Ok(_) => Ok(warp::reply::with_status(
            "Email sent".into(),
            StatusCode::OK,
        )),
        Err(e) => Ok(warp::reply::with_status(
            format!("Error sending email => #{e:?}"),
            StatusCode::IM_A_TEAPOT,
        )),
    }
}

fn with_smtp_client(
    client: SmtpClient,
) -> impl Filter<Extract = (SmtpClient,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

fn with_receiver(user: String) -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::any().map(move || format!("{0}@localhost", user.clone()))
}

#[tokio::main]
async fn main() {
    let config = Config::build();

    println!("Trying to connect...");

    let client = SmtpClient::new(
        (config.server_url, config.smtp_port),
        Credentials {
            user: config.email_sender_user,
            password: config.email_sender_password,
        },
    )
    .await;

    let health_route = warp::path!("health").and_then(health_handler);
    let send_email_route = warp::path!("send")
        .and(warp::post())
        .and(with_smtp_client(client))
        .and(with_receiver(config.email_receiver_user))
        .and_then(send_email_handler);

    let routes = health_route
        .or(send_email_route)
        .with(warp::cors().allow_any_origin());

    println!("Running email sender...");

    warp::serve(routes)
        .run(([0, 0, 0, 0], config.sender_port))
        .await;
}
