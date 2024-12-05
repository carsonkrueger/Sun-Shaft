use std::env;

use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use lib_routes::error::{RouteError, RouterResult};
use once_cell::sync::Lazy;
use tower_cookies::{Cookie, Cookies};

use ctx::Ctx;

pub const AUTH_TOKEN_HEADER: &'static str = "auth_token";

/// Enforces auth Ctx within extensions and validates the jwt
pub async fn validate_auth(
    ctx: RouterResult<Ctx>,
    req: Request<Body>,
    next: Next,
) -> RouterResult<Response> {
    ctx?.jwt().validate_token(&JWT_SECRET)?;
    Ok(next.run(req).await)
}

/// Creates Ctx from cookies and inserts into Extensions then calls next layer.
/// Returns Err if missing or invalid JWT.
pub async fn ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> RouterResult<Response> {
    let token_str = cookies
        .get(AUTH_TOKEN_HEADER)
        .map(|c| c.value().to_string());

    let result_ctx: Result<Ctx, RouteError> = match token_str {
        Some(t) => match JWT::parse_token(t) {
            Ok(jwt) => Ok(Ctx::new(jwt)),
            Err(e) => Err(RouteError::JWTError(e)),
        },
        None => Err(RouteError::MissingAuthCookie),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(RouteError::MissingAuthCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN_HEADER));
    }

    req.extensions_mut().insert(result_ctx);
    Ok(next.run(req).await)
}
