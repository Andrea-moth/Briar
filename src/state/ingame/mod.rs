pub mod image;
pub mod location;
pub mod story;
pub mod ui;

use crate::prelude::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(story::StoryPlugin)
            .add_plugin(image::ImagePlugin)
            .add_plugin(location::LocationPlugin)
            .add_plugin(ui::UiPlugin)
            .add_system(test.in_set(OnUpdate(GameState::InGame)));
    }
}

fn test(mut story: ResMut<Story>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::A) {
        story.append_paragraph(StoryEvent::Append(TEST.to_vec().into()));
    }
}
