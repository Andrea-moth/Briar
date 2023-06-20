mod image;
mod location;
mod story;
mod ui;

use bevy::prelude::{EventWriter, IntoSystemAppConfig, OnEnter, Plugin, Res};

use self::{
    image::ImageEvent,
    story::{
        dialogue_library::{TEST, TEST2},
        StoryEvent,
    },
    ui::setup,
};

use super::{assets::ImageAssets, GameState};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(story::StoryPlugin)
            .add_plugin(image::ImagePlugin)
            .add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(test.in_schedule(OnEnter(GameState::InGame)));
    }
}

fn test(
    mut story: EventWriter<StoryEvent>,
    mut image: EventWriter<ImageEvent>,
    images: Res<ImageAssets>,
) {
    story.send(StoryEvent::Append(TEST.to_vec().into()));
    story.send(StoryEvent::Append(TEST2.to_vec().into()));
    story.send(StoryEvent::Clear);
    image.send(ImageEvent::Set(images.les.clone()));
}
