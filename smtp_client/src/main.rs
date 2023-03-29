mod config;
mod smtp_client;

use config::Config;
use serde::Deserialize;
use smtp_client::{Client as SmtpClient, Credentials, SendEmailCommand};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct SendEmailRequest {
    subject: String,
    body: String,
}

async fn send_email_handler(
    data: SendEmailRequest,
    client: SmtpClient,
    receiver: String,
) -> Result<impl Reply, Rejection> {
    let mock_req = SendEmailCommand {
        to: receiver,
        subject: data.subject,
        body: data.body,
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
    env_logger::init();

    let config = Config::build();

    log::debug!("Connecting to the server...");

    let client = SmtpClient::new(
        (config.server_url, config.server_port),
        Credentials {
            user: config.user,
            password: config.password,
        },
    )
    .await;

    log::debug!("Connected successfully");

    let health_route = warp::path!("health").and_then(health_handler);
    let send_email_route = warp::path!("send")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_smtp_client(client))
        .and(with_receiver(config.recipient_user))
        .and_then(send_email_handler);

    let routes = health_route
        .or(send_email_route)
        .with(warp::cors().allow_any_origin());

    log::debug!("Running email sender...");

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}
