use bevy::{
    prelude::{
        Component, EventReader, Handle, Image as BevyImage, IntoSystemConfig, OnUpdate, Plugin,
        Query, Res, ResMut, Resource, With,
    },
    ui::UiImage,
};

use crate::state::GameState;

#[derive(Component)]
pub struct ImageDisplay;

#[derive(Resource, Default)]
struct Image(Handle<BevyImage>);

#[derive(Component)]
pub enum ImageEvent {
    Set(Handle<BevyImage>),
}

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Image>()
            .add_event::<ImageEvent>()
            .add_system(update_image.in_set(OnUpdate(GameState::InGame)))
            .add_system(update_image_display.in_set(OnUpdate(GameState::InGame)));
    }
}

fn update_image(mut image_event: EventReader<ImageEvent>, mut image: ResMut<Image>) {
    for event in image_event.iter() {
        match event {
            ImageEvent::Set(new_image) => image.0 = new_image.clone(),
        }
    }
}

fn update_image_display(
    mut image_query: Query<&mut UiImage, With<ImageDisplay>>,
    image: Res<Image>,
) {
    for mut new_image in &mut image_query {
        *new_image = image.0.clone().into();
    }
}
