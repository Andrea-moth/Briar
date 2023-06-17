mod location;
mod story;
mod ui;

use bevy::prelude::{EventWriter, IntoSystemAppConfig, OnEnter, Plugin};

use self::{story::StoryEvent, ui::setup};

use super::GameState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(story::StoryPlugin)
            .add_system(setup.in_schedule(OnEnter(GameState::InGame)))
            .add_system(test_test.in_schedule(OnEnter(GameState::InGame)))
            .add_system(text_test.in_schedule(OnEnter(GameState::InGame)));
    }
}

fn test_test(mut event_writer: EventWriter<StoryEvent>) {
    event_writer.send(StoryEvent::Append("Hello there".to_string()));
}

fn text_test(mut event_writer: EventWriter<StoryEvent>) {
    event_writer.send(StoryEvent::Clear);
}
