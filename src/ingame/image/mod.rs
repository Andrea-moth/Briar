use crate::prelude::*;

#[derive(Component)]
pub struct ImageDisplay;

#[derive(Resource, Default)]
pub struct CurrentImage(Handle<Image>);

impl CurrentImage {
    pub fn set(&mut self, new: Handle<Image>) {
        self.0 = new;
    }
}

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CurrentImage>().add_systems(
            Update,
            update_image_display.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_image_display(
    mut image_query: Query<&mut UiImage, With<ImageDisplay>>,
    current_image: Res<CurrentImage>,
) {
    image_query.iter_mut().for_each(|mut image| {
        *image = current_image.0.clone().into();
    });
}
