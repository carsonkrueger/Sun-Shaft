use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;

use crate::{
    route::{error::RouteResult, state::AppState, RoutePath, RouteRouter},
    services::user::create_user,
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
    Json(body): Json<PostSignupBody>,
) -> RouteResult<()> {
    let user = create_user(&body.email, &body.password, &s.pool).await?;
    Ok(())
}

async fn post_login(State(s): State<AppState>) -> RouteResult<()> {
    Ok(())
}
