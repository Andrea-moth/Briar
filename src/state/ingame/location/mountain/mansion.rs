#![allow(dead_code)]
use crate::state::ingame::location::ChangeArea;

#[derive(Default, Debug)]
pub enum Mansion {
    #[default]
    Attic,
    FloorTwo(FloorTwo),
    FloorOne(FloorOne),
    Ground(Ground),
    Basement(Basement),
    Gardens(Gardens),
}
impl ChangeArea for Mansion {
    fn check_change(&self, new: &Self) -> bool {
        match (self, new) {
            (&Self::Attic, &Self::FloorTwo(FloorTwo::Stairs)) => true,
            (
                &Self::FloorTwo(FloorTwo::Stairs),
                &Self::Attic | &Self::FloorOne(FloorOne::Stairs),
            ) => true,
            (
                &Self::FloorOne(FloorOne::Stairs),
                &Self::Basement(Basement::MainRoom) | Self::Ground(Ground::Stairs),
            ) => true,
            (
                &Self::Ground(Ground::Stairs),
                &Self::Basement(Basement::MainRoom) | &Self::FloorOne(FloorOne::Stairs),
            ) => true,
            (
                &Self::Ground(Ground::Entrance | Ground::LivingRoom),
                &Self::Gardens(Gardens::Grounds),
            ) => true,
            (&Self::Basement(Basement::MainRoom), &Self::Ground(Ground::Stairs)) => true,
            (&Self::Basement(Basement::MerchantsEntrance), &Self::Gardens(Gardens::Entrance)) => {
                true
            }
            (
                &Self::Gardens(Gardens::Grounds),
                &Self::Ground(Ground::LivingRoom | Ground::Entrance),
            ) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum FloorTwo {
    Stairs,
    Library,
}
impl ChangeArea for FloorTwo {
    fn check_change(&self, _new: &Self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum FloorOne {
    Stairs,
    VictoriasRoom,
    ServantsRoom,
    Hallway,
}
impl ChangeArea for FloorOne {
    fn check_change(&self, new: &Self) -> bool {
        match (self, new) {
            (&Self::Stairs, &Self::Hallway) => true,
            (&Self::Hallway, &Self::ServantsRoom | &Self::VictoriasRoom | &Self::Stairs) => true,
            (&Self::VictoriasRoom, &Self::Hallway) => true,
            (&Self::ServantsRoom, &Self::Hallway) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Ground {
    Stairs,
    Hallway,
    Storage,
    Entrance,
    Kitchen,
    LivingRoom,
}
impl ChangeArea for Ground {
    fn check_change(&self, new: &Self) -> bool {
        match (self, new) {
            (&Self::Stairs, &Self::Hallway) => true,
            (
                &Self::Hallway,
                &Self::Storage
                | &Self::Entrance
                | &Self::Kitchen
                | &Self::LivingRoom
                | &Self::Stairs,
            ) => true,
            (&Self::Kitchen, &Self::LivingRoom | &Self::Hallway) => true,
            (&Self::LivingRoom, &Self::Kitchen | &Self::Hallway) => true,
            (&Self::Entrance, &Self::Hallway) => true,
            (&Self::Storage, &Self::Hallway) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Basement {
    MainRoom,
    MerchantsEntrance,
}
impl ChangeArea for Basement {
    fn check_change(&self, _new: &Self) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum Gardens {
    Entrance,
    Grounds,
    GreenHouse,
    GardenHouse,
}
impl ChangeArea for Gardens {
    fn check_change(&self, new: &Self) -> bool {
        match (self, new) {
            (&Self::Grounds, &Self::Entrance | &Self::GreenHouse | &Self::GardenHouse) => true,
            (&Self::Entrance, &Self::Grounds) => true,
            (&Self::GardenHouse, &Self::Grounds | &Self::GreenHouse) => true,
            (&Self::GreenHouse, &Self::Grounds | &Self::GardenHouse) => true,
            _ => false,
        }
    }
}
