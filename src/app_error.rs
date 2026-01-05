use core::fmt;

#[derive(Debug)]
pub enum AppError {
    /// Failed to create WiFi driver (EspWifi::new)
    WifiDriverCreationFailed,
    /// Failed to wrap WiFi in blocking mode (BlockingWifi::wrap)
    WifiWrapFailed,
    /// Invalid WiFi SSID in configuration
    InvalidWifiSsid,
    /// Invalid WiFi password in configuration
    InvalidWifiPassword,
    /// Failed to set WiFi configuration (wifi.set_configuration)
    WifiConfigurationFailed,
    /// Failed to start WiFi (wifi.start)
    WifiStartFailed,
    /// Failed to connect to WiFi network (wifi.connect)
    WifiConnectFailed,
    /// Failed to wait for network interface up (wifi.wait_netif_up)
    WifiNetifUpFailed,
    // Failed to disconnect from wifi
    WifiDisconnectFailed,
    /// Failed to create HTTP connection (EspHttpConnection::new)
    HttpConnectionFailed,
    /// Failed to create HTTP request (client.request)
    HttpRequestCreationFailed,
    /// Failed to submit HTTP request or write body
    HttpRequestSubmitFailed,
    /// Failed to read HTTP response body
    HttpResponseReadFailed,
    /// HTTP response body exceeds maximum size (35KB)
    HttpResponseTooLarge,
    /// HTTP response body is not valid UTF-8
    HttpResponseInvalidUtf8,
    /// HTTP response returned non-2xx status code
    HttpUnexpectedStatus(u16),
    /// Failed to serialize JSON (serde_json::to_string)
    JsonSerializationFailed,
    /// Failed to deserialize JSON (serde_json::from_str)
    JsonDeserializationFailed,
    /// Failed to configure GPIO pin (PinDriver::output/input or set_high/set_low)
    GpioPinConfigFailed,
    /// Failed to create SPI driver (SpiDeviceDriver::new_single)
    SpiDriverCreationFailed,
    /// Failed to initialize e-paper display (Epd::new)
    EpdInitFailed,
    /// Failed to wake up e-paper display (epd.wake_up)
    EpdWakeUpFailed,
    /// Failed to update e-paper frame (epd.update_and_display_frame)
    EpdUpdateFrameFailed,
    /// Failed to put e-paper display to sleep (epd.sleep)
    EpdSleepFailed,
    /// Invalid time interval provided (TimeInterval::new)
    InvalidTimeInterval,
    /// Failed to create schedule table (ScheduleTable::new)
    ScheduleTableCreationFailed,
    /// Failed to draw graphics to display buffer
    DrawFailed,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::WifiDriverCreationFailed => write!(f, "E_WIFI_0"),
            AppError::WifiWrapFailed => write!(f, "E_WIFI_1"),
            AppError::InvalidWifiSsid => write!(f, "E_WIFI_2"),
            AppError::InvalidWifiPassword => write!(f, "E_WIFI_3"),
            AppError::WifiConfigurationFailed => write!(f, "E_WIFI_4"),
            AppError::WifiStartFailed => write!(f, "E_WIFI_5"),
            AppError::WifiConnectFailed => write!(f, "E_WIFI_6"),
            AppError::WifiNetifUpFailed => write!(f, "E_WIFI_7"),
            AppError::WifiDisconnectFailed => write!(f, "E_WIFI_8"),
            AppError::HttpConnectionFailed => write!(f, "E_HTTP_0"),
            AppError::HttpRequestCreationFailed => write!(f, "E_HTTP_1"),
            AppError::HttpRequestSubmitFailed => write!(f, "E_HTTP_2"),
            AppError::HttpResponseReadFailed => write!(f, "E_HTTP_3"),
            AppError::HttpResponseTooLarge => write!(f, "E_HTTP_4"),
            AppError::HttpResponseInvalidUtf8 => write!(f, "E_HTTP_5"),
            AppError::HttpUnexpectedStatus(status) => write!(f, "E_HTTP_6({})", status),
            AppError::JsonSerializationFailed => write!(f, "E_JSON_0"),
            AppError::JsonDeserializationFailed => write!(f, "E_JSON_1"),
            AppError::GpioPinConfigFailed => write!(f, "E_HW_0"),
            AppError::SpiDriverCreationFailed => write!(f, "E_HW_1"),
            AppError::EpdInitFailed => write!(f, "E_EPD_0"),
            AppError::EpdWakeUpFailed => write!(f, "E_EPD_1"),
            AppError::EpdUpdateFrameFailed => write!(f, "E_EPD_2"),
            AppError::EpdSleepFailed => write!(f, "E_EPD_3"),
            AppError::InvalidTimeInterval => write!(f, "E_RENDER_0"),
            AppError::ScheduleTableCreationFailed => write!(f, "E_RENDER_1"),
            AppError::DrawFailed => write!(f, "E_RENDER_2"),
        }
    }
}

impl std::error::Error for AppError {}
