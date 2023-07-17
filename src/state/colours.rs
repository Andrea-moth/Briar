use crate::prelude::*;

pub const BACKGROUND_COLOUR: Color = Color::rgb(65.0 / 255.0, 60.0 / 255.0, 88.0 / 255.0);

pub const BORDER_COLOUR: BackgroundColor =
    BackgroundColor(Color::rgb(1.0, 184.0 / 255.0, 222.0 / 255.0));
pub const INNER_COLOUR: BackgroundColor = BackgroundColor(BACKGROUND_COLOUR);

pub const TEXT_COLOUR: Color = Color::rgb(119.0 / 255.0, 160.0 / 255.0, 169.0 / 255.0);

pub const BUTTON_COLOUR: BackgroundColor =
    BackgroundColor(Color::rgb(216.0 / 255.0, 30.0 / 255.0, 91.0 / 255.0));
pub const BUTTON_HOVERED: BackgroundColor =
    BackgroundColor(Color::rgb(183.0 / 255.0, 27.0 / 255.0, 79.0 / 255.0));
pub const BUTTON_CLICKED: BackgroundColor =
    BackgroundColor(Color::rgb(130.0 / 255.0, 20.0 / 255.0, 57.0 / 255.0));
