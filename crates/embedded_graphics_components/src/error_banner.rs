use embedded_graphics::prelude::{DrawTarget, Point, Primitive, Size};
use embedded_graphics::Drawable;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::PixelColor;

pub struct ErrorBanner<'a, C>
where
    C: PixelColor,
{
    top_left: Point,
    size: Size,
    text_style: MonoTextStyle<'a, C>,
    background: PrimitiveStyle<C>,
    padding: i32,
}

impl<'a, C> ErrorBanner<'a, C>
where
    C: PixelColor,
{
    pub fn new(
        top_left: Point,
        size: Size,
        text_style: MonoTextStyle<'a, C>,
        background: PrimitiveStyle<C>,
        padding: i32,
    ) -> Self {
        Self {
            top_left,
            size,
            text_style,
            background,
            padding,
        }
    }

    pub fn draw<D>(&self, display: &mut D, line1: &str, line2: &str) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        Rectangle::new(self.top_left, self.size)
            .into_styled(self.background)
            .draw(display)?;

        let center_x = self.top_left.x + (self.size.width as i32 / 2);
        let line_height = self.text_style.font.character_size.height as i32;

        let line1_pos = Point::new(center_x, self.top_left.y + self.padding);
        let line2_pos = Point::new(center_x, line1_pos.y + line_height);

        let style = TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Top)
            .build();

        Text::with_text_style(line1, line1_pos, self.text_style, style).draw(display)?;
        Text::with_text_style(line2, line2_pos, self.text_style, style).draw(display)?;

        Ok(())
    }
}
