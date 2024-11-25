use std::process::Command;

use crate::services::display::{start_virtual_display, swap_display};

use super::{AppState, PublicRoute, RoutePath, RouteResult};
use axum::{extract::Path, routing::get, Router};
use steamworks::AppId;

pub struct SteamRoute;

impl RoutePath for SteamRoute {
    fn path(&self) -> &'static str {
        &"/steam"
    }
}

impl PublicRoute for SteamRoute {
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

    let display_num = 99;
    swap_display(display_num);
    let mut child =
        start_virtual_display(display_num, &"1920x1080x24").expect("Could not start display");
    child
        .wait()
        .expect("Could not wait for virtual display child");

    let mut handler = Command::new("steam")
        .arg("-applaunch")
        .arg(id.to_string())
        .spawn()
        .expect(&format!("Could not execute steam app {}", id));
    handler.wait().expect("Could not wait for steam child");

    Ok(())
}
