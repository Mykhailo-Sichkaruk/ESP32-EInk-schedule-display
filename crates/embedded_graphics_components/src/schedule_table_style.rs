use std::ops::RangeInclusive;

use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_10X20};
// use embedded_graphics::mono_font::ascii::{FONT_7X14, FONT_10X20};
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::prelude::{PixelColor, Size};
use embedded_graphics::primitives::{
    CornerRadii, CornerRadiiBuilder, PrimitiveStyle, PrimitiveStyleBuilder,
};

#[derive(Clone, Copy, Debug)]
pub struct Palette<C>
where
    C: PixelColor,
{
    pub primary: C,
    pub secondary: C,
    pub accent: C,
}

impl<C> Palette<C>
where
    C: PixelColor,
{
    pub fn new(primary: C, secondary: C, accent: C) -> Self {
        Self {
            primary,
            secondary,
            accent,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScheduleTableStyle<'a, C>
where
    C: PixelColor,
{
    /// Main text style
    pub text_body: MonoTextStyle<'a, C>,
    /// Small text style
    pub text_small: MonoTextStyle<'a, C>,
    /// Small text shadow style
    pub text_small_shadow: Option<(
        MonoTextStyle<'a, C>,
        RangeInclusive<i32>,
        RangeInclusive<i32>,
    )>,
    /// Grid line style
    pub grid_line: PrimitiveStyle<C>,
    /// Header line style
    pub header_line: PrimitiveStyle<C>,
    /// Interval box style
    pub interval_box: PrimitiveStyle<C>,
    /// Interval box corner radii
    pub interval_box_corners: CornerRadii,
    /// Interval box margin
    pub interval_box_margin: i32,
    /// Time line style
    pub time_line: PrimitiveStyle<C>,
    /// Background style for the entire widget before rendering
    pub background: PrimitiveStyle<C>,
    /// Border style for the widget
    pub border: PrimitiveStyle<C>,

    pub(crate) _palette: Palette<C>,
}

#[derive(Clone, Debug)]
pub struct ScheduleTableStyleBuilder<'a, C>
where
    C: PixelColor,
{
    style: ScheduleTableStyle<'a, C>,
}

impl<'a, C> ScheduleTableStyleBuilder<'a, C>
where
    C: PixelColor,
{
    pub fn new(palette: Palette<C>) -> Self {
        Self {
            style: ScheduleTableStyle {
                text_body: MonoTextStyleBuilder::new()
                    .font(&FONT_10X20)
                    .text_color(palette.primary)
                    .build(),
                text_small: MonoTextStyleBuilder::new()
                    .font(&FONT_6X12)
                    .text_color(palette.secondary)
                    .build(),
                text_small_shadow: Some((
                    MonoTextStyleBuilder::new()
                        .font(&FONT_6X12)
                        .text_color(palette.primary)
                        .build(),
                    -1..=1,
                    -1..=1,
                )),
                grid_line: PrimitiveStyleBuilder::new()
                    .stroke_color(palette.primary)
                    .stroke_width(1)
                    .build(),
                header_line: PrimitiveStyleBuilder::new()
                    .stroke_color(palette.primary)
                    .stroke_width(2)
                    .build(),
                interval_box: PrimitiveStyleBuilder::new()
                    .stroke_color(palette.primary)
                    .fill_color(palette.secondary)
                    .stroke_width(2)
                    .build(),
                interval_box_corners: CornerRadiiBuilder::new().all(Size::new(10, 10)).build(),
                interval_box_margin: 4,
                time_line: PrimitiveStyleBuilder::new()
                    .stroke_color(palette.accent)
                    .stroke_width(4)
                    .build(),
                background: PrimitiveStyleBuilder::new()
                    .fill_color(palette.secondary)
                    .build(),
                border: PrimitiveStyleBuilder::new()
                    .stroke_color(palette.primary)
                    .stroke_width(4)
                    .build(),
                _palette: palette,
            },
        }
    }

    pub fn text_body(mut self, text_body: MonoTextStyle<'a, C>) -> Self {
        self.style.text_body = text_body;
        self
    }

    pub fn text_small(mut self, text_small: MonoTextStyle<'a, C>) -> Self {
        self.style.text_small = text_small;
        self
    }

    pub fn text_small_shadow(
        mut self,
        text_small_shadow: Option<(
            MonoTextStyle<'a, C>,
            RangeInclusive<i32>,
            RangeInclusive<i32>,
        )>,
    ) -> Self {
        self.style.text_small_shadow = text_small_shadow;
        self
    }

    pub fn grid_line(mut self, grid_line: PrimitiveStyle<C>) -> Self {
        self.style.grid_line = grid_line;
        self
    }

    pub fn header_line(mut self, header_line: PrimitiveStyle<C>) -> Self {
        self.style.header_line = header_line;
        self
    }

    pub fn interval_box(mut self, interval_box: PrimitiveStyle<C>) -> Self {
        self.style.interval_box = interval_box;
        self
    }

    pub fn interval_box_radii(mut self, interval_box_radii: CornerRadii) -> Self {
        self.style.interval_box_corners = interval_box_radii;
        self
    }

    pub fn interval_box_margin(mut self, interval_box_margin: i32) -> Self {
        self.style.interval_box_margin = interval_box_margin;
        self
    }

    pub fn time_line(mut self, time_line: PrimitiveStyle<C>) -> Self {
        self.style.time_line = time_line;
        self
    }

    pub fn background(mut self, background: PrimitiveStyle<C>) -> Self {
        self.style.background = background;
        self
    }

    pub fn border(mut self, border: PrimitiveStyle<C>) -> Self {
        self.style.border = border;
        self
    }

    pub fn build(self) -> ScheduleTableStyle<'a, C> {
        self.style
    }
}
