use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::Size;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
// use epd_waveshare::color::Color;
use epd_waveshare::color::TriColor;
use epd_waveshare::epd7in5b_v3::Display7in5;
use epd_waveshare::epd7in5b_v3::Epd7in5;
use epd_waveshare::prelude::WaveshareDisplay;
use esp_backtrace as _;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyInputPin;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_sys::esp_deep_sleep;
use esp_idf_sys::esp_sleep_enable_ext0_wakeup;
use log::info;

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder};
use embedded_graphics::text::{Text, TextStyle, TextStyleBuilder};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting EPD example");

    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    let spi = peripherals.spi3;
    let sclk = peripherals.pins.gpio18;
    let mosi = peripherals.pins.gpio23;
    let cs = peripherals.pins.gpio5;

    let busy_in: AnyInputPin = peripherals.pins.gpio4.into();
    let rst: AnyOutputPin = peripherals.pins.gpio16.into();
    let dc: AnyOutputPin = peripherals.pins.gpio17.into();

    let pwr: AnyOutputPin = peripherals.pins.gpio2.into();
    let mut pwr = PinDriver::output(pwr)?;
    pwr.set_high()?;

    let mut spidd = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config::new().baudrate(115200.Hz()),
    )?;

    let mut delay = Delay::new(100);

    let mut epd = Epd7in5::new(
        &mut spidd,
        PinDriver::input(busy_in)?,
        PinDriver::output(dc)?,
        PinDriver::output(rst)?,
        &mut delay,
        None,
    )
    .expect("Failed to create EPD instance");

    epd.wake_up(&mut spidd, &mut delay)
        .expect("Failed to wake up EPD");

    epd.clear_frame(&mut spidd, &mut delay)
        .expect("Failed to clear EPD frame");

    let mut display = Box::new(Display7in5::default());

    let mut counter = 0;
    loop {
        info!("Drawing frame, counter: {counter}");
        display
            .clear(TriColor::White)
            .expect("Failed to clear display buffer");

        // ===== Rectangle
        // let style = PrimitiveStyleBuilder::new()
        //     .fill_color(TriColor::Black)
        //     .stroke_width(0)
        //     .build();

        // Rectangle::new(Point::new(10, 10), Size::new(60, 40))
        //     .into_styled(style)
        //     .draw(display.as_mut())
        //     .expect("Failed to draw rectangle");
        // !===== Rectangle

        // ===== Text
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(TriColor::Chromatic)
            .build();

        Text::new(&format!("Count: {counter}"), Point::new(10, 20), text_style)
            .draw(display.as_mut())
            .expect("Failed to draw text");

        epd.update_and_display_frame(&mut spidd, display.buffer(), &mut delay)
            .expect("Failed to update and display EPD frame");

        info!("Frame updated and displayed, counter: {counter}");
        counter += 1;

        delay.delay_ms(1000);
    }
    epd.sleep(&mut spidd, &mut delay)
        .expect("Failed to put EPD to sleep");

    // info!("Configuring wakeup pin");
    // let wakeup_pin =
    //     PinDriver::input(peripherals.pins.gpio33).expect("Failed to create wakeup pin");
    // unsafe {
    //     esp_idf_sys::esp_sleep_enable_ext0_wakeup(wakeup_pin.pin(), 0);
    // }
    // info!("Wakeup pin configured");
    // info!("Entering deep sleep");
    // unsafe {
    //     esp_sleep_enable_ext0_wakeup(25, 1);
    //     esp_deep_sleep(20_000_000);
    // }

    Ok(())
}
