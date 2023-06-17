use bevy::{
    prelude::{Button, Camera2dBundle, Changed, ClearColor, Commands, Plugin, Query, With},
    ui::{BackgroundColor, Interaction},
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use super::{
    assets::{FontAssets, ImageAssets},
    colours::{BACKGROUND_COLOUR, BUTTON_CLICKED, BUTTON_COLOUR, BUTTON_HOVERED},
    GameState,
};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .insert_resource(ClearColor(BACKGROUND_COLOUR))
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
            )
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading)
            .add_startup_system(setup_everything)
            .add_system(button_system);
    }
}

fn button_system(
    mut query: Query<(&mut BackgroundColor, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (mut colour, interaction) in &mut query {
        match interaction {
            &Interaction::Clicked => *colour = BUTTON_CLICKED,
            &Interaction::Hovered => *colour = BUTTON_HOVERED,
            &Interaction::None => *colour = BUTTON_COLOUR,
        }
    }
}

fn setup_everything(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
