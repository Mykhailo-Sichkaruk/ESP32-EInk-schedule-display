#[cfg(not(feature = "wokwi"))]
use epd_waveshare::color::TriColor;

#[cfg(feature = "wokwi")]
use epd_waveshare::color::Color;

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
}
