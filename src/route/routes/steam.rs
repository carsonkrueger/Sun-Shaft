use crate::route::error::RouteResult;

use super::super::{AppState, RoutePath, RouteRouter};
use axum::{extract::Path, routing::get, Router};
use std::process::Command;

pub struct SteamRoute;

impl RoutePath for SteamRoute {
    fn path(&self) -> &'static str {
        &"/steam"
    }
}

impl RouteRouter for SteamRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:id", get(start_game))
    }
}

async fn start_game(Path(id): Path<u32>) -> RouteResult<()> {
    // setup socket {
    // handle offer & ICE candidates
    //   - open game
    //     - stream video to client via webrtc
    //     - handle client input
    // }
    // let app_id = AppId::from(id);
    // let client = steamworks::Client::init()?;

    // if !client.0.apps().is_app_installed(app_id) {
    //     return Err(super::RouteError::SAAppNotInstalled(app_id));
    // }

    let mut handler = Command::new("steam")
        .arg("-applaunch")
        .arg(id.to_string())
        .spawn()
        .expect(&format!("Could not execute steam app {}", id));
    handler.wait().expect("Could not wait for steam child");

    Ok(())
}
