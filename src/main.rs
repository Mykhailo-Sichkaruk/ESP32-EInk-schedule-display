#[cfg(feature = "wokwi")]
use epd_waveshare::epd2in9_v2::{Display2in9 as Display, Epd2in9 as Epd};

use esp_backtrace as _;
use esp_eink_schedule::epd::epd_start_render_text;
use esp_eink_schedule::epd_pins;
use esp_eink_schedule::wifilib::getRequest;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
// use esp_idf_sys::esp_deep_sleep;
// use esp_idf_sys::esp_sleep_enable_ext0_wakeup;
use log::info;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let nvs = EspDefaultNvsPartition::take()?;

    info!("Starting EPD example");

    let (epd, net) = epd_pins::get_pins()?;

    let res = getRequest(net, nvs)?;
    epd_start_render_text(epd, res)?;
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
