use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_svc::http::Method;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
};
use log::{error, info};
use std::time::Duration;

use crate::app_error::AppError;
use crate::esp_resource::NetParts;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    server_url: &'static str,
    #[default("")]
    device_mac: &'static str,
}

const MAX_BODY_LEN_BYTES: usize = 35 * 1024;
const READ_CHUNK_SIZE: usize = 1024;

pub struct WifiClient {
    wifi: BlockingWifi<EspWifi<'static>>,
}

impl WifiClient {
    pub fn new(
        NetParts { modem, sysloop }: NetParts,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self, anyhow::Error> {
        let wifi = BlockingWifi::wrap(
            EspWifi::new(modem, sysloop.clone(), Some(nvs))
                .map_err(|error| AppError::WifiInit(anyhow::anyhow!(error.to_string())))?,
            sysloop,
        )
        .map_err(|error| AppError::WifiInit(anyhow::anyhow!(error.to_string())))?;

        Ok(Self { wifi })
    }

    pub fn connect(&mut self) -> anyhow::Result<()> {
        let ssid = CONFIG
            .wifi_ssid
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid wifi_ssid"))?;
        let password = CONFIG
            .wifi_psk
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid wifi_psk"))?;

        let wifi_configuration: WifiConfiguration =
            WifiConfiguration::Client(ClientConfiguration {
                ssid,
                bssid: None,
                auth_method: AuthMethod::WPA2Personal,
                password,
                channel: None,
                ..Default::default()
            });

        self.wifi
            .set_configuration(&wifi_configuration)
            .map_err(|error| anyhow::anyhow!("Wifi configuration error, {}", error))?;
        self.wifi
            .start()
            .map_err(|error| anyhow::anyhow!("Wifi start error, {}", error))?;
        self.wifi
            .connect()
            .map_err(|error| anyhow::anyhow!("Wifi connect error, {}", error))?;
        self.wifi
            .wait_netif_up()
            .map_err(|error| anyhow::anyhow!("Wifi wait netif up error, {}", error))?;

        Ok(())
    }

    pub fn fetch_shedule_retried(&mut self, retries: u8) -> anyhow::Result<String> {
        let mut attempt = 0;
        loop {
            match self.fetch_schedule() {
                Ok(body) => return Ok(body),
                Err(e) => {
                    attempt += 1;
                    if attempt > retries {
                        error!("fetch_schedule failed after {} attempts: {:?}", attempt, e);
                        return Err(e);
                    } else {
                        error!(
                            "fetch_schedule attempt {} failed: {:?}. Retrying...",
                            attempt, e
                        );
                    }
                }
            }
        }
    }

    pub fn fetch_schedule(&mut self) -> anyhow::Result<String> {
        self.connect()?;
        let http_config = Configuration {
            buffer_size: Some(READ_CHUNK_SIZE),
            timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        };

        let connection = EspHttpConnection::new(&http_config)?;
        let mut client = HttpClient::wrap(connection);

        let headers = [("accept", "application/json")];
        let request = client.request(Method::Get, CONFIG.server_url, &headers)?;
        let mut response = request.submit()?;

        let status = response.status();
        match status {
            200..=299 => info!("Request successful with status {}", status),
            _ => {
                error!("Request failed with status code: {}", status);
                return Err(anyhow::anyhow!(
                    "Request failed with status code: {}",
                    status
                ));
            }
        }

        // Heap-allocated fixed-size buffer for the whole body
        let mut body_buf = Box::new([0u8; MAX_BODY_LEN_BYTES]);
        let mut total_len: usize = 0;

        // Small stack buffer for chunked reads
        let mut chunk = [0u8; READ_CHUNK_SIZE];

        loop {
            let n = response.read(&mut chunk)?;
            if n == 0 {
                // End of body
                break;
            }

            if total_len + n > MAX_BODY_LEN_BYTES {
                return Err(anyhow::anyhow!("HTTP Response body too large"));
            }

            // Copy chunk into the big buffer
            body_buf[total_len..total_len + n].copy_from_slice(&chunk[..n]);
            total_len += n;
        }

        // Convert the used slice into &str then to owned String
        let body_str = std::str::from_utf8(&body_buf[..total_len])
            .map_err(|e| anyhow::anyhow!("Failed to decode response body as UTF-8"))?
            .to_owned();

        // Be careful logging the whole 35KB body; this can be slow over UART.
        // You might truncate it for logs:
        let log_preview_len = body_str.len().min(256);

        Ok(body_str)
    }

    pub fn post_error(&mut self, message: &str) -> anyhow::Result<()> {
        self.connect()?;

        let url = format!("{}/error", CONFIG.server_url.trim_end_matches('/'));
        let payload = serde_json::json!({
            "mac": CONFIG.device_mac,
            "message": message,
        });
        let body = serde_json::to_string(&payload)?;

        let http_config = Configuration {
            timeout: Some(Duration::from_secs(15)),
            ..Default::default()
        };

        let connection = EspHttpConnection::new(&http_config)?;
        let mut client = HttpClient::wrap(connection);
        let headers = [("content-type", "application/json")];

        let mut request = client.request(Method::Post, &url, &headers)?;
        request.write(body.as_bytes())?;

        let response = request.submit()?;
        let status = response.status();

        match status {
            200..=299 => info!("Error posted successfully (status {})", status),
            _ => {
                error!("Error post failed with status code: {}", status);
                return Err(anyhow::anyhow!(
                    "Error post failed with status code: {}",
                    status
                ));
            }
        }

        Ok(())
    }
}
