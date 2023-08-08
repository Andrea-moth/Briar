pub mod assets;
pub mod ingame;
pub mod loading;
pub mod mainmenu;
pub mod prelude;

use crate::prelude::*;

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(loading::LoadingPlugin)
            .add(mainmenu::MainMenuPlugin)
            .add(ingame::InGamePlugin)
    }
}
