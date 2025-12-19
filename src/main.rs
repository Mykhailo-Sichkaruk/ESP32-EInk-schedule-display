use esp_backtrace as _;
use esp_eink_schedule::{esp_resource, render, schedule_api, wifilib};
use log::info;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    server_url: &'static str,
    #[default(300)]
    sleep_time_secs: u64,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let (epd_pins, net, nvs) = esp_resource::get()?;

    wifilib::connect_wifi(net, nvs)?;
    let result = wifilib::fetch_schedule()?;
    let schedule = schedule_api::parse_schedule(&result)?;

    let res = render::render_schedule(epd_pins, schedule)?;

    info!("Going to deep sleep for {0} seconds...", CONFIG.sleep_time_secs);
    unsafe {
        esp_idf_sys::esp_sleep_enable_timer_wakeup(CONFIG.sleep_time_secs * 1_000_000);
        esp_idf_sys::esp_deep_sleep_start();
    }
}

