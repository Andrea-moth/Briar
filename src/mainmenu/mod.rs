pub mod ui;

use crate::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(Update, button_system.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), clean_up);
    }
}

fn button_system(
    query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    query.iter().for_each(|(interaction, button)| {
        let Interaction::Pressed = interaction else {
            return;
        };

        match *button {
            MainMenuButton::Start => state.set(GameState::InGame),
            MainMenuButton::Exit => app_exit.send(AppExit),
            _ => {}
        }
    });
}

fn clean_up(query: Query<Entity, With<MainMenuItems>>, mut commands: Commands) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}
