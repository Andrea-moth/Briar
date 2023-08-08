pub mod colours;

use crate::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/OpenDyslexic-Regular.otf")]
    pub regular: Handle<Font>,
    #[asset(path = "fonts/OpenDyslexic-Bold.otf")]
    pub bold: Handle<Font>,
    #[asset(path = "fonts/OpenDyslexic-Italic.otf")]
    pub italic: Handle<Font>,
    #[asset(path = "fonts/OpenDyslexic-BoldItalic.otf")]
    pub bold_italic: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/Les.png")]
    pub les: Handle<Image>,
    #[asset(path = "images/map.png")]
    pub map: Handle<Image>,
}
