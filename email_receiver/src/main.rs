mod config;
mod imap_client;

use config::Config;
use futures::lock::Mutex;
use imap_client::{Client as ImapClient, Credentials};
use std::{convert::Infallible, sync::Arc};
use warp::{http::StatusCode, Filter, Rejection, Reply};

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

async fn fetch_inbox_handler(client: Arc<Mutex<ImapClient>>) -> Result<impl Reply, Rejection> {
    let mut client = client.lock().await;

    match client.fetch_inbox(10).await {
        Ok(messages) => Ok(warp::reply::with_status(
            format!("Got {0} messages", messages.len()),
            StatusCode::OK,
        )),
        Err(error) => Ok(warp::reply::with_status(
            format!("Got an error: {error:?}"),
            StatusCode::IM_A_TEAPOT,
        )),
    }
}

fn with_imap_client(
    client: ImapClient,
) -> impl Filter<Extract = (Arc<Mutex<ImapClient>>,), Error = Infallible> + Clone {
    let arc = Arc::new(Mutex::new(client));
    warp::any().map(move || arc.clone())
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
