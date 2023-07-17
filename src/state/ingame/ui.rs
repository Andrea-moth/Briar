use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct InGameItems;

fn setup(mut commands: Commands) {
    commands
        .spawn(FRAMING())
        .insert(InGameItems)
        .with_children(|parent| {
            parent.spawn(BORDER()).with_children(|parent| {
                parent.spawn(IMAGE_AREA()).with_children(|parent| {
                    parent.spawn(IMAGE()).insert(ImageDisplay);
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

const IMAGE: fn() -> ImageBundle = || ImageBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        ..Default::default()
    },
    ..Default::default()
};

const STORY_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(54.25),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
            top: Val::Percent(0.5),
            bottom: Val::Percent(0.5),
        },
        overflow: Overflow::Hidden,
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

const ACTIONS_AREA: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(9.25),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            bottom: Val::Percent(0.5),
            top: Val::Percent(0.5),
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
        },
        flex_direction: FlexDirection::Column,
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};
