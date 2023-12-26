//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use std::hash::{Hash, Hasher};

use iced::Color;
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};

use crate::gui::styles::style_constants::{NIGHT_PALETTE, SARASA_MONO, SARASA_MONO_BOLD};
use crate::gui::styles::types::color_remote::color_hash;
use crate::gui::styles::types::palette_extension::PaletteExtension;

use super::color_remote::{deserialize_color, serialize_color};

/// Set of colors to apply to GUI
///
/// Best practices:
/// - `primary` should be a kind of neutral color
/// - `secondary` and `outgoing` should be complementary colors if possible
/// - `text_headers` should be black or white and must have a strong contrast with `secondary`
/// - `text_body` should be black or white and must have a strong contrast with `primary`
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Palette {
    /// Main color of the GUI (background, hovered buttons, active tab)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub primary: Color,
    /// Secondary color of the GUI (incoming connections, header, footer, buttons' borders, radio selection)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub secondary: Color,
    /// Color of outgoing connections
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub outgoing: Color,
    /// Color of favorites' star symbol
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub starred: Color,
    /// Color of header and footer text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_headers: Color,
    /// Color of body and buttons text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_body: Color,
}

impl Palette {
    pub fn generate_buttons_color(self) -> Color {
        let primary = self.primary;
        let is_nightly = primary.r + primary.g + primary.b <= 1.5;
        if is_nightly {
            Color {
                r: f32::min(primary.r + 0.15, 1.0),
                g: f32::min(primary.g + 0.15, 1.0),
                b: f32::min(primary.b + 0.15, 1.0),
                a: 1.0,
            }
        } else {
            Color {
                r: f32::max(primary.r - 0.15, 0.0),
                g: f32::max(primary.g - 0.15, 0.0),
                b: f32::max(primary.b - 0.15, 0.0),
                a: 1.0,
            }
        }
    }

    pub fn generate_palette_extension(self) -> PaletteExtension {
        let primary = self.primary;
        let text_body = self.text_body;
        let text_headers = self.text_headers;
        let is_text_body_dark = text_body.r + text_body.g + text_body.b <= 1.5;
        let is_text_headers_dark = text_headers.r + text_headers.g + text_headers.b <= 1.5;

        let is_nightly = primary.r + primary.g + primary.b <= 1.5;
        let font = if is_text_body_dark {
            SARASA_MONO_BOLD
        } else {
            SARASA_MONO
        };
        let font_headers = if is_text_headers_dark {
            SARASA_MONO_BOLD
        } else {
            SARASA_MONO
        };
        let alpha_chart_badge = if is_nightly { 0.15 } else { 0.75 };
        let alpha_round_borders = if is_nightly { 0.3 } else { 0.6 };
        let alpha_round_containers = if is_nightly { 0.12 } else { 0.24 };
        let buttons_color = self.generate_buttons_color();
        println!("ciao");
        PaletteExtension {
            is_nightly,
            font,
            font_headers,
            alpha_chart_badge,
            alpha_round_borders,
            alpha_round_containers,
            buttons_color,
        }
    }
}

impl Hash for Palette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Palette {
            primary,
            secondary,
            outgoing,
            starred,
            text_headers,
            text_body,
        } = self;

        color_hash(*primary, state);
        color_hash(*secondary, state);
        color_hash(*outgoing, state);
        color_hash(*starred, state);
        color_hash(*text_headers, state);
        color_hash(*text_body, state);
    }
}

pub fn to_rgb_color(color: Color) -> RGBColor {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if color.r <= 1.0
        && color.r >= 0.0
        && color.g <= 1.0
        && color.g >= 0.0
        && color.b <= 1.0
        && color.b >= 0.0
    {
        RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    } else {
        RGBColor(0, 0, 0) // Black
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: (color_1.r + color_2.r) / 2.0,
        g: (color_1.g + color_2.g) / 2.0,
        b: (color_1.b + color_2.b) / 2.0,
        a: 1.0,
    }
}

impl Default for Palette {
    fn default() -> Self {
        NIGHT_PALETTE
    }
}

#[cfg(test)]
mod tests {
    use iced::Color;

    use crate::gui::styles::style_constants::{SARASA_MONO, SARASA_MONO_BOLD};
    use crate::gui::styles::types::palette::Palette;
    use crate::gui::styles::types::palette_extension::PaletteExtension;

    #[test]
    fn test_generate_palette_extension_dark() {
        let palette = Palette {
            primary: Color {
                r: 1.0,
                g: 0.4,
                b: 0.05,
                a: 1.0,
            },
            secondary: Color {
                r: 0.7,
                g: 0.9,
                b: 0.5,
                a: 1.0,
            },
            outgoing: Color {
                r: 0.9,
                g: 0.5,
                b: 0.7,
                a: 1.0,
            },
            starred: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 0.5,
            },
            text_headers: Color {
                r: 0.0,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            text_body: Color {
                r: 0.5,
                g: 0.5,
                b: 0.51,
                a: 1.0,
            },
        };

        assert_eq!(
            palette.generate_palette_extension(),
            PaletteExtension {
                is_nightly: true,
                font: SARASA_MONO,
                font_headers: SARASA_MONO_BOLD,
                alpha_chart_badge: 0.15,
                alpha_round_borders: 0.3,
                alpha_round_containers: 0.12,
                buttons_color: Color {
                    r: 1.0,
                    g: 0.55,
                    b: 0.2,
                    a: 1.0
                }
            }
        )
    }

    #[test]
    fn test_generate_palette_extension_light() {
        let palette = Palette {
            primary: Color {
                r: 1.0,
                g: 0.9,
                b: 0.05,
                a: 1.0,
            },
            secondary: Color {
                r: 0.7,
                g: 0.9,
                b: 0.5,
                a: 1.0,
            },
            outgoing: Color {
                r: 0.9,
                g: 0.5,
                b: 0.7,
                a: 1.0,
            },
            starred: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 0.5,
            },
            text_headers: Color {
                r: 0.7,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            text_body: Color {
                r: 1.0,
                g: 0.9,
                b: 0.4,
                a: 1.0,
            },
        };

        assert_eq!(
            palette.generate_palette_extension(),
            PaletteExtension {
                is_nightly: false,
                font: SARASA_MONO,
                font_headers: SARASA_MONO_BOLD,
                alpha_chart_badge: 0.75,
                alpha_round_borders: 0.6,
                alpha_round_containers: 0.24,
                buttons_color: Color {
                    r: 0.85,
                    g: 0.75,
                    b: 0.0,
                    a: 1.0
                }
            }
        )
    }
}
