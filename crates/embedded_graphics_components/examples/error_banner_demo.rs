use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::{FONT_6X10, FONT_9X15, FONT_10X20}},
    pixelcolor::Rgb565,
    prelude::{Point, Size},
    primitives::PrimitiveStyleBuilder,
};
use embedded_graphics_components::error_banner::ErrorBanner;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(600, 400));

    // Example 1: Simple error banner
    ErrorBanner::new(
        Point::new(20, 20),
        Size::new(260, 100),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::new(255, 0, 0))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(255, 255, 200))
            .stroke_color(Rgb565::new(255, 0, 0))
            .stroke_width(2)
            .build(),
        10,
    )
    .draw(&mut display, "Network Error", "Failed to fetch schedule")?;

    // Example 2: Warning banner with different style
    ErrorBanner::new(
        Point::new(300, 20),
        Size::new(280, 100),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::new(200, 100, 0))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(255, 200, 100))
            .stroke_color(Rgb565::new(200, 100, 0))
            .stroke_width(2)
            .build(),
        10,
    )
    .draw(&mut display, "Warning:", "Low memory available")?;

    // Example 3: Info banner
    ErrorBanner::new(
        Point::new(20, 140),
        Size::new(260, 120),
        MonoTextStyleBuilder::new()
            .font(&FONT_9X15)
            .text_color(Rgb565::new(0, 0, 200))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(200, 220, 255))
            .stroke_color(Rgb565::new(0, 0, 200))
            .stroke_width(3)
            .build(),
        15,
    )
    .draw(&mut display, "Information", "System updated")?;

    // Example 4: Critical error with larger font
    ErrorBanner::new(
        Point::new(300, 140),
        Size::new(280, 120),
        MonoTextStyleBuilder::new()
            .font(&FONT_9X15)
            .text_color(Rgb565::new(255, 255, 255))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(180, 0, 0))
            .stroke_color(Rgb565::new(255, 255, 255))
            .stroke_width(3)
            .build(),
        12,
    )
    .draw(&mut display, "CRITICAL ERROR", "Authentication failed")?;

    // Example 5: Success message
    ErrorBanner::new(
        Point::new(160, 280),
        Size::new(280, 100),
        MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(Rgb565::new(0, 150, 0))
            .build(),
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(200, 255, 200))
            .stroke_color(Rgb565::new(0, 150, 0))
            .stroke_width(4)
            .build(),
        15,
    )
    .draw(&mut display, "Success!", "Data saved")?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Error Banner Examples", &output_settings);
    window.show_static(&display);

    Ok(())
}
