use crate::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
            )
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, ImageAssets>(GameState::Loading);
    }
}
