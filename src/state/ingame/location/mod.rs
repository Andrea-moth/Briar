pub mod mountain;
// mod plains;
// mod wildlands;

use crate::prelude::*;

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Region>()
            .add_event::<ChangeAreaEvent>()
            .add_system(change_area.in_set(OnUpdate(GameState::InGame)));
    }
}

#[derive(Component, Clone)]
pub struct ChangeAreaEvent(pub Region);

fn change_area(mut location: ResMut<Region>, mut events: EventReader<ChangeAreaEvent>) {
    for event in events.iter() {
        location.change(event.0.clone());
    }
}

pub trait ChangeArea
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
/// A large area of land, keep it to specific places
#[derive(Resource, Debug, Clone)]
pub enum Region {
    Mountains(MountainArea),
    // Plains(PlainsArea),
}
impl ChangeArea for Region {
    fn check_change(&self, _new: &Self) -> bool {
        true
    }
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
