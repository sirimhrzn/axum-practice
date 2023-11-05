use std::time::Duration;

use crate::{handlers::api, middlewares::fallbacks::fallback_root_api};
use anyhow::Context;
use axum::{
    error_handling::HandleErrorLayer,
    middleware,
    routing::{get, post},
    Router,
};
use middlewares::{error_message::mw_error_response, timeout::mw_timeout_response};
use tower::ServiceBuilder;
use tracing::info;
mod error;
mod handlers;
mod middlewares;

pub async fn server() -> anyhow::Result<()> {
    info!("initializing router...");

    let test_routes = Router::new()
        .route("/", post(api::login))
        .route("/", get(api::timeout_handler))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(mw_timeout_response))
                .timeout(Duration::from_secs(2)),
        );

    let router = Router::new()
        .nest("/test", test_routes)
        .fallback(fallback_root_api)
        .layer(middleware::from_fn(mw_error_response));

    let port = 8001_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, now listening on port {}", port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;
    Ok(())
}
