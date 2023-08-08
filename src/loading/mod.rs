use crate::prelude::*;

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
            .add_systems(Startup, setup_everything)
            .add_systems(Update, button_system);
    }
}

fn button_system(
    mut query: Query<(&mut BackgroundColor, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    query
        .iter_mut()
        .for_each(|(mut color, interaction)| match *interaction {
            Interaction::Pressed => *color = BUTTON_CLICKED,
            Interaction::Hovered => *color = BUTTON_HOVERED,
            Interaction::None => *color = BUTTON_COLOUR,
        });
}

fn setup_everything(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
