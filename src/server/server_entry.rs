use anyhow::Ok;
use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, Uri},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    result::{AppError, AppResponse, AppResult, AppResultUtil},
};

use super::{common, init::init};

pub async fn server_entry(config: &Config) {
    init(&config).await;

    return;

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .fallback(common::fallback);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
