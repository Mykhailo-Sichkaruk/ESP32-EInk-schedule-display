#[cfg(not(feature = "wokwi"))]
use epd_waveshare::color::TriColor;

#[cfg(feature = "wokwi")]
use epd_waveshare::color::Color as BinColor;

use embedded_graphics::pixelcolor::*;

pub enum UnifiedColor {
    Black,
    White,
    Chromatic,
}
#[cfg(feature = "wokwi")]
pub type AnyColor = BinColor;

#[cfg(not(feature = "wokwi"))]
pub type AnyColor = TriColor;

impl UnifiedColor {
    #[cfg(feature = "wokwi")]
    fn to_color(&self) -> AnyColor {
        match self {
            UnifiedColor::Black => AnyColor::Black,
            UnifiedColor::White => AnyColor::White,
            UnifiedColor::Chromatic => AnyColor::Black,
        }
    }

    #[cfg(not(feature = "wokwi"))]
    fn to_color(&self) -> AnyColor {
        match self {
            UnifiedColor::Black => AnyColor::Black,
            UnifiedColor::White => AnyColor::White,
            UnifiedColor::Chromatic => AnyColor::Chromatic,
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

#[cfg(feature = "wokwi")]
impl Into<AnyColor> for UnifiedColor {
    fn into(self) -> AnyColor {
        self.to_color()
    }
}

#[cfg(not(feature = "wokwi"))]
impl From<UnifiedColor> for AnyColor {
    fn from(val: UnifiedColor) -> Self {
        val.to_color()
    }
}
