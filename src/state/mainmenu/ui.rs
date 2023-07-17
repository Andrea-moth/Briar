use crate::prelude::*;

#[derive(Component, Default)]
pub enum Button {
    #[default]
    Start,
    Options,
    Exit,
}
impl ToString for Button {
    fn to_string(&self) -> String {
        match self {
            Self::Start => "Start",
            Self::Options => "Options",
            Self::Exit => "Exit",
        }
        .to_string()
    }
}
impl Button {
    const LIST: [Self; 3] = [Self::Start, Self::Options, Self::Exit];
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::MainMenu)));
    }
}

#[derive(Component)]
pub struct MainMenuItems;

pub fn setup(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn(FRAMING())
        .insert(MainMenuItems)
        .with_children(|parent| {
            parent.spawn(BORDER()).with_children(|parent| {
                parent.spawn(INNER()).with_children(|parent| {
                    parent.spawn(TITLE_FRAMING()).with_children(|parent| {
                        parent.spawn(TITLE(&fonts));
                    });
                    parent.spawn(BUTTON_FRAMING()).with_children(|parent| {
                        parent.spawn(BUTTON_INNER()).with_children(|parent| {
                            for button in Button::LIST {
                                let text = button.to_string();
                                parent.spawn(BUTTON_PADDING()).with_children(|parent| {
                                    parent
                                        .spawn(BUTTON())
                                        .insert(button)
                                        .with_children(|parent| {
                                            parent.spawn(BUTTON_TEXT(&fonts, text));
                                        });
                                });
                            }
                        });
                    });
                });
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
        padding: UiRect::all(Val::Percent(0.75)),
        ..Default::default()
    },
    background_color: BORDER_COLOUR,
    ..Default::default()
};

const INNER: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            bottom: Val::Percent(3.0),
            ..Default::default()
        },
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    },
    background_color: INNER_COLOUR,
    ..Default::default()
};

const TITLE_FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(70.0),
            height: Val::Percent(60.0),
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    },
    ..Default::default()
};

const TITLE: fn(&Res<FontAssets>) -> TextBundle = |fonts| TextBundle {
    text: Text {
        sections: vec![TITLE_SECTIONS(fonts, "Briar".to_string())],
        alignment: TextAlignment::Center,
        linebreak_behaviour: BreakLineOn::WordBoundary,
    },
    ..Default::default()
};

const TITLE_SECTIONS: fn(&Res<FontAssets>, String) -> TextSection = |fonts, value| TextSection {
    value,
    style: TextStyle {
        font: fonts.regular.clone(),
        font_size: 128.0,
        color: TEXT_COLOUR,
    },
};

const BUTTON_FRAMING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(25.0),
            height: Val::Percent(40.0),
        },
        padding: UiRect::all(Val::Percent(0.5)),
        ..Default::default()
    },
    ..Default::default()
};

const BUTTON_INNER: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        padding: UiRect::all(Val::Percent(1.0)),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    },
    ..Default::default()
};

const BUTTON_PADDING: fn() -> NodeBundle = || NodeBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        padding: UiRect {
            left: Val::Percent(1.0),
            right: Val::Percent(1.0),
            top: Val::Percent(3.0),
            bottom: Val::Percent(3.0),
        },
        ..Default::default()
    },
    ..Default::default()
};

const BUTTON: fn() -> ButtonBundle = || ButtonBundle {
    style: Style {
        size: Size {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
        },
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    },
    background_color: BUTTON_COLOUR,
    ..Default::default()
};

const BUTTON_TEXT: fn(&Res<FontAssets>, String) -> TextBundle = |fonts, value| TextBundle {
    text: Text {
        sections: vec![BUTTON_SECTIONS(fonts, value)],
        alignment: TextAlignment::Center,
        linebreak_behaviour: BreakLineOn::WordBoundary,
    },
    ..Default::default()
};

const BUTTON_SECTIONS: fn(&Res<FontAssets>, String) -> TextSection = |fonts, value| TextSection {
    value,
    style: TextStyle {
        font: fonts.regular.clone(),
        font_size: 48.0,
        color: TEXT_COLOUR,
    },
};
