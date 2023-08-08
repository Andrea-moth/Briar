use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
    }
}

fn setup(mut commands: Commands, fonts: Res<FontAssets>) {
    commands.spawn(FRAMING()).with_children(|parent| {
        parent.spawn(BORDER()).with_children(|parent| {
            parent.spawn(IMAGE_AREA()).with_children(|parent| {
                parent.spawn(IMAGE()).insert(ImageDisplay);
            });
            parent.spawn(STORY_AREA()).with_children(|parent| {
                parent.spawn(STORY(&fonts)).insert(StoryDisplay);
            });
            parent.spawn(ACTIONS_AREA()).with_children(|parent| {
                parent.spawn(MAP());
            });
        });
    });
}

const FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect::all(Val::Percent(1.5)),
        ..Default::default()
    },
    ..Default::default()
};

const BORDER: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        padding: UiRect {
            top: Val::Percent(0.75),
            bottom: Val::Percent(0.75),
            left: Val::Percent(0.8),
            right: Val::Percent(0.8),
        },
        justify_content: JustifyContent::SpaceBetween,
        ..Default::default()
    },
    background_color: BORDER_COLOUR,
    ..Default::default()
};

const IMAGE_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(34.25),
        height: Val::Percent(100.0),
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const IMAGE: fn() -> ImageBundle = || ImageBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Default::default()
    },
    ..Default::default()
};

const STORY_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(49.25),
        height: Val::Percent(100.0),
        padding: UiRect {
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
            top: Val::Percent(0.5),
            bottom: Val::Percent(0.5),
        },
        overflow: Overflow::clip(),
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const STORY: fn(&Res<FontAssets>) -> TextBundle = |fonts| TextBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Default::default()
    },
    text: Text {
        sections: vec![
            TextSection {
                value: "It's a cold morning".to_string(),
                style: TextStyle {
                    font: fonts.regular.clone(),
                    font_size: 24.0,
                    color: TEXT_COLOUR,
                },
            },
            TextSection {
                value: "\n\n".to_string(),
                style: TextStyle {
                    font: fonts.regular.clone(),
                    font_size: 8.0,
                    color: TEXT_COLOUR,
                },
            },
        ],
        ..Default::default()
    },
    ..Default::default()
};

const ACTIONS_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        width: Val::Percent(14.25),
        height: Val::Percent(100.0),
        padding: UiRect {
            bottom: Val::Percent(1.0),
            top: Val::Percent(1.0),
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
        },
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const MAP: fn() -> ButtonBundle = || ButtonBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        ..Default::default()
    },
    background_color: Color::LIME_GREEN.into(),
    ..Default::default()
};
