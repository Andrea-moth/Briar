use bevy::prelude::{Font, Resource, World, Mut, AssetServer, HandleUntyped, Handle, Color};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct OpenDyslexicFonts {
    #[asset(path = "fonts/OpenDyslexic-Regular.otf")]
    pub regular: Handle<Font>
    
    //#[asset(path="fonts", collection(typed))]
    //pub fonts: Vec<Handle<Font>>
}

pub const WIREFRAME_COLOUR: Color = Color::rgb(91.0/255.0, 47.0/255.0, 135.0/255.0);
pub const BACKGROUND_COLOUR: Color = Color::rgb(15.0/255.0, 42.0/255.0, 63.0/255.0);
pub const TEXTCOLOUR: Color = Color::WHITE;