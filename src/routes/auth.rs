use crate::prelude::*;
use axum::http::StatusCode;

pub(crate) fn router() -> Router<AppStateRef> {
    Router::new()
        .route("/login", get(get_info))
        .route("/login", post(post_login))
        .route("/signup", post(post_signup))
        .route("/logout", post(post_logout))
}

async fn post_login(
    mut auth_session: AuthSession,
    Json(credentials): Json<dto::auth::CredentialsDto>,
) -> ResultJson<dto::auth::UserInfoDto> {
    let user = match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED.into()),
        Err(err) => return Err(AppError::AxumLogin(err)),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into());
    }

    Ok(Json(dto::auth::UserInfoDto {
        id: user.id,
        username: user.username,
    }))
}

async fn post_signup(
    State(state): State<AppStateRef>,
    mut auth_session: AuthSession,
    Json(signup): Json<dto::auth::UserSignupDto>,
) -> ResultJson<dto::auth::UserInfoDto> {
    // Signup token check
    if (*CONFIG).signup.disable == true {
        warn!("Signup is completely disabled");
        return Err(StatusCode::IM_A_TEAPOT.into());
    }
    if (*CONFIG).signup.token != signup.token {
        return Err(StatusCode::UNAUTHORIZED.into());
    }
    // check if a user with similar creds exists
    let user_exists: DBExists =
        sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1 OR username = $2);")
            .bind(&signup.email)
            .bind(&signup.username)
            .fetch_one(state.db())
            .await?;

    if user_exists.exists() {
        // User exists. return early
        return Err(StatusCode::UNAUTHORIZED.into());
    }
    info!("User does not exists");

    let user: models::user::UserInsert = signup.into();

    let insert: models::user::User = sqlx::query_as(
        "INSERT INTO users (id, username, email, pw_hash) values ( $1, $2, $3, $4) returning *",
    )
    .bind(user.id)
    .bind(user.username)
    .bind(user.email)
    .bind(user.pw_hash)
    .fetch_one(state.db())
    .await?;

    if auth_session.login(&insert).await.is_err() {
        return Err(StatusCode::UNAUTHORIZED.into());
    };

    Ok(Json(dto::auth::UserInfoDto {
        id: insert.id,
        username: insert.username,
    }))
}

async fn get_info(auth_session: AuthSession) -> ResultJson<dto::auth::UserInfoDto> {
    let user = match auth_session.user {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    Ok(Json(dto::auth::UserInfoDto {
        id: user.id,
        username: user.username,
    }))
}

async fn post_logout(mut auth_session: AuthSession) -> ResultJson<dto::shared::SuccessResponse> {
    let user = match auth_session.logout().await {
        Ok(u) => u,
        Err(_) => return Err(StatusCode::UNAUTHORIZED.into()),
    };

    Ok(Json(dto::shared::SuccessResponse {
        message: "Success".to_string(),
    }))
}
