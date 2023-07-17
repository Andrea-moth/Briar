pub mod dialogue_library;
pub mod prose;
pub mod style;

use crate::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct Story(VecDeque<StoryEvent>);

impl Story {
    fn get_latest_paragraph(&mut self) -> Option<StoryEvent> {
        self.0.pop_front()
    }
    pub fn append_paragraph(&mut self, action: StoryEvent) {
        self.0.push_back(action);
    }
}

#[derive(Clone, Component)]
pub struct StoryDisplay;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Story>()
            .add_event::<StoryEvent>()
            .add_system(update_story_display.in_set(OnUpdate(GameState::InGame)));
    }
}

fn update_story_display(
    mut text_query: Query<&mut Text, With<StoryDisplay>>,
    mut story: ResMut<Story>,
    fonts: Res<FontAssets>,
    input: Res<Input<KeyCode>>,
) {
    if story.is_changed() {
        return;
    }
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
