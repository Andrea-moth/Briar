use bevy::{
    prelude::{
        Component, EventReader, IntoSystemAppConfig, IntoSystemConfig, OnUpdate, Plugin, Query,
        Res, ResMut, Resource, With,
    },
    text::{Text, TextSection},
};

use crate::state::{assets::FontAssets, GameState};

use super::ui::STORY_SECTION;

#[derive(Resource, Default)]
struct Story(Vec<TextSection>);

#[derive(Component)]
pub enum StoryEvent {
    Clear,
    Append(String),
}

#[derive(Component)]
pub struct StoryDisplay;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Story>()
            .add_event::<StoryEvent>()
            .add_system(update_story.in_set(OnUpdate(GameState::InGame)))
            .add_system(update_story_display.in_set(OnUpdate(GameState::InGame)));
    }
}

fn update_story(
    mut story_event: EventReader<StoryEvent>,
    mut story: ResMut<Story>,
    fonts: Res<FontAssets>,
) {
    for event in story_event.iter() {
        match event {
            &StoryEvent::Clear => story.0.clear(),
            StoryEvent::Append(text) => {
                let section = STORY_SECTION(text.clone() + "\n\n", &fonts);
                story.0.append(&mut vec![section]);
            }
        }
    }
}

fn update_story_display(mut text_query: Query<&mut Text, With<StoryDisplay>>, story: Res<Story>) {
    for mut text in &mut text_query {
        text.sections = story.0.clone();
    }
}
