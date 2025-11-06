use fred::prelude::{Client, Pool};
use sqlx::PgPool;
use std::sync::Arc;
use unknown_actor_lib::pool::ActorPoolRef;

pub struct AppState {
    pg_pool: PgPool,
    fred_pool: Pool,
    actor_pool: ActorPoolRef,
}

impl AppState {
    pub(crate) fn new(pg_pool: PgPool, fred_pool: Pool, actor_pool: ActorPoolRef) -> Self {
        Self {
            pg_pool,
            fred_pool,
            actor_pool,
        }
    }

    pub fn db(&self) -> &PgPool {
        &self.pg_pool
    }

    #[allow(unused)]
    pub fn fred(&self) -> &Client {
        self.fred_pool.next()
    }

    #[allow(unused)]
    pub fn actor(&self) -> ActorPoolRef {
        self.actor_pool.clone()
    }
}

pub type AppStateRef = Arc<AppState>;
