use embedded_graphics::pixelcolor::PixelColor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnifiedColor {
    Black,
    White,
    Chromatic,
}

pub trait IntoPixelColorConverter {
    type Output: PixelColor;

    fn convert(color: UnifiedColor) -> Self::Output;
}
