use self::mansion::Mansion;

pub mod mansion;

#[derive(Debug)]
pub enum MountainArea {
    Mansion(Mansion),
}
impl Default for MountainArea {
    fn default() -> Self {
        Self::Mansion(Mansion::default())
    }
}
