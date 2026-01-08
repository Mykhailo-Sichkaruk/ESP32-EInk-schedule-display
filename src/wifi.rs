use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
    ScanMethod,
};

use crate::app_error::AppError;
use crate::hardware::NetParts;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

pub struct WifiConnection {
    wifi: BlockingWifi<EspWifi<'static>>,
}

pub fn connect(
    NetParts { modem, sysloop }: NetParts,
    nvs: EspDefaultNvsPartition,
) -> Result<WifiConnection, AppError> {
    let esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))
        .map_err(|_| AppError::WifiDriverCreationFailed)?;

    let mut wifi = BlockingWifi::wrap(esp_wifi, sysloop).map_err(|_| AppError::WifiWrapFailed)?;

    let ssid = CONFIG
        .wifi_ssid
        .try_into()
        .map_err(|_| AppError::InvalidWifiSsid)?;
    let password = CONFIG
        .wifi_psk
        .try_into()
        .map_err(|_| AppError::InvalidWifiPassword)?;

    let wifi_configuration: WifiConfiguration = WifiConfiguration::Client(ClientConfiguration {
        ssid,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password,
        channel: None,
        scan_method: ScanMethod::FastScan,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)
        .map_err(|_| AppError::WifiConfigurationFailed)?;
    wifi.start().map_err(|_| AppError::WifiStartFailed)?;
    wifi.connect().map_err(|_| AppError::WifiConnectFailed)?;
    wifi.wait_netif_up()
        .map_err(|_| AppError::WifiNetifUpFailed)?;

    Ok(WifiConnection { wifi })
}

pub fn disconnect(mut conn: WifiConnection) -> Result<(), AppError> {
    conn.wifi
        .disconnect()
        .map_err(|_| AppError::WifiDisconnectFailed)?;
    Ok(())
}
