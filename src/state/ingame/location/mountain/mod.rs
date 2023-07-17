pub mod mansion;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum MountainArea {
    Mansion(Mansion),
}
impl Default for MountainArea {
    fn default() -> Self {
        Self::Mansion(Mansion::default())
    }
}
