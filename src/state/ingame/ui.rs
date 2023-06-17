use bevy::{
    prelude::{BuildChildren, Commands, ImageBundle, NodeBundle, Res, TextBundle},
    text::{TextSection, TextStyle},
    ui::{JustifyContent, Size, Style, UiRect, Val},
};

use crate::state::{
    assets::{FontAssets, ImageAssets},
    colours::{BORDER_COLOUR, INNER_COLOUR, TEXT_COLOUR},
};

use super::story::StoryDisplay;

pub fn setup(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn(FRAMING()).with_children(|parent| {
        parent.spawn(BORDER()).with_children(|parent| {
            parent.spawn(IMAGE_AREA()).with_children(|parent| {
                parent.spawn(IMAGE(images));
            });
            parent.spawn(STORY_AREA()).with_children(|parent| {
                parent.spawn(STORY()).insert(StoryDisplay);
            });
            parent.spawn(ACTIONS_AREA());
        });
    });
}

const FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        padding: UiRect::all(Val::Percent(1.5)),
        ..Default::default()
    },
    ..Default::default()
};

const BORDER: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
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
        size: Size {
            width: Val::Percent(34.25),
            height: Val::Percent(100.0),
        },
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const IMAGE: fn(Res<ImageAssets>) -> ImageBundle = |images| ImageBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        ..Default::default()
    },
    image: images.les.clone().into(),
    ..Default::default()
};

const STORY_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(44.25),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
            top: Val::Percent(0.5),
            bottom: Val::Percent(0.5),
        },
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const STORY: fn() -> TextBundle = || TextBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        ..Default::default()
    },
    ..Default::default()
};

pub const STORY_SECTION: fn(String, &Res<FontAssets>) -> TextSection = |value, fonts| TextSection {
    value,
    style: TextStyle {
        font: fonts.regular.clone(),
        font_size: 48.0,
        color: TEXT_COLOUR,
    },
};

const ACTIONS_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(19.25),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            bottom: Val::Percent(3.0),
            ..Default::default()
        },
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};
