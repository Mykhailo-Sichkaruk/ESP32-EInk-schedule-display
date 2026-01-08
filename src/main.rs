use esp_backtrace as _;
use esp_eink_schedule::{
    app_error::AppError, display, hardware, http, parallel, schedule_api, wifi,
};

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

fn main() {
    esp_idf_sys::link_patches();

    let (display_pins, net_parts, nvs) = hardware::get();

    let _ = run_sequential(display_pins, net_parts, nvs);

    deep_sleep(CONFIG.sleep_time_secs);
}

fn run_sequential(
    display_pins: hardware::DisplayPins,
    net_parts: hardware::NetParts,
    nvs: esp_idf_svc::nvs::EspDefaultNvsPartition,
) -> Result<(), AppError> {
    // Step 1: Connect WiFi and fetch schedule
    let wifi_conn = match wifi::connect(net_parts, nvs) {
        Ok(conn) => conn,
        Err(err) => {
            parallel::handle_error(None, Some(display_pins), &err);
            return Err(err);
        }
    };

    let schedule_json = match http::fetch_schedule() {
        Ok(json) => json,
        Err(err) => {
            parallel::handle_error(Some(wifi_conn), Some(display_pins), &err);
            return Err(err);
        }
    };

    // Step 2: Parse JSON
    let response = match schedule_api::parse(&schedule_json) {
        Ok(resp) => resp,
        Err(err) => {
            parallel::handle_error(Some(wifi_conn), Some(display_pins), &err);
            return Err(err);
        }
    };

    // Step 3: Disconnect WiFi
    let _ = wifi::disconnect(wifi_conn);

    // Step 4: Render display
    display::render_schedule(display_pins, response)?;

    Ok(())
}

fn deep_sleep(seconds: u64) {
    unsafe {
        esp_idf_sys::esp_sleep_enable_timer_wakeup(seconds * 1_000_000);
        esp_idf_sys::esp_deep_sleep_start();
    }
}
