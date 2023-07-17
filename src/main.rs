mod prelude;
mod state;

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Briar".to_string(),
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(1280., 720.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugins)
        .run();
}
