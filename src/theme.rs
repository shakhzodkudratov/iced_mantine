// type Theme: Default + DefaultStyle;
// https://mantine.dev/theming/theme-object/

use iced::Theme;

use crate::{palettes::{self, Palette}, utils};


#[derive(Debug, Clone, PartialEq)]
pub struct MantineTheme {
    pub mode: Mode,
    /// rem units scale, change if you customize font-size of [`Widget`] element
    /// default value is `1` (for `100%`/`16px` font-size on [`Widget`])
    pub scale: f32,
    pub primary_palette: Palette,
}

impl MantineTheme {
    pub fn from_rem(&self, rem: f32) -> f32 {
        utils::rem_to_px(self.scale, rem)
    }
}

impl Default for MantineTheme {
    fn default() -> Self {
        use once_cell::sync::Lazy;

        static DEFAULT: Lazy<Mode> =
            Lazy::new(|| match dark_light::detect().unwrap_or(dark_light::Mode::Unspecified) {
                dark_light::Mode::Dark => Mode::Dark,
                dark_light::Mode::Light | dark_light::Mode::Unspecified => {
                    Mode::Light
                }
            });

        Self { mode: DEFAULT.clone(), scale: 16.0, primary_palette: palettes::BLUE }
    }
}


// use iced::Theme;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Light,
    Dark,
}

impl From<&Theme> for Mode {
    fn from(theme: &Theme) -> Self {
        match theme {
            Theme::Light => Mode::Light,
            _ => Mode::Dark,
        }
    }
}
