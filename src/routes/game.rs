use super::{AppState, PublicRoute, RoutePath};
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use steamworks::AppId;

pub struct GameRoute;

impl RoutePath for GameRoute {
    fn path(&self) -> &'static str {
        &"/game"
    }
}

impl PublicRoute for GameRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:id", get(start_game))
    }
}

async fn start_game(Path(id): Path<u32>) -> Result<(), ()> {
    // setup socket {
    // handle offer & ICE candidates
    //   - open game
    //     - stream video to client via webrtc
    //     - handle client input
    // }
    let app_id = AppId::from(id);
    let client = steamworks::Client::init()?;
    Ok(())
}
