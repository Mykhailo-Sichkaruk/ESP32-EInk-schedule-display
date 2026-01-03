use esp_backtrace as _;
use esp_eink_schedule::{
    app_error::AppError,
    esp_resource, render,
    schedule_api::Response,
    wifilib,
};
use log::{error, info, warn};

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
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut ctx = AppContext::default();
    if let Err(err) = run_cycle(&mut ctx) {
        handle_error(&mut ctx, &err);
    }

    info!(
        "Going to deep sleep for {0} seconds...",
        CONFIG.sleep_time_secs
    );
    unsafe {
        esp_idf_sys::esp_sleep_enable_timer_wakeup(CONFIG.sleep_time_secs * 1_000_000);
        esp_idf_sys::esp_deep_sleep_start();
    }
}

#[derive(Default)]
struct AppContext {
    wifi: Option<wifilib::WifiClient>,
    epd_pins: Option<esp_resource::EpdHardwarePins>,
}

fn run_cycle(ctx: &mut AppContext) -> Result<(), AppError> {
    let (epd_pins, net, nvs) = esp_resource::get();
    ctx.epd_pins = Some(epd_pins);

    let wifi = wifilib::WifiClient::new(net, nvs)?;
    ctx.wifi = Some(wifi);

    let schedule_json = ctx
        .wifi
        .as_mut()
        .expect("wifi not initialized")
        .fetch_schedule()?;

    let response: Response =
        serde_json::from_str(&schedule_json).map_err(|_| AppError::JsonDeserializationFailed)?;

    let epd_pins = ctx.epd_pins.take().expect("epd pins not initialized");
    render::render_schedule(epd_pins, response)?;

    Ok(())
}

fn handle_error(ctx: &mut AppContext, err: &AppError) {
    error!("App error: {err}");

    if let Some(wifi) = ctx.wifi.as_mut() {
        if let Err(send_err) = wifi.post_error(&err.to_string()) {
            warn!("Failed to post error: {send_err}");
        }
    }

    if let Some(epd_pins) = ctx.epd_pins.take() {
        if let Err(render_err) = render::render_error(epd_pins, &err.to_string()) {
            warn!("Failed to render error: {render_err}");
        }
    }
}
