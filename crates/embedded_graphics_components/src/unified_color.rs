use embedded_graphics::pixelcolor::{PixelColor, Rgb565};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnifiedColor {
    Black,
    White,
    Chromatic,
}

pub trait IntoWith<F, C> {
    fn into_with(self, color_converter: F) -> C;
}

impl<F, C> IntoWith<F, C> for UnifiedColor
where
    F: Fn(UnifiedColor) -> C,
    C: PixelColor,
{
    fn into_with(self, color_converter: F) -> C {
        color_converter(self)
    }
}

impl From<UnifiedColor> for Rgb565 {
    fn from(color: UnifiedColor) -> Self {
        match color {
            UnifiedColor::Black => Rgb565::new(0, 0, 0),
            UnifiedColor::White => Rgb565::new(255, 255, 255),
            UnifiedColor::Chromatic => Rgb565::new(255, 0, 0),
        }
    }
}
