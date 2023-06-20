mod mountain;
// mod plains;
// mod wildlands;

use self::mountain::MountainArea;
use bevy::prelude::{Plugin, Resource};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Region>();
    }
}

trait ChangeArea
where
    Self: Sized,
{
    fn check_change(&self, new: &Self) -> bool;
    fn change(&mut self, new: Self) {
        if !self.check_change(&new) {
            return;
        }
        *self = new;
    }
}

/// # Region
/// A large area of land, keep it to specific biomes
#[derive(Resource, Debug)]
enum Region {
    Mountains(MountainArea),
    // Plains(PlainsArea),
}
impl Default for Region {
    fn default() -> Self {
        Self::Mountains(MountainArea::default())
    }
}
impl ToString for Region {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}
