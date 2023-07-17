use crate::prelude::*;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Character {
    None,
    Player,
    Victoria,
}
impl Into<Color> for Character {
    fn into(self) -> Color {
        match self {
            Character::None => TEXT_COLOUR,
            Character::Player => Color::ALICE_BLUE,
            Character::Victoria => Color::MIDNIGHT_BLUE,
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum FontStyle {
    None,
    Bold,
    Italic,
    BoldItalic,
}
impl FontStyle {
    pub fn into_font(self, fonts: &Res<FontAssets>) -> Handle<Font> {
        match self {
            Self::None => fonts.regular.clone(),
            Self::Italic => fonts.italic.clone(),
            Self::Bold => fonts.bold.clone(),
            Self::BoldItalic => fonts.bold_italic.clone(),
        }
    }
}
