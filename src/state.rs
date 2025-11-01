use fred::prelude::{Client, ClientLike, KeysInterface, Pool};
use sqlx::pool::PoolConnection;
use sqlx::{Error, PgPool, Postgres};
use std::sync::Arc;

pub struct AppState {
    pg_pool: PgPool,
    fred_pool: Pool,
}

impl AppState {
    pub(crate) fn new(pg_pool: PgPool, fred_pool: Pool) -> Self {
        Self { pg_pool, fred_pool }
    }

    pub fn db(&self) -> &PgPool {
        &self.pg_pool
    }

    pub fn fred(&self) -> &Client {
        self.fred_pool.next()
    }
}

pub type AppStateRef = Arc<AppState>;
