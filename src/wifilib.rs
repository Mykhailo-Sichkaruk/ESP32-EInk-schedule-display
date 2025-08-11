use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::http::Method;
use embedded_svc::utils::io;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::{
    wifi::{
        AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
    },
};
use log::{error, info};

use crate::epd_pins::NetParts;

const SSID: &str = "Mi 9 SE";
const PASS: &str = "passpass";

pub fn getRequest(NetParts { modem, sysloop }: NetParts, nvs: EspDefaultNvsPartition) -> anyhow::Result<String> {
    let mut wifi = BlockingWifi::wrap(
        // TODO: handle wifi connection error
        EspWifi::new(modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    let wifi_configuration: WifiConfiguration = WifiConfiguration::Client(ClientConfiguration {
        ssid: SSID
            .try_into()
            .expect("Could not parse the given SSID into WiFi config"),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASS
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

    let mut client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);
    let headers = [("accept", "application/json")];
    let request = client.request(Method::Get, "http://google.com:80", &headers)?;
    let mut response = request.submit()?;
    let status = response.status();
    info!("Response status: {}", status);
    let mut bytes = [0; 1024]; // Buffer size of 1024 bytes
    let readBytes = response.read(&mut bytes)?;
    info!("Read {} bytes", bytes.len());
    let body = String::from_utf8(bytes[0..readBytes].to_vec());
    info!("Response body: {:?}", body);

    if body.is_err() {
        error!("Failed to read response body: {:?}", body);
        return Err(anyhow::anyhow!("Failed to read response body"));
    }

    Ok(body?)
}
