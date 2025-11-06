mod config;
mod dto;
mod error;
mod models;
mod prelude;
mod routes;
mod state;
mod user;

use crate::prelude::*;
use axum::{Extension, Router, routing::get};
use axum_login::AuthManagerLayerBuilder;
use axum_prometheus::PrometheusMetricLayer;
use fred::prelude::{ClientLike, Config as FredConfig, Pool as FredPool};
use memory_serve::{CacheControl, MemoryServe, load_assets};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use time::Duration;
use tokio::signal;
use tower_http::trace::{OnFailure, TraceLayer};
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use unknown_actor_lib::pool::pool;

type Result<T, E = Box<dyn core::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let fmt_layer = {
        let format = tracing_subscriber::fmt::format().with_source_location(false);

        tracing_subscriber::fmt::layer().event_format(format)
    };

    let filter_layer =
        { EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("debug"))? };

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();

    // info!("{:?}",*CONFIG);
    // std::process::exit(0);

    // Fred connection
    let fred_pool = {
        let s = tracing::info_span!("startup_fred");
        let _ = s.enter();
        let connection = &CONFIG.redis.url;

        info!("Connecting pool");
        let config = FredConfig::from_url(connection)?;
        let pool = FredPool::new(
            config,
            None,
            None,
            None,
            CONFIG.redis.max_connections as usize,
        )?;

        info!("Testing connection");
        let _ = pool.connect();
        pool.wait_for_connect().await?;

        info!("Connected");
        pool
    };

    let pg_pool = {
        let s = tracing::info_span!("startup_pg");
        let _ = s.enter();
        let connection = &CONFIG.database.url;
        info!("Connecting pool");
        let pool = PgPoolOptions::new()
            .max_connections(CONFIG.database.max_connections)
            .connect(connection)
            .await?;

        info!("Testing connection");
        let _ = sqlx::query("SELECT $1").bind(1).fetch_one(&pool).await?;

        info!("Applying migrations.");
        sqlx::migrate!("./migrations").run(&pool).await?;
        info!("Migrations done!");

        info!("Connected");
        pool
    };

    // Metrics
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Axum sessions
    let auth_layer = {
        let s = tracing::info_span!("auth");
        let _ = s.enter();
        let session_store = RedisStore::new(fred_pool.clone());

        let session_layer = SessionManagerLayer::new(session_store)
            .with_expiry(Expiry::OnInactivity(Duration::hours(1)))
            .with_secure(false);

        let backend = Backend::new(pg_pool.clone());
        let layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        layer
    };

    let jinja_env = {
        let mut env = minijinja::Environment::new();
        minijinja_embed::load_templates!(&mut env);

        env
    };

    let assets_router = {
        MemoryServe::new(load_assets!("assets/"))
            .index_file(None)
            .cache_control(CacheControl::NoCache)
            .into_router()
    };

    // Actor pool
    let actor_pool = {
        let pool = pool(None, None).await?;

        pool
    };

    let state = Arc::new(AppState::new(pg_pool, fred_pool, actor_pool));
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .nest("/auth", routes::auth::router())
        .nest("/upload", routes::upload::router())
        .merge(assets_router)
        .with_state(state)
        .layer(auth_layer)
        .layer(prometheus_layer)
        .layer(Extension(jinja_env))
        .layer((*CONFIG).ip_source.clone().into_extension())
        .layer(TraceLayer::new_for_http());

    let app_host = &CONFIG.app_host;
    let listener = tokio::net::TcpListener::bind(app_host).await?;
    info!(
        "Signup token is set to '{}'. Use it when trying to signup.",
        CONFIG.signup.token
    );
    info!("Starting on {app_host}");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    info!("Done! Exiting...");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {warn!("shutdown signal received")},
        _ = terminate => {info!("shutdown signal received")},
    }
}
