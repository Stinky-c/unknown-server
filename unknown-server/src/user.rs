use crate::{dto, models};
use axum_login::{AuthnBackend, UserId};
use password_auth::verify_password;
use sqlx::PgPool;
use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[derive(Debug, Clone)]
pub struct Backend(PgPool);

impl Backend {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;

impl AuthnBackend for Backend {
    type User = models::user::User;
    type Credentials = dto::auth::CredentialsDto;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = $1")
            .bind(creds.username)
            .fetch_optional(&self.0)
            .await?;

        task::spawn_blocking(|| {
            // We're using password-based authentication: this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| verify_password(creds.password, &user.pw_hash).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.0)
            .await?;

        Ok(user)
    }
}
