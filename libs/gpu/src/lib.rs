mod card;
pub mod card_facade;

#[cfg(target_family = "unix")]
pub fn get_framebuffer() {
    use std::path::Path;

    use card::Card;
    use drm::control::Device;

    let resources = device.resource_handles().unwrap();

    let connector = resources
        .connectors()
        .iter()
        .find(|&&conn| {
            device.get_connector(conn, false).unwrap().state()
                == drm::control::connector::State::Connected
        })
        .expect("No connected connectors found");

    connector.
}
