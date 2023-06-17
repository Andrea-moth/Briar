pub enum MountainArea {
    Mansion(MansionFloor),
}

enum MansionFloor {
    Attic(AtticRooms),
    FloorTwo(FloorTwoRooms),
    FloorOne(FloorOneRooms),
    Basement(BasementRooms),
}

enum AtticRooms {}

enum FloorTwoRooms {}

enum FloorOneRooms {}

enum BasementRooms {}
