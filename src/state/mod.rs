pub mod assets;
pub mod colours;
mod ingame;
mod loading;
mod mainmenu;

use bevy::{
    app::PluginGroupBuilder,
    prelude::{PluginGroup, States},
};

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
