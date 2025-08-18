use std::marker::PhantomData;

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::{Baseline, Text};

use crate::unified_color::{IntoPixelColorConverter, UnifiedColor};

pub struct BatteryIndicator<T>
where
    T: IntoPixelColorConverter,
    T::Output: PixelColor,
{
    top_left: Point,
    size: Size,
    _phantom_converter: PhantomData<T>,
}

impl<T> BatteryIndicator<T>
where
    T: IntoPixelColorConverter,
    T::Output: PixelColor,
{
    pub fn new(top_left: Point, size: Size) -> Self {
        BatteryIndicator {
            top_left,
            size,
            _phantom_converter: PhantomData,
        }
    }

    pub fn draw<D>(&self, display: &mut D, battery_level_percent: u8) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = T::Output>,
    {
        // 1. Рисуем фоновый белый прямоугольник (пустая часть батареи)
        // Это по сути стирает предыдущее состояние полосы перед отрисовкой нового.
        Rectangle::new(self.top_left, self.size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(T::convert(UnifiedColor::White))
                    .build(),
            )
            .draw(display)?;

        // 2. Рассчитываем ширину заполненной части
        let filled_width = (self.size.width as f32 * (battery_level_percent as f32 / 100.0)) as u32;

        // 3. Определяем цвет заполнения
        let fill_color = if battery_level_percent <= 20 {
            T::convert(UnifiedColor::Chromatic) // Красный для низкого заряда
        } else {
            T::convert(UnifiedColor::Black) // Черный для нормального заряда
        };

        // 4. Рисуем заполненный прямоугольник
        if filled_width > 0 {
            // Рисуем только если есть что заполнять
            Rectangle::new(self.top_left, Size::new(filled_width, self.size.height))
                .into_styled(PrimitiveStyleBuilder::new().fill_color(fill_color).build())
                .draw(display)?;
        }

        // display power text only if battery level is low
        if battery_level_percent <= 20 {
            let text_style_black: MonoTextStyle<T::Output> = MonoTextStyleBuilder::new()
                .font(&FONT_6X10)
                .text_color(T::convert(UnifiedColor::Chromatic))
                .build();

            let text = format!("{battery_level_percent}%");

            Text::with_baseline(
                &text,
                // draw after the filled part
                self.top_left + Point::new(filled_width as i32 + 2, self.size.height as i32 / 2),
                text_style_black,
                Baseline::Middle,
            )
            .draw(display)?;
        }

        Ok(())
    }
}
