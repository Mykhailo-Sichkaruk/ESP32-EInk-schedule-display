use embedded_graphics::pixelcolor::PixelColor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnifiedColor {
    Black,
    White,
    Chromatic,
}

impl UnifiedColor {
    pub fn into_with<F, C>(self, color_converter: F) -> C
    where
        F: Fn(UnifiedColor) -> C,
        C: PixelColor,
    {
        color_converter(self)
    }
}
