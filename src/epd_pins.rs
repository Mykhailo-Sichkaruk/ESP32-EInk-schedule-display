use esp_idf_hal::{
    gpio::{AnyInputPin, AnyOutputPin},
    peripherals::Peripherals,
    spi::SPI3,
};
use log::info;

pub struct EpdHardwarePins {
    pub spi: SPI3,
    pub sclk: AnyOutputPin,
    pub mosi: AnyOutputPin,
    pub cs: AnyOutputPin,
    pub busy_in: AnyInputPin,
    pub rst: AnyOutputPin,
    pub dc: AnyOutputPin,
    pub pwr: AnyOutputPin,
}

/// Retrieves the hardware pins for the EPD display.
pub fn get_pins() -> anyhow::Result<EpdHardwarePins> {
    let peripherals = Peripherals::take()?;

    let sclk: AnyOutputPin = peripherals.pins.gpio18.into();
    let cs: AnyOutputPin = peripherals.pins.gpio5.into();
    let busy_in: AnyInputPin = peripherals.pins.gpio4.into();
    let pwr: AnyOutputPin = peripherals.pins.gpio2.into();

    let (mosi, rst, dc) = if cfg!(feature = "wokwi") {
        info!("EPD_CONFIG: Using Wokwi pinout for EPD.");
        (
            peripherals.pins.gpio19.into(), // Wokwi MOSI
            peripherals.pins.gpio21.into(), // Wokwi RST
            peripherals.pins.gpio23.into(), // Wokwi DC
        )
    } else {
        info!("EPD_CONFIG: Using Physical hardware pinout for EPD.");
        (
            peripherals.pins.gpio23.into(), // Physical MOSI
            peripherals.pins.gpio16.into(), // Physical RST
            peripherals.pins.gpio17.into(), // Physical DC
        )
    };

    Ok(EpdHardwarePins {
        spi: peripherals.spi3,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    })
}
