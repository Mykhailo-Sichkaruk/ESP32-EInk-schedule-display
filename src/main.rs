use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::Size;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use epd_waveshare::color::Color;
use epd_waveshare::epd2in9::Display2in9;
use epd_waveshare::epd2in9::Epd2in9;
use epd_waveshare::prelude::WaveshareDisplay;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_sys::esp_deep_sleep_start;
use esp_idf_sys::esp_deep_sleep_try;
use esp_idf_sys::esp_deep_sleep_try_to_start;
use esp_idf_sys::esp_sleep_enable_ext0_wakeup;
use esp_idf_sys::esp_sleep_enable_gpio_wakeup;
use esp_idf_sys::esp_sleep_enable_timer_wakeup;
use log::info;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    let spi = peripherals.spi3;
    let sclk = peripherals.pins.gpio18;
    let mosi = peripherals.pins.gpio19;
    let cs = peripherals.pins.gpio5;

    let busy_in: AnyInputPin = peripherals.pins.gpio4.into();
    let rst: AnyOutputPin = peripherals.pins.gpio21.into();
    let dc: AnyOutputPin = peripherals.pins.gpio23.into();

    let mut spidd = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config::new().baudrate(115200.Hz().into()),
    )?;
    info!("SPI2 driver setup completed");

    let mut delay = Delay::new(100);

    let mut display = Display2in9::default();
    let mut epd = Epd2in9::new(
        &mut spidd,
        PinDriver::input(busy_in)?,
        PinDriver::output(dc)?,
        PinDriver::output(rst)?,
        &mut delay,
        100.into(),
    )
    .expect("Failed to create Epd7in5 driver");

    info!("Drawing completed");

    epd.wake_up(&mut spidd, &mut delay)
        .expect("Failed to wake up EPD");
    info!("EPD wake up completed");

    epd.clear_frame(&mut spidd, &mut delay)
        .expect("Failed to wake up EPD");
    info!("EPD frame cleared");

    display
        .clear(Color::Black)
        .expect("Failed to clear display");
    info!("Display cleared");

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Color::White)
        .stroke_width(0)
        .build();
    Rectangle::new(Point::new(10, 10), Size::new(60, 40))
        .into_styled(style)
        .draw(&mut display)
        .expect("Failed to draw rectangle");
    info!("Rectangle drawn");

    epd.update_frame(&mut spidd, display.buffer(), &mut delay)
        .expect("Failed to update frame");
    info!("EPD frame updated with rectangle");

    epd.display_frame(&mut spidd, &mut delay)
        .expect("Failed to display frame");

    epd.sleep(&mut spidd, &mut delay)
        .expect("Failed to put EPD to sleep");

    info!("EPD put to sleep");

    let wakeup_pin = PinDriver::input(peripherals.pins.gpio33).expect("wakeup pin sleep");
    unsafe { esp_idf_sys::esp_sleep_enable_ext0_wakeup(wakeup_pin.pin(), 0); }
    unsafe {
        esp_sleep_enable_ext0_wakeup(25, 1);
        esp_deep_sleep_start();
    }

    Ok(())
}
