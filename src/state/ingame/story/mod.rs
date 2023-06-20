pub mod dialogue_library;

use std::collections::VecDeque;

use bevy::{
    prelude::{
        Color, Component, EventReader, Handle, Input, IntoSystemConfig, KeyCode, OnUpdate, Plugin,
        Query, Res, ResMut, Resource, With,
    },
    text::{Font, Text, TextSection, TextStyle},
};

use crate::state::{assets::FontAssets, colours::TEXT_COLOUR, GameState};

#[derive(Resource, Default, Clone)]
struct Story(VecDeque<StoryEvent>);

impl Story {
    fn get_latest_paragraph(&mut self) -> Option<StoryEvent> {
        self.0.pop_front()
    }
    fn append_paragraph(&mut self, action: StoryEvent) {
        self.0.push_back(action);
    }
}

#[derive(Component, Clone)]
pub enum StoryEvent {
    Clear,
    Append(Paragraph),
}

#[derive(Clone)]
pub struct Paragraph(Vec<Snippet>);

impl From<Vec<(&'static str, Character, Style)>> for Paragraph {
    fn from(value: Vec<(&'static str, Character, Style)>) -> Self {
        Self(
            value
                .into_iter()
                .map(|snip| snip.into())
                .collect::<Vec<Snippet>>(),
        )
    }
}
impl Paragraph {
    fn into_text_sections(self, fonts: &Res<FontAssets>) -> Vec<TextSection> {
        self.0
            .into_iter()
            .map(|snip| TextSection {
                value: snip.0,
                style: TextStyle {
                    font: snip.2.into_font(fonts),
                    font_size: 32.0,
                    color: snip.1.into(),
                },
            })
            .chain([TextSection {
                value: "\n\n".to_string(),
                style: TextStyle {
                    font: fonts.regular.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            }])
            .collect::<Vec<TextSection>>()
    }
}

#[derive(Clone)]
struct Snippet(String, Character, Style);

impl From<(&'static str, Character, Style)> for Snippet {
    fn from(value: (&'static str, Character, Style)) -> Self {
        Self(value.0.to_string(), value.1, value.2)
    }
}

#[derive(Clone)]
pub enum Character {
    None,
    Player,
    Victoria,
}
impl Into<Color> for Character {
    fn into(self) -> Color {
        match self {
            Character::None => TEXT_COLOUR,
            Character::Player => Color::ALICE_BLUE,
            Character::Victoria => Color::MIDNIGHT_BLUE,
        }
    }
}

#[derive(Clone)]
pub enum Style {
    None,
    Bold,
    Italic,
    BoldItalic,
}
impl Style {
    fn into_font(self, fonts: &Res<FontAssets>) -> Handle<Font> {
        match self {
            Self::None => fonts.regular.clone(),
            Self::Italic => fonts.italic.clone(),
            Self::Bold => fonts.bold.clone(),
            Self::BoldItalic => fonts.bold_italic.clone(),
        }
    }
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

fn update_story(mut story_event: EventReader<StoryEvent>, mut story: ResMut<Story>) {
    for event in story_event.iter() {
        story.append_paragraph(event.clone());
    }
}

fn update_story_display(
    mut text_query: Query<&mut Text, With<StoryDisplay>>,
    mut story: ResMut<Story>,
    fonts: Res<FontAssets>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut text in &mut text_query {
            let Some(new_paragraph) = story.get_latest_paragraph() else {
                return;
            };
            match new_paragraph {
                StoryEvent::Clear => text.sections.clear(),
                StoryEvent::Append(paragraph) => text
                    .sections
                    .append(&mut paragraph.into_text_sections(&fonts)),
            }
        }
    }
}
