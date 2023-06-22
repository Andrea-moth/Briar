mod image;
mod location;
mod story;
mod ui;

use bevy::prelude::{
    EventWriter, Input, IntoSystemAppConfig, IntoSystemConfig, KeyCode, OnEnter, OnUpdate, Plugin,
    Res,
};

use self::{
    story::{dialogue_library::TEST, StoryEvent},
    ui::setup,
};

use super::GameState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(story::StoryPlugin)
            .add_plugin(image::ImagePlugin)
            .add_plugin(location::LocationPlugin)
            .add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(test.in_set(OnUpdate(GameState::InGame)));
    }
}

fn test(input: Res<Input<KeyCode>>, mut story_event: EventWriter<StoryEvent>) {
    if input.just_pressed(KeyCode::A) {
        story_event.send(StoryEvent::Append(TEST.to_vec().into()));
    }
}
