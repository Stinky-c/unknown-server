pub(crate) mod auth;

crate::make_mod! {
    prelude
    super::auth::router => auth
}
