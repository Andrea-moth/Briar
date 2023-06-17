use bevy::{
    prelude::{AssetServer, Handle, Image, Resource},
    text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/OpenDyslexic-Regular.otf")]
    pub regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/Les.png")]
    pub les: Handle<Image>,
}
