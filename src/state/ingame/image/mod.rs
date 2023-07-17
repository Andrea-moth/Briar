use crate::prelude::*;

#[derive(Component)]
pub struct ImageDisplay;

#[derive(Resource, Default)]
struct CurrentImage(Handle<Image>);

#[derive(Component)]
#[allow(dead_code)]
pub enum ImageEvent {
    Set(Handle<Image>),
}

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CurrentImage>()
            .add_event::<ImageEvent>()
            .add_system(update_image.in_set(OnUpdate(GameState::InGame)))
            .add_system(update_image_display.in_set(OnUpdate(GameState::InGame)));
    }
}

fn update_image(mut image_event: EventReader<ImageEvent>, mut image: ResMut<CurrentImage>) {
    for event in image_event.iter() {
        match event {
            ImageEvent::Set(new_image) => image.0 = new_image.clone(),
        }
    }
}

fn update_image_display(
    mut image_query: Query<&mut UiImage, With<ImageDisplay>>,
    image: Res<CurrentImage>,
) {
    for mut new_image in &mut image_query {
        *new_image = image.0.clone().into();
    }
}
