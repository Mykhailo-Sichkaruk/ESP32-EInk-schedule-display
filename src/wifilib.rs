use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_svc::http::Method;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::{
    wifi::{
        AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
    },
};
use log::{error, info};

use crate::esp_resource::NetParts;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    server_url: &'static str,
}

pub fn connect_wifi(NetParts { modem, sysloop }: NetParts, nvs: EspDefaultNvsPartition) -> anyhow::Result<()> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    let wifi_configuration: WifiConfiguration = WifiConfiguration::Client(ClientConfiguration {
        ssid: CONFIG.wifi_ssid
            .try_into()
            .expect("Could not parse the given SSID into WiFi config"),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: CONFIG.wifi_psk
            .try_into()
            .expect("Could not parse the given password into WiFi config"),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    info!("Wifi connected");
    Ok(())
}

pub fn fetch_schedule() -> anyhow::Result<String> {
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = HttpClient::wrap(connection);
    let headers = [("accept", "application/json")];
    let request = client.request(Method::Get, CONFIG.server_url, &headers)?;
    let mut response = request.submit()?;
    let status = response.status();
    match status {
        200..=299 => info!("Request successful"),
        _ => {
            error!("Request failed with status code: {}", status);
            return Err(anyhow::anyhow!("Request failed with status code: {}", status));
        }
    }
    let mut bytes = [0; 1024]; // Buffer size of 1024 bytes
    let read_bytes = response.read(&mut bytes)?;
    info!("Read {} bytes", bytes.len());
    let body = String::from_utf8(bytes[0..read_bytes].to_vec());
    info!("Response body: {:?}", body);

    if body.is_err() {
        error!("Failed to read response body: {:?}", body);
        return Err(anyhow::anyhow!("Failed to read response body"));
    }

    Ok(body?)
}

pub fn post_error() -> anyhow::Result<()> {
    // Placeholder for posting error back to server
    Ok(())
}
