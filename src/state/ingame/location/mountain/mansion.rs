#![allow(dead_code)]

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Mansion {
    Attic,
    FloorTwo(FloorTwo),
    FloorOne(FloorOne),
    Ground(Ground),
    Basement(Basement),
    Gardens(Gardens),
}
impl Default for Mansion {
    fn default() -> Self {
        Self::FloorOne(FloorOne::ServantsRoom)
    }
}
impl ChangeArea for Mansion {
    fn check_change(&self, new: &Self) -> bool {
        matches!(
            (self, new),
            (&Self::Attic, &Self::FloorTwo(FloorTwo::Stairs))
                | (
                    &Self::FloorTwo(FloorTwo::Stairs),
                    &Self::Attic | &Self::FloorOne(FloorOne::Stairs),
                )
                | (
                    &Self::FloorOne(FloorOne::Stairs),
                    &Self::FloorTwo(FloorTwo::Stairs) | Self::Ground(Ground::Stairs),
                )
                | (
                    &Self::Ground(Ground::Stairs),
                    &Self::Basement(Basement::MainRoom) | &Self::FloorOne(FloorOne::Stairs),
                )
                | (
                    &Self::Ground(Ground::Entrance | Ground::LivingRoom),
                    &Self::Gardens(Gardens::Grounds),
                )
                | (
                    &Self::Basement(Basement::MainRoom),
                    &Self::Ground(Ground::Stairs)
                )
                | (
                    &Self::Basement(Basement::MerchantsEntrance),
                    &Self::Gardens(Gardens::Entrance)
                )
                | (
                    &Self::Gardens(Gardens::Grounds),
                    &Self::Ground(Ground::LivingRoom | Ground::Entrance),
                )
        )
    }
}

#[derive(Debug, Clone)]
pub enum FloorTwo {
    Stairs,
    Library,
}
impl ChangeArea for FloorTwo {
    fn check_change(&self, _new: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub enum FloorOne {
    Stairs,
    VictoriasRoom,
    ServantsRoom,
    Hallway,
}
impl ChangeArea for FloorOne {
    fn check_change(&self, new: &Self) -> bool {
        matches!(
            (self, new),
            (&Self::Stairs, &Self::Hallway)
                | (
                    &Self::Hallway,
                    &Self::ServantsRoom | &Self::VictoriasRoom | &Self::Stairs
                )
                | (&Self::VictoriasRoom, &Self::Hallway)
                | (&Self::ServantsRoom, &Self::Hallway)
        )
    }
}

#[derive(Debug, Clone)]
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
        matches!(
            (self, new),
            (&Self::Stairs, &Self::Hallway)
                | (
                    &Self::Hallway,
                    &Self::Storage
                        | &Self::Entrance
                        | &Self::Kitchen
                        | &Self::LivingRoom
                        | &Self::Stairs,
                )
                | (&Self::Kitchen, &Self::LivingRoom | &Self::Hallway)
                | (&Self::LivingRoom, &Self::Kitchen | &Self::Hallway)
                | (&Self::Entrance, &Self::Hallway)
                | (&Self::Storage, &Self::Hallway)
        )
    }
}

#[derive(Debug, Clone)]
pub enum Basement {
    MainRoom,
    MerchantsEntrance,
}
impl ChangeArea for Basement {
    fn check_change(&self, _new: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub enum Gardens {
    Entrance,
    Grounds,
    GreenHouse,
    GardenHouse,
}
impl ChangeArea for Gardens {
    fn check_change(&self, new: &Self) -> bool {
        matches!(
            (self, new),
            (
                &Self::Grounds,
                &Self::Entrance | &Self::GreenHouse | &Self::GardenHouse
            ) | (&Self::Entrance, &Self::Grounds)
                | (&Self::GardenHouse, &Self::Grounds | &Self::GreenHouse)
                | (&Self::GreenHouse, &Self::Grounds | &Self::GardenHouse)
        )
    }
}
