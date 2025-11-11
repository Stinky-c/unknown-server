use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoDto {
    pub id: uuid::Uuid,
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserLoginDto {
    pub username: String,
    pub password: String,

    #[serde(default)] // Default for bool is false.
    pub remember: bool,
}

impl Debug for UserLoginDto {
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
    /// unhashed confirm password
    pub password_confirm: String,
}
impl Debug for UserSignupDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserSignupDto")
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password", &"[protected]")
            .finish()
    }
}

crate::make_mod!(prelude UserInfoDto, UserLoginDto, UserSignupDto);
