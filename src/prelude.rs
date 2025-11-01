#![allow(unused_imports)]
pub use crate::error::AppError;
pub use crate::state::{AppState, AppStateRef};
pub use axum::routing::{delete, get, patch, post, put};
pub use axum::{Form, Json, Router};
pub use axum::extract::State;


pub use crate::config::CONFIG;
pub use crate::user::{AuthSession, Backend};
use axum_login::login_required;

pub(crate) use tracing::{debug, error, info, trace, warn};

pub(crate) use crate::dto;
pub(crate) use crate::models;

pub type Result<T> = axum::response::Result<T, AppError>;
pub type ResultJson<T> = Result<Json<T>>;
pub type ResultForm<T> = Result<Form<T>>;

/// A simple macro used to make a crate visible module and export names from `super`.
/// ```
/// make_mod!(name, FooBar, BarBaz);
/// ```
/// ```
/// pub (crate) mod name {
///     pub(crate) user super::FooBar;
///     pub(crate) user super::BarBaz;
/// }
/// ```
#[macro_export]
macro_rules! make_mod {
    // applies super
    ($modname:ident $( $use_ident:ident $( => $exportname:ident)? ),* ) => {
        #[allow(unused_imports)]
        pub (crate) mod $modname {
            $(
                pub(crate) use super::$use_ident $(as $exportname)?;
            )*
        }
    };
    // Uses a full path
    ($modname:ident $( $use_item:path $( => $exportname:ident)? ),* ) => {
        #[allow(unused_imports)]
        pub (crate) mod $modname {
            $(
                pub(crate) use $use_item $(as $exportname)?;
            )*
        }
    }
}

#[derive(sqlx::FromRow, PartialEq, Eq, Debug)]
/// A helper that encapsulates a single
pub struct DBExists {
    exists: bool,
}

impl DBExists {
    pub fn exists(self) -> bool {
        self.exists
    }
}
