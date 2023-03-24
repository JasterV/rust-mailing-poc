mod smtp_client;

use smtp_client::{Client as SmtpClient, Credentials, SendEmailRequest};
use std::convert::Infallible;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

pub async fn send_email_handler(client: SmtpClient) -> std::result::Result<impl Reply, Rejection> {
    let mock_req = SendEmailRequest {
        to: "patata@email.com".into(),
        subject: "test".into(),
        body: "This is a test".into(),
    };

    match client.send_email(mock_req).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::IM_A_TEAPOT),
    }
}

fn with_smtp_client(
    client: SmtpClient,
) -> impl Filter<Extract = (SmtpClient,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

#[tokio::main]
async fn main() {
    let client = SmtpClient::new(Credentials {
        email: "mock@email.com".into(),
        domain: "localhost".into(),
        user: "admin".into(),
        password: "password".into(),
    });

    let health_route = warp::path!("health").and_then(health_handler);
    let send_email_route = warp::path!("send")
        .and(warp::post())
        .and(with_smtp_client(client))
        .and_then(send_email_handler);

    let routes = health_route
        .or(send_email_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
