pub mod ui;

use crate::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ui::UiPlugin)
            .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(clean_up.in_schedule(OnEnter(GameState::InGame)));
    }
}

fn button_system(
    button_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, button) in button_query.iter() {
        let Interaction::Clicked = interaction else {
            return;
        };

        match *button {
            MenuButton::Start => state.set(GameState::InGame),
            MenuButton::Exit => app_exit.send(AppExit),
            _ => {}
        }
    }
}

fn clean_up(query: Query<Entity, With<MainMenuItems>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
