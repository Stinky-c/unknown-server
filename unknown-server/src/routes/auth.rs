use crate::prelude::*;
use crate::user;

pub(crate) fn router() -> Router<AppStateRef> {
    Router::new()
        .route("/", get(get_root))
        .route("/login", get(get_login).post(post_login))
        .route("/signup", get(get_signup).post(post_signup))

    // .route("/login", get(get_info))
    // .route("/login", post(post_login))
    // .route("/signup", post(post_signup))
    // .route("/logout", post(post_logout))
}

async fn get_root() -> Redirect {
    Redirect::to("/auth/signin")
}

async fn get_login(State(state): State<AppStateRef>) -> ResultHtml {
    let template = state.render_template("auth/login.j2.html", None)?;
    Ok(Html(template))
}

async fn get_signup(State(state): State<AppStateRef>) -> ResultHtml {
    let template = state.render_template("auth/signup.j2.html", None)?;
    Ok(Html(template))
}

async fn post_login(
    mut auth_session: AuthSession,
    Form(credentials): Form<dto::auth::UserLoginDto>,
) -> ResultJson<dto::auth::UserInfoDto> {
    info!("{:?}", credentials);

    let user = match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED.into()),
        Err(err) => return Err(AppError::AxumLogin(err)),
    };

    if let Err(err) = auth_session.login(&user).await {
        return Err(AppError::AxumLogin(err));
    }

    Ok(Json(dto::auth::UserInfoDto {
        id: user.id,
        username: user.username,
    }))
}

async fn post_signup(
    State(state): State<AppStateRef>,
    mut auth_session: AuthSession,
    Form(signup): Form<dto::auth::UserSignupDto>,
) -> ResultJson<dto::auth::UserInfoDto> {
    // Signup token check
    if CONFIG.signup.disable {
        warn!("Signup is completely disabled");
        return Err(StatusCode::FORBIDDEN.into());
    }

    if signup.password != signup.confirm_password {
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
        warn!("Account already exists");
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    let pw_hash = user::hash_password(signup.password)
        .await
        .map_err(|_| AppError::Code(StatusCode::INTERNAL_SERVER_ERROR))?; //TODO: map to a better error
    let user = models::user::UserInsert::new(signup.username, signup.email, pw_hash);

    let insert: models::user::User = sqlx::query_as(
        "INSERT INTO users (id, username, email, pw_hash) values ( $1, $2, $3, $4) returning *",
    )
    .bind(user.id)
    .bind(user.username)
    .bind(user.email)
    .bind(user.pw_hash)
    .fetch_one(state.db())
    .await?;

    if let Err(err) = auth_session.login(&insert).await {
        return Err(AppError::AxumLogin(err));
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
    if let Err(err) = auth_session.logout().await {
        return Err(AppError::AxumLogin(err));
    }

    Ok(Json(dto::shared::SuccessResponse {
        message: "Success".to_string(),
    }))
}
