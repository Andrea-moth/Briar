use super::ChangeArea;

#[derive(Debug)]
pub enum WildlandsArea {
    Village(VillageArea),
    PlainsEdge,
    MountainEdge,
}
impl Default for WildlandsArea {
    fn default() -> Self {
        Self::Village(VillageArea::default())
    }
}

#[derive(Default, Debug)]
pub enum VillageArea {
    #[default]
    Auction,
    Stream,
    Outside,
}
