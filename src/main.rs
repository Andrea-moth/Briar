mod state;

use bevy::prelude::{App, DefaultPlugins};
use state::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
