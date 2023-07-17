pub use crate::state::{
    assets::{FontAssets, ImageAssets},
    colours::{
        BACKGROUND_COLOUR, BORDER_COLOUR, BUTTON_CLICKED, BUTTON_COLOUR, BUTTON_HOVERED,
        INNER_COLOUR, TEXT_COLOUR,
    },
    ingame::{
        image::{ImageDisplay, ImageEvent},
        location::{
            mountain::{
                mansion::{FloorOne, Mansion},
                MountainArea,
            },
            ChangeArea, ChangeAreaEvent, Region,
        },
        story::{
            dialogue_library::{TEST, TEST2},
            prose::StoryEvent,
            style::{Character, FontStyle},
            Story, StoryDisplay,
        },
    },
    mainmenu::ui::{Button as MenuButton, MainMenuItems},
    GamePlugins, GameState,
};
pub use bevy::{
    app::{AppExit, PluginGroupBuilder},
    prelude::{
        AlignItems, App, AssetServer, BackgroundColor, BuildChildren, Button, ButtonBundle,
        Camera2dBundle, Changed, ClearColor, Color, Commands, Component, DefaultPlugins,
        DespawnRecursive, DespawnRecursiveExt, DetectChanges, Entity, EventReader, EventWriter,
        FlexDirection, Font, Handle, Image, ImageBundle, Input, Interaction, IntoSystemAppConfig,
        IntoSystemConfig, JustifyContent, KeyCode, Local, NextState, NodeBundle, OnEnter, OnUpdate,
        Overflow, Plugin, PluginGroup, Query, Res, ResMut, Resource, Size, States, Style, Text,
        TextAlignment, TextBundle, TextSection, TextStyle, UiImage, UiRect, Val, Window,
        WindowPlugin, With,
    },
    text::BreakLineOn,
    window::{WindowMode, WindowResolution},
};
pub use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
pub use std::collections::VecDeque;
