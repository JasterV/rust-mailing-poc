mod config;
mod imap_client;

use config::Config;
use imap_client::{Client as ImapClient, Credentials};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

async fn fetch_inbox_handler(mut client: ImapClient) -> Result<impl Reply, Rejection> {
    match client.fetch_inbox().await {
        Ok(messages) => Ok(warp::reply::with_status(
            warp::reply::json(&messages),
            StatusCode::OK,
        )),
        Err(error) => Ok(warp::reply::with_status(
            warp::reply::json(&format!("Got an error: {error:?}")),
            StatusCode::IM_A_TEAPOT,
        )),
    }
}

fn with_imap_client(
    client: ImapClient,
) -> impl Filter<Extract = (ImapClient,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

#[tokio::main]
async fn main() {
    let config = Config::build();

    let credentials = Credentials {
        user: config.email_receiver_user,
        password: config.email_receiver_password,
    };
    let imap_client = ImapClient::new((config.server_url, config.imaps_port), credentials).await;

    let health_route = warp::path!("health").and_then(health_handler);

    let fetch_inbox_route = warp::path!("inbox")
        .and(warp::get())
        .and(with_imap_client(imap_client))
        .and_then(fetch_inbox_handler);

    let routes = health_route
        .or(fetch_inbox_route)
        .with(warp::cors().allow_any_origin());

    println!("Running email receiver...");

    warp::serve(routes)
        .run(([0, 0, 0, 0], config.receiver_port))
        .await;
}
