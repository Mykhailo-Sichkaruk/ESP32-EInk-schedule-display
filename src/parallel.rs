use esp_idf_svc::nvs::EspDefaultNvsPartition;
use std::thread::Builder;

use crate::app_error::AppError;
use crate::display;
use crate::hardware::{DisplayPins, NetParts};
use crate::http;
use crate::schedule_api::Response;
use crate::wifi::{self, WifiConnection};

// Stack sizes for ESP32 threads (in bytes)
// ESP-IDF WiFi typically needs 8-16KB, HTTP with TLS needs more
const NETWORK_STACK_SIZE: usize = 32 * 1024; // 32KB for WiFi + HTTP
const DISPLAY_STACK_SIZE: usize = 16 * 1024; // 16KB for display operations
const SMALL_STACK_SIZE: usize = 8 * 1024; // 8KB for disconnect

pub struct Phase1Result {
    pub wifi_conn: WifiConnection,
    pub schedule_json: String,
}

/// Phase 1: Connect WiFi and fetch schedule (sequential, display can't be parallelized)
pub fn phase1_fetch(
    net_parts: NetParts,
    nvs: EspDefaultNvsPartition,
) -> Result<Phase1Result, AppError> {
    let wifi_conn = wifi::connect(net_parts, nvs)?;
    let schedule_json = http::fetch_schedule()?;

    Ok(Phase1Result {
        wifi_conn,
        schedule_json,
    })
}

/// Phase 2: Disconnect WiFi and render display in parallel
pub fn phase2_finish(
    wifi_conn: WifiConnection,
    display_pins: DisplayPins,
    response: Response,
) -> Result<(), AppError> {
    // Disconnect thread
    let disconnect_handle = Builder::new()
        .stack_size(SMALL_STACK_SIZE)
        .spawn(move || -> Result<(), AppError> { wifi::disconnect(wifi_conn) })
        .expect("Failed to spawn disconnect thread");

    // Render thread
    let render_handle = Builder::new()
        .stack_size(DISPLAY_STACK_SIZE)
        .spawn(move || -> Result<(), AppError> { display::render_schedule(display_pins, response) })
        .expect("Failed to spawn render thread");

    // Wait for both - render is likely slower, but we wait for both
    disconnect_handle
        .join()
        .expect("Disconnect thread panicked")?;

    render_handle.join().expect("Render thread panicked")?;

    Ok(())
}

pub fn handle_error(
    wifi_conn: Option<WifiConnection>,
    display_pins: Option<DisplayPins>,
    error: &AppError,
) {
    let error_message = error.to_string();

    // Try to post error if we have WiFi
    if let Some(conn) = wifi_conn {
        let msg = error_message.clone();
        let _ = Builder::new()
            .stack_size(NETWORK_STACK_SIZE)
            .spawn(move || {
                let _ = http::post_error(&msg);
                let _ = wifi::disconnect(conn);
            })
            .map(|h| h.join());
    }

    // Try to render error if we have display
    if let Some(pins) = display_pins {
        let _ = display::render_error(pins, &error_message);
    }
}
