pub mod mountain;
// mod plains;
// mod wildlands;

use crate::prelude::*;

use self::mountain::MountainArea;

use super::story::{Character, FontStyle, Story};

pub struct LocationPlugin;

impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Region>()
            .add_event::<ChangeAreaEvent>()
            .add_systems(Update, change_area.run_if(in_state(GameState::InGame)))
            .add_systems(Update, display_area.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Event, Clone)]
pub struct ChangeAreaEvent(pub Region);

fn change_area(mut location: ResMut<Region>, mut events: EventReader<ChangeAreaEvent>) {
    events.iter().for_each(|event| {
        location.change(event.0.clone());
    });
}

fn display_area(mut story: ResMut<Story>, location: ResMut<Region>) {
    if !location.is_changed() {
        return;
    }
    let para = [(location.to_string(), Character::None, FontStyle::None)];
    story.append_paragraph(para.to_vec().into());
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
