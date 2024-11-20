use super::{AppState, PublicRoute, RoutePath};
use axum::{response::IntoResponse, routing::get, Router};

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

async fn start_game() -> Result<(), ()> {
    // setup socket {
    // handle offer & ICE candidates
    //   - open game
    //     - stream video to client via webrtc
    //     - handle client input
    // }
    Ok(())
}
