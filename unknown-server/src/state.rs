use crate::prelude::*;
use fred::prelude::{Client, Pool};
use minijinja::{Environment, Value};
use sqlx::PgPool;
use std::sync::Arc;
use unknown_actor_lib::pool::ActorPoolRef;

pub struct AppState {
    pg_pool: PgPool,
    fred_pool: Pool,
    actor_pool: ActorPoolRef,
    jinja_env: Environment<'static>,
}

impl AppState {
    pub(crate) fn new(pg_pool: PgPool, fred_pool: Pool, actor_pool: ActorPoolRef) -> Self {
        let mut jinja_env = Environment::new();
        minijinja_embed::load_templates!(&mut jinja_env);

        Self {
            pg_pool,
            fred_pool,
            actor_pool,
            jinja_env,
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

    pub fn render_template(&self, name: &str, ctx: Option<Value>) -> Result<String> {
        let template = self.jinja_env.get_template(name)?;
        let context = ctx.unwrap_or_default();

        Ok(template.render(context)?)
    }
}

pub type AppStateRef = Arc<AppState>;
