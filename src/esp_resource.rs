use esp_idf_hal::{
    gpio::{AnyInputPin, AnyOutputPin},
    modem::Modem,
    peripherals::Peripherals,
    spi::SPI3,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault},
};

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

pub struct NetParts {
    pub modem: Modem,
    pub sysloop: EspSystemEventLoop,
}

pub fn get() -> (EpdHardwarePins, NetParts, EspNvsPartition<NvsDefault>) {
    let peripherals = Peripherals::take().expect("Failed to take peripherals");

    let modem = peripherals.modem;
    let sysloop = EspSystemEventLoop::take().expect("Failed to take system event loop");
    let sclk: AnyOutputPin = peripherals.pins.gpio18.into();
    let cs: AnyOutputPin = peripherals.pins.gpio5.into();
    let busy_in: AnyInputPin = peripherals.pins.gpio4.into();
    let pwr: AnyOutputPin = peripherals.pins.gpio2.into();

    let mosi = peripherals.pins.gpio23.into(); // Physical MOSI
    let rst = peripherals.pins.gpio16.into(); // Physical RST
    let dc = peripherals.pins.gpio17.into(); // Physical DC

    let epd = EpdHardwarePins {
        spi: peripherals.spi3,
        sclk,
        mosi,
        cs,
        busy_in,
        rst,
        dc,
        pwr,
    };
    let net_parts = NetParts { modem, sysloop };

    let nvs = EspDefaultNvsPartition::take().expect("Failed to take default NVS partition");

    (epd, net_parts, nvs)
}
