pub use crate::{
    assets::{
        colours::{
            BACKGROUND_COLOUR, BORDER_COLOUR, BUTTON_CLICKED, BUTTON_COLOUR, BUTTON_HOVERED,
            INNER_COLOUR, TEXT_COLOUR,
        },
        FontAssets, ImageAssets,
    },
    ingame::{
        image::{CurrentImage, ImageDisplay},
        location::ChangeArea,
        story::{dialogue_library::TEST, Character, FontStyle, Story, StoryDisplay},
    },
    mainmenu::ui::{setup, MainMenuButton, MainMenuItems},
    GamePlugins, GameState,
};
pub use bevy::{
    app::{AppExit, PluginGroupBuilder},
    prelude::*,
    text::{BreakLineOn, Font, Text, TextAlignment, TextSection, TextStyle},
    ui::{AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiImage, UiRect, Val},
};
pub use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
pub use bevy_embedded_assets::EmbeddedAssetPlugin;
pub use std::collections::VecDeque;
