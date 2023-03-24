mod config;
mod smtp_client;

use config::Config;
use smtp_client::{Client as SmtpClient, Credentials, SendEmailRequest};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

pub async fn send_email_handler(client: SmtpClient) -> Result<impl Reply, Rejection> {
    let mock_req = SendEmailRequest {
        from: "from@localhost".into(),
        to: "patata@localhost".into(),
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

#[tokio::main]
async fn main() {
    let config = Config::build();

    println!("Trying to connect...");

    let client = SmtpClient::new(
        (config.server_url, config.smtp_port),
        Credentials {
            user: config.email_user,
            password: config.email_password,
        },
    )
    .await;

    let health_route = warp::path!("health").and_then(health_handler);
    let send_email_route = warp::path!("send")
        .and(warp::post())
        .and(with_smtp_client(client))
        .and_then(send_email_handler);

    let routes = health_route
        .or(send_email_route)
        .with(warp::cors().allow_any_origin());

    println!("Running email sender...");

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
