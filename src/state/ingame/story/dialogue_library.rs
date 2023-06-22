use super::{Character, FontStyle};

pub const TEST: [(&str, Character, FontStyle); 3] = [
    ("hello? ", Character::None, FontStyle::None),
    ("Does this... ", Character::Player, FontStyle::Bold),
    ("Work?", Character::Victoria, FontStyle::Italic),
];

pub const TEST2: [(&str, Character, FontStyle); 1] =
    [("Surely it does?", Character::None, FontStyle::BoldItalic)];
