use iced::Color;

use crate::colors;

#[derive(Debug, Clone, PartialEq)]
pub struct Palette(pub Color, pub Color, pub Color, pub Color, pub Color, pub Color, pub Color, pub Color, pub Color, pub Color);

#[derive(Debug, Clone, PartialEq)]
pub enum Shade {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9
}

impl Palette {
    pub fn from_shade(&self, shade: &Shade) -> Color {
        match shade {
            Shade::_0 => self.0,
            Shade::_1 => self.1,
            Shade::_2 => self.2,
            Shade::_3 => self.3,
            Shade::_4 => self.4,
            Shade::_5 => self.5,
            Shade::_6 => self.6,
            Shade::_7 => self.7,
            Shade::_8 => self.8,
            Shade::_9 => self.9,
        }
    }
}

pub const DARK: Palette = Palette(colors::DARK_0, colors::DARK_1, colors::DARK_2, colors::DARK_3, colors::DARK_4, colors::DARK_5, colors::DARK_6, colors::DARK_7, colors::DARK_8, colors::DARK_9);
pub const GRAY: Palette = Palette(colors::GRAY_0, colors::GRAY_1, colors::GRAY_2, colors::GRAY_3, colors::GRAY_4, colors::GRAY_5, colors::GRAY_6, colors::GRAY_7, colors::GRAY_8, colors::GRAY_9);
pub const RED: Palette = Palette(colors::RED_0, colors::RED_1, colors::RED_2, colors::RED_3, colors::RED_4, colors::RED_5, colors::RED_6, colors::RED_7, colors::RED_8, colors::RED_9);
pub const PINK: Palette = Palette(colors::PINK_0, colors::PINK_1, colors::PINK_2, colors::PINK_3, colors::PINK_4, colors::PINK_5, colors::PINK_6, colors::PINK_7, colors::PINK_8, colors::PINK_9);
pub const GRAPE: Palette = Palette(colors::GRAPE_0, colors::GRAPE_1, colors::GRAPE_2, colors::GRAPE_3, colors::GRAPE_4, colors::GRAPE_5, colors::GRAPE_6, colors::GRAPE_7, colors::GRAPE_8, colors::GRAPE_9);
pub const VIOLET: Palette = Palette(colors::VIOLET_0, colors::VIOLET_1, colors::VIOLET_2, colors::VIOLET_3, colors::VIOLET_4, colors::VIOLET_5, colors::VIOLET_6, colors::VIOLET_7, colors::VIOLET_8, colors::VIOLET_9);
pub const INDIGO: Palette = Palette(colors::INDIGO_0, colors::INDIGO_1, colors::INDIGO_2, colors::INDIGO_3, colors::INDIGO_4, colors::INDIGO_5, colors::INDIGO_6, colors::INDIGO_7, colors::INDIGO_8, colors::INDIGO_9);
pub const BLUE: Palette = Palette(colors::BLUE_0, colors::BLUE_1, colors::BLUE_2, colors::BLUE_3, colors::BLUE_4, colors::BLUE_5, colors::BLUE_6, colors::BLUE_7, colors::BLUE_8, colors::BLUE_9);
pub const CYAN: Palette = Palette(colors::CYAN_0, colors::CYAN_1, colors::CYAN_2, colors::CYAN_3, colors::CYAN_4, colors::CYAN_5, colors::CYAN_6, colors::CYAN_7, colors::CYAN_8, colors::CYAN_9);
pub const TEAL: Palette = Palette(colors::TEAL_0, colors::TEAL_1, colors::TEAL_2, colors::TEAL_3, colors::TEAL_4, colors::TEAL_5, colors::TEAL_6, colors::TEAL_7, colors::TEAL_8, colors::TEAL_9);
pub const GREEN: Palette = Palette(colors::GREEN_0, colors::GREEN_1, colors::GREEN_2, colors::GREEN_3, colors::GREEN_4, colors::GREEN_5, colors::GREEN_6, colors::GREEN_7, colors::GREEN_8, colors::GREEN_9);
pub const LIME: Palette = Palette(colors::LIME_0, colors::LIME_1, colors::LIME_2, colors::LIME_3, colors::LIME_4, colors::LIME_5, colors::LIME_6, colors::LIME_7, colors::LIME_8, colors::LIME_9);
pub const YELLOW: Palette = Palette(colors::YELLOW_0, colors::YELLOW_1, colors::YELLOW_2, colors::YELLOW_3, colors::YELLOW_4, colors::YELLOW_5, colors::YELLOW_6, colors::YELLOW_7, colors::YELLOW_8, colors::YELLOW_9);
pub const ORANGE: Palette = Palette(colors::ORANGE_0, colors::ORANGE_1, colors::ORANGE_2, colors::ORANGE_3, colors::ORANGE_4, colors::ORANGE_5, colors::ORANGE_6, colors::ORANGE_7, colors::ORANGE_8, colors::ORANGE_9);
