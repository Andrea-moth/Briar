pub mod image;
pub mod location;
pub mod story;
pub mod ui;

use crate::prelude::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(story::StoryPlugin)
            .add_plugins(image::ImagePlugin)
            .add_plugins(location::LocationPlugin)
            .add_plugins(ui::UiPlugin)
            .add_systems(Update, test.run_if(in_state(GameState::InGame)));
    }
}

fn test(
    input: Res<Input<KeyCode>>,
    mut story: ResMut<Story>,
    mut image: ResMut<CurrentImage>,
    image_assets: Res<ImageAssets>,
) {
    if input.just_pressed(KeyCode::A) {
        story.append_paragraph(TEST.to_vec().into());
    }
    if input.just_pressed(KeyCode::D) {
        story.clear_story();
    }
    if input.just_pressed(KeyCode::S) {
        image.set(image_assets.les.clone());
    }
}
