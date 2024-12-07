use crate::{
    enums::permission::Permission,
    route::error::{RouteError, RouteResult},
    services::ctx::Ctx,
};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use futures::future::BoxFuture;

pub fn create_permission_mw(
    permission: Permission,
) -> impl Fn(Ctx, Request<Body>, Next) -> BoxFuture<'static, RouteResult<Response>> + Clone {
    move |ctx: Ctx, mut req: Request<Body>, next: Next| {
        req.extensions_mut().insert(permission.clone());
        Box::pin(async move { mw(ctx, req, next).await })
    }
}

async fn mw(c: Ctx, r: Request<Body>, n: Next) -> RouteResult<Response<Body>> {
    let permission = r
        .extensions()
        .get::<Permission>()
        .ok_or(RouteError::RouteMissingPermission)?;
    if !c.jwt().permissions().contains(permission) {
        return Err(RouteError::PermissionUnathorized);
    };
    Ok(n.run(r).await)
}
