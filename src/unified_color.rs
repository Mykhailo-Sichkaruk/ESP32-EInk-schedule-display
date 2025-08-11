#[cfg(not(feature = "wokwi"))]
use epd_waveshare::color::TriColor;

#[cfg(feature = "wokwi")]
use epd_waveshare::color::Color;

use embedded_graphics::pixelcolor::*;

pub enum UnifiedColor {
    Black,
    White,
    Chromatic,
}

impl UnifiedColor {
    #[cfg(feature = "wokwi")]
    pub fn to_color(&self) -> Color {
        match self {
            UnifiedColor::Black => Color::Black,
            UnifiedColor::White => Color::White,
            UnifiedColor::Chromatic => Color::Black,
        }
    }

    #[cfg(not(feature = "wokwi"))]
    pub fn to_color(&self) -> TriColor {
        match self {
            UnifiedColor::Black => TriColor::Black,
            UnifiedColor::White => TriColor::White,
            UnifiedColor::Chromatic => TriColor::Chromatic,
        }
    }

    pub fn from_rgb565(rgb565: Rgb565 /* u16 */) -> Self {
        let raw: u16 = rgb565.into_storage();
        let r = (raw >> 11) & 0x1F;
        let g = (raw >> 5) & 0x3F;
        let b = raw & 0x1F;

        // Simple thresholding to determine color type
        if r > 15 && g > 15 && b > 15 {
            UnifiedColor::Chromatic
        } else if r == 0 && g == 0 && b == 0 {
            UnifiedColor::Black
        } else {
            UnifiedColor::White
        }
    }
}
