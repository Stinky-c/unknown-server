use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoDto {
    pub id: uuid::Uuid,
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CredentialsDto {
    pub username: String,
    pub password: String,
}

impl Debug for CredentialsDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CredentialsDto")
            .field("username", &self.username)
            .field("password", &"[protected]")
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserSignupDto {
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Unhashed password
    pub password: String,
    /// Signup token
    pub token: String,
}
impl Debug for UserSignupDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserSignupDto")
            .field("username", &self.username)
            .field("email", &self.email)
            .field("token", &"[protected]")
            .field("password", &"[protected]")
            .finish()
    }
}

crate::make_mod!(prelude UserInfoDto, CredentialsDto, UserSignupDto);

// Convert UserSignup into a UserInsert object
impl From<UserSignupDto> for crate::models::user::UserInsert {
    fn from(value: UserSignupDto) -> Self {
        Self::new(value.username, value.email, value.password)
    }
}
