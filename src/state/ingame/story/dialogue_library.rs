use super::{Character, Style};

pub const TEST: [(&str, Character, Style); 3] = [
    ("hello? ", Character::None, Style::None),
    ("Does this... ", Character::Player, Style::Bold),
    ("Work?", Character::Victoria, Style::Italic),
];

pub const TEST2: [(&str, Character, Style); 1] =
    [("Surely it does?", Character::None, Style::BoldItalic)];
