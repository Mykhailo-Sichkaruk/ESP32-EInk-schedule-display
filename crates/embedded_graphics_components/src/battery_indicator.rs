use embedded_graphics::mono_font::ascii::FONT_4X6;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::Text;

use crate::unified_color::UnifiedColor;

pub struct BatteryIndicator {
    top_left: Point,
    size: Size,
}

impl BatteryIndicator {
    pub fn new(top_left: Point, size: Size) -> Self {
        // Убрал num_segments
        BatteryIndicator { top_left, size }
    }

    pub fn draw<D, Color>(&self, display: &mut D, battery_level_percent: u8) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Color>,
        Color: PixelColor + std::convert::From<UnifiedColor>,
    {
        // 1. Рисуем фоновый белый прямоугольник (пустая часть батареи)
        // Это по сути стирает предыдущее состояние полосы перед отрисовкой нового.
        Rectangle::new(self.top_left, self.size)
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(UnifiedColor::White.into())
                    .build(),
            )
            .draw(display)?;

        // 2. Рассчитываем ширину заполненной части
        let filled_width = (self.size.width as f32 * (battery_level_percent as f32 / 100.0)) as u32;

        // 3. Определяем цвет заполнения
        let fill_color = if battery_level_percent <= 20 {
            UnifiedColor::Chromatic.into() // Красный для низкого заряда
        } else {
            UnifiedColor::Black.into() // Черный для нормального заряда
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
            let text_style_black: MonoTextStyle<Color> = MonoTextStyleBuilder::new()
                .font(&FONT_4X6)
                .text_color(UnifiedColor::Chromatic.into())
                .build();

            let text = format!("{battery_level_percent}%");

            Text::new(
                &text,
                // draw after the filled part
                self.top_left
                    + Point::new(
                        filled_width as i32 + 2,                       // 2 pixels padding
                        FONT_4X6.character_size.height as i32 / 2 + 1, // Center vertically
                    ),
                text_style_black,
            )
            .draw(display)?;
        }

        Ok(())
    }
}
