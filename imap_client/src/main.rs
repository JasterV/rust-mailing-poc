mod config;

use config::Config;
use deadpool::managed;
use deadpool_imap::{
    connection::{ConnectionConfig, Credentials},
    session_wrapper::Flag,
    ImapConnectionManager,
};
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

type Pool = managed::Pool<ImapConnectionManager>;

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

async fn set_flags_handler(uids: Vec<u32>, pool: Pool) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().await.unwrap();

    log::debug!("Got new connection from the pool!");
    log::info!("Set flags handler called: {:?}", uids);

    match conn
        .set_flags("INBOX", &uids, &vec![Flag::MyCustomFlag])
        .await
    {
        Ok(()) => Ok(warp::reply::with_status(
            warp::reply::json(&String::from("Flags set")),
            StatusCode::OK,
        )),
        Err(error) => Ok(warp::reply::with_status(
            warp::reply::json(&format!("Got an error: {error:?}")),
            StatusCode::IM_A_TEAPOT,
        )),
    }
}

async fn fetch_inbox_handler(pool: Pool) -> Result<impl Reply, Rejection> {
    let mut conn = pool.get().await.unwrap();

    log::debug!("Got new connection from the pool!");

    match conn.fetch("INBOX").await {
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

fn with_connection_pool(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::build();

    let manager = ImapConnectionManager::new(ConnectionConfig {
        domain: config.server_url,
        port: config.server_port,
        credentials: Credentials {
            user: config.user,
            password: config.password,
        },
    });

    let pool = Pool::builder(manager)
        .build()
        .expect("Can't build the connection pool");

    let _conn = pool.get().await.expect("Can't connect to the IMAP server");

    log::info!("Connection pool built successfully.");

    let health_route = warp::path!("health").and_then(health_handler);

    let set_flags_route = warp::path!("inbox" / "flags")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_connection_pool(pool.clone()))
        .and_then(set_flags_handler);

    let fetch_inbox_route = warp::path!("inbox")
        .and(warp::get())
        .and(with_connection_pool(pool))
        .and_then(fetch_inbox_handler);

    let routes = health_route
        .or(set_flags_route)
        .or(fetch_inbox_route)
        .with(warp::cors().allow_any_origin());

    log::info!("Email receiver running...");

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}
