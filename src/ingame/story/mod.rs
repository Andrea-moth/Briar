pub mod dialogue_library;

use crate::prelude::*;

#[derive(Resource, Default, Clone, Debug)]
pub struct Story(VecDeque<StoryAction>);

impl Story {
    fn get_latest_paragraph(&mut self) -> Option<StoryAction> {
        self.0.pop_front()
    }
    pub fn append_paragraph(&mut self, paragraph: Paragraph) {
        self.0.push_back(StoryAction::Append(paragraph));
    }
    pub fn clear_story(&mut self) {
        self.0.push_back(StoryAction::Clear)
    }
}

#[derive(Component, Clone, Debug)]
enum StoryAction {
    Clear,
    Append(Paragraph),
}

#[derive(Clone, Debug)]
pub struct Paragraph(Vec<Snippet>);

impl<T> From<Vec<(T, Character, FontStyle)>> for Paragraph
where
    T: ToString,
{
    fn from(value: Vec<(T, Character, FontStyle)>) -> Self {
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
                    font_size: 24.0,
                    color: snip.1.into(),
                },
            })
            .chain([TextSection {
                value: "\n\n".to_string(),
                style: TextStyle {
                    font: fonts.regular.clone(),
                    font_size: 8.0,
                    color: Color::WHITE,
                },
            }])
            .collect::<Vec<TextSection>>()
    }
}

#[derive(Clone, Debug)]
struct Snippet(String, Character, FontStyle);

impl<T> From<(T, Character, FontStyle)> for Snippet
where
    T: ToString,
{
    fn from(value: (T, Character, FontStyle)) -> Self {
        Self(value.0.to_string(), value.1, value.2)
    }
}

#[derive(Clone, Debug)]
pub enum Character {
    None,
    Player,
    Victoria,
}
impl From<Character> for Color {
    fn from(val: Character) -> Self {
        match val {
            Character::None => TEXT_COLOUR,
            Character::Player => Color::ALICE_BLUE,
            Character::Victoria => Color::MIDNIGHT_BLUE,
        }
    }
}

#[derive(Clone, Debug)]
pub enum FontStyle {
    None,
    Bold,
    Italic,
    BoldItalic,
}
impl FontStyle {
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
        app.init_resource::<Story>().add_systems(
            Update,
            update_story_display.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_story_display(
    mut text_query: Query<&mut Text, With<StoryDisplay>>,
    mut story: ResMut<Story>,
    fonts: Res<FontAssets>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    text_query.iter_mut().for_each(|mut text| {
        if let Some(new_paragraph) = story.get_latest_paragraph() {
            match new_paragraph {
                StoryAction::Clear => text.sections.clear(),
                StoryAction::Append(paragraph) => text
                    .sections
                    .append(&mut paragraph.into_text_sections(&fonts)),
            }
        }
    });
}
