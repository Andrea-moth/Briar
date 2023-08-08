use crate::prelude::*;

pub const TEST: [(&str, Character, FontStyle); 4] = [
    ("hello? ", Character::None, FontStyle::None),
    ("Does this... ", Character::Player, FontStyle::Bold),
    ("Work?", Character::Victoria, FontStyle::Italic),
    ("Please", Character::None, FontStyle::BoldItalic),
];
