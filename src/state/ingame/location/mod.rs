use self::mountain::MountainArea;

mod mountain;
mod plains;
mod wildlands;

/// # Region
/// A large area of land, keep it to specific biomes
enum Region {
    Mountains(MountainArea),
    // Wildlands(WildlandsArea),
    // Plains(PlainsArea),
}
