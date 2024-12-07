use crate::services::jwt::JWT;
use crate::{
    route::error::{RouteError, RouteResult},
    services::ctx::Ctx,
};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use std::{cell::LazyCell, env};
use tower_cookies::{Cookie, Cookies};

pub const AUTH_TOKEN_HEADER: &'static str = "auth_token";
pub const JWT_SECRET: LazyCell<String> =
    LazyCell::new(|| env::var("JWT_SECRET").expect("JWT_SECRET"));

/// Enforces auth Ctx within extensions and validates the jwt
pub async fn validate_auth(
    ctx: RouteResult<Ctx>,
    req: Request<Body>,
    next: Next,
) -> RouteResult<Response> {
    ctx?;
    Ok(next.run(req).await)
}

/// Creates Ctx from cookies and inserts into Extensions then calls next layer.
/// Returns Err if missing or invalid JWT.
pub async fn ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> RouteResult<Response> {
    let token_str = cookies
        .get(AUTH_TOKEN_HEADER)
        .map(|c| c.value().to_string());

    let result_ctx: Result<Ctx, RouteError> = match token_str {
        Some(t) => match JWT::decode(&t, JWT_SECRET.as_bytes()) {
            Ok(jwt) => Ok(Ctx::new(jwt.claims)),
            Err(e) => Err(RouteError::JWT(e.to_string())),
        },
        None => Err(RouteError::MissingAuthCookie),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(RouteError::MissingAuthCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN_HEADER));
    }

    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}
