use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    middleware::auth::{AUTH_TOKEN_HEADER, JWT_SECRET},
    route::{
        error::{RouteError, RouteResult},
        state::AppState,
        RoutePath, RouteRouter,
    },
    services::{
        jwt::JWT,
        user::{create_user, get_user_by_email, verify_user},
    },
};

pub struct AuthRoute;

impl RoutePath for AuthRoute {
    fn path(&self) -> &'static str {
        "/auth"
    }
}

impl RouteRouter for AuthRoute {
    fn router(&self) -> axum::Router<crate::route::state::AppState> {
        Router::new()
            .route("/signup", post(post_signup))
            .route("/login", post(post_login))
    }
}

#[derive(Deserialize)]
struct PostSignupBody {
    pub email: String,
    pub password: String,
}

async fn post_signup(
    State(s): State<AppState>,
    cookies: Cookies,
    Json(body): Json<PostSignupBody>,
) -> RouteResult<()> {
    let user = create_user(&body.email, &body.password, &s.pool).await?;
    let jwt = JWT::new(user.id, Vec::new());
    let jwt_str = jwt.encode(JWT_SECRET.as_bytes())?;
    let cookie = Cookie::new(AUTH_TOKEN_HEADER, jwt_str);
    cookies.add(cookie);
    Ok(())
}

async fn post_login(
    State(s): State<AppState>,
    cookies: Cookies,
    Json(body): Json<PostSignupBody>,
) -> RouteResult<()> {
    let user = get_user_by_email(&body.email, &s.pool)
        .await?
        .ok_or(RouteError::InvalidAuth)?;
    verify_user(&user, &body.password)?;
    let jwt = JWT::new(user.id, Vec::new());
    let jwt_str = jwt.encode(JWT_SECRET.as_bytes())?;
    let cookie = Cookie::new(AUTH_TOKEN_HEADER, jwt_str);
    cookies.add(cookie);
    Ok(())
}
