use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::prelude::Size;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::Rectangle;
use epd_waveshare::color::Color;
// use epd_waveshare::color::TriColor;
use epd_waveshare::epd7in5_v2::Display7in5;
use epd_waveshare::epd7in5_v2::Epd7in5;
use epd_waveshare::prelude::WaveshareDisplay;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyInputPin;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_sys::esp_deep_sleep_start;
use esp_idf_sys::esp_sleep_enable_ext0_wakeup;
use log::info;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("!!!!!!!!!!!!!!!!!!!!Starting EPD example");

    let peripherals = {
        let this = Peripherals::take();
        match this {
            Ok(t) => t,
            Err(e) => {
                info!("!!!!!!!!!!!!!!!!!!!!Failed to take peripherals: {e}");
                panic!("@@@@@@@@@@@@@@@@@@@@Failed to take peripherals");
            }
        }
    };

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

    std::thread::sleep(std::time::Duration::from_millis(1000));

    let mut spidd = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        mosi,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config::new().baudrate(115200.Hz()),
    )?;
    info!("!!!!!!!!!!!!!!!!!!!!SPI2 driver setup completed");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let mut delay = Delay::new(100);

    let mut display = Display7in5::default();
    let mut epd = {
        let this = Epd7in5::new(
            &mut spidd,
            PinDriver::input(busy_in)?,
            PinDriver::output(dc)?,
            PinDriver::output(rst)?,
            &mut delay,
            100.into(),
        );
        match this {
            Ok(t) => t,
            Err(e) => {
                info!("!!!!!!!!!!!!!!!!!!!!Failed to create Epd7in5 driver: {e}");
                panic!("@@@@@@@@@@@@@@@@@@@@Failed to create Epd7in5 driver");
            }
        }
    };
    //

    info!("!!!!!!!!!!!!!!!!!!!!Drawing completed");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    {
        let this = epd.wake_up(&mut spidd, &mut delay);
        match this {
            Ok(t) => t,
            Err(e) => {
                info!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!Failed to wake up EPD: {e}");
                panic!("@@@@@@@@@@@@@@@@@@@@Failed to wake up EPD");
            }
        }
    };
    info!("!!!!!!!!!!!!!!!!!!!!EPD wake up completed");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    {
        let this = epd.clear_frame(&mut spidd, &mut delay);
        match this {
            Ok(t) => t,
            Err(e) => {
                info!("!!!!!!!!!!!!!!!!!!!!Failed to clear EPD frame: {e}");
                panic!("@@@@@@@@@@@@@@@@@@@@Failed to clear EPD frame");
            }
        }
    };
    info!("!!!!!!!!!!!!!!!!!!!!EPD frame cleared");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // {
    //     let this = display.clear(Color::White);
    //     match this {
    //         Ok(t) => t,
    //         Err(e) => {
    //             info!("!!!!!!!!!!!!!!!!!!!!Failed to clear display: {e}");
    //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to clear display");
    //         }
    //     }
    // };
    // info!("!!!!!!!!!!!!!!!!!!!!Display cleared");
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // let style = PrimitiveStyleBuilder::new()
    //     .fill_color(TriColor::White)
    //     .stroke_width(0)
    //     .build();
    // {
    //     let this = Rectangle::new(Point::new(10, 10), Size::new(60, 40))
    //         .into_styled(style)
    //         .draw(&mut display);
    //     match this {
    //         Ok(t) => t,
    //         Err(e) => {
    //             info!("!!!!!!!!!!!!!!!!!!!!Failed to draw rectangle: {e}");
    //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to draw rectangle");
    //         }
    //     }
    // };
    // info!("!!!!!!!!!!!!!!!!!!!!Rectangle drawn");
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // {
    //     let this = epd.update_frame(&mut spidd, display.buffer(), &mut delay);
    //     match this {
    //         Ok(t) => t,
    //         Err(e) => {
    //             info!("!!!!!!!!!!!!!!!!!!!!Failed to update frame: {e}");
    //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to update frame");
    //         }
    //     }
    // };
    // info!("!!!!!!!!!!!!!!!!!!!!EPD frame updated with rectangle");
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // {
    //     let this = epd.display_frame(&mut spidd, &mut delay);
    //     match this {
    //         Ok(t) => t,
    //         Err(e) => {
    //             info!("!!!!!!!!!!!!!!!!!!!!Failed to display frame: {e}");
    //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to display frame");
    //         }
    //     }
    // };

    // {
    //     let this = epd.sleep(&mut spidd, &mut delay);
    //     match this {
    //         Ok(t) => t,
    //         Err(e) => {
    //             info!("!!!!!!!!!!!!!!!!!!!!Failed to put EPD to sleep: {e}");
    //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to put EPD to sleep");
    //         }
    //     }
    // };

    // info!("!!!!!!!!!!!!!!!!!!!!EPD put to sleep");
    // std::thread::sleep(std::time::Duration::from_millis(1000));

    // // let wakeup_pin = {
    // //     let this = PinDriver::input(peripherals.pins.gpio33);
    // //     match this {
    // //         Ok(t) => t,
    // //         Err(e) => {
    // //             info!("!!!!!!!!!!!!!!!!!!!!Failed to create wakeup pin: {e}");
    // //             panic!("@@@@@@@@@@@@@@@@@@@@Failed to create wakeup pin");
    // //         }
    // //     }
    // // };
    // // unsafe {
    // //     esp_idf_sys::esp_sleep_enable_ext0_wakeup(wakeup_pin.pin(), 0);
    // // }
    // // unsafe {
    // //     esp_sleep_enable_ext0_wakeup(25, 1);
    // //     esp_deep_sleep_start();
    // // }

    Ok(())
}
