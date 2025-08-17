use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};

use crate::unified_color::{AnyColor, UnifiedColor};
use esp_backtrace as _;

pub struct BatteryIndicator {
    top_left: Point,
    size: Size,
}

impl BatteryIndicator {
    pub fn new(top_left: Point, size: Size) -> Self {
        // Убрал num_segments
        BatteryIndicator { top_left, size }
    }

    pub fn draw<D>(&self, display: &mut D, battery_level_percent: u8) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = AnyColor>,
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

        // Текстовое значение процента удалено, так как "точность не важна" и данные уходят по Wi-Fi.

        Ok(())
    }
}
