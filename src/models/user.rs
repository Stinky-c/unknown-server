use axum_login::AuthUser;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

use crate::make_mod;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created: time::OffsetDateTime,
    pub modified: time::OffsetDateTime,
    pub pw_hash: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("created", &self.created)
            .field("modified", &self.modified)
            .field("pw_hash", &"[protected]")
            .finish()
    }
}

// AuthUser impl - for axum login
impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.pw_hash.as_bytes()
    }
}

// Convert UserInsert into User
impl From<UserInsert> for User {
    fn from(value: UserInsert) -> Self {
        let now = time::OffsetDateTime::now_local().expect("Cannot get time");
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            created: now,
            modified: now,
            pw_hash: value.pw_hash,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInsert {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub pw_hash: String,
}
impl Debug for UserInsert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserInsert")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("pw_hash", &"[protected]")
            .finish()
    }
}
impl UserInsert {
    pub fn new(username: String, email: String, password: String) -> Self {
        let pw_hash = password_auth::generate_hash(password); //TODO: io blocking. move to thread spawn
        let id = Uuid::now_v7();
        Self {
            id,
            username,
            email,
            pw_hash,
        }
    }
}

make_mod!(prelude User, UserInsert);
