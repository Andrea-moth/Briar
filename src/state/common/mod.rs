use crate::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOUR))
            .add_system(button_system)
            .add_system(f11)
            .add_startup_system(setup_everything);
    }
}

fn button_system(
    mut query: Query<(&mut BackgroundColor, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (mut colour, interaction) in &mut query {
        match *interaction {
            Interaction::Clicked => *colour = BUTTON_CLICKED,
            Interaction::Hovered => *colour = BUTTON_HOVERED,
            Interaction::None => *colour = BUTTON_COLOUR,
        }
    }
}

fn f11(input: Res<Input<KeyCode>>, mut window_mode: Query<&mut Window>) {
    if input.just_pressed(KeyCode::F11) {
        for mut window in &mut window_mode {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::Fullscreen,
                WindowMode::Fullscreen => WindowMode::Windowed,
                _ => WindowMode::Windowed,
            };
        }
    }
}

fn setup_everything(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
