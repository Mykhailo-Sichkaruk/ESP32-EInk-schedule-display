use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_svc::http::Method;
use std::time::Duration;

use crate::app_error::AppError;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    server_url: &'static str,
    #[default("")]
    device_mac: &'static str,
}

const MAX_BODY_LEN_BYTES: usize = 35 * 1024;
const READ_CHUNK_SIZE: usize = 1024;

pub fn fetch_schedule() -> Result<String, AppError> {
    let http_config = Configuration {
        buffer_size: Some(READ_CHUNK_SIZE),
        timeout: Some(Duration::from_secs(30)),
        ..Default::default()
    };

    let connection =
        EspHttpConnection::new(&http_config).map_err(|_| AppError::HttpConnectionFailed)?;
    let mut client = HttpClient::wrap(connection);

    let headers = [("accept", "application/json")];
    let request = client
        .request(Method::Get, CONFIG.server_url, &headers)
        .map_err(|_| AppError::HttpRequestCreationFailed)?;
    let mut response = request
        .submit()
        .map_err(|_| AppError::HttpRequestSubmitFailed)?;

    let status = response.status();
    if !(200..=299).contains(&status) {
        return Err(AppError::HttpUnexpectedStatus(status));
    }

    let mut body_buf = Box::new([0u8; MAX_BODY_LEN_BYTES]);
    let mut total_len: usize = 0;
    let mut chunk = [0u8; READ_CHUNK_SIZE];

    loop {
        let n = response
            .read(&mut chunk)
            .map_err(|_| AppError::HttpResponseReadFailed)?;
        if n == 0 {
            break;
        }

        if total_len + n > MAX_BODY_LEN_BYTES {
            return Err(AppError::HttpResponseTooLarge);
        }

        body_buf[total_len..total_len + n].copy_from_slice(&chunk[..n]);
        total_len += n;
    }

    let body_str = std::str::from_utf8(&body_buf[..total_len])
        .map_err(|_| AppError::HttpResponseInvalidUtf8)?
        .to_owned();

    Ok(body_str)
}

pub fn post_error(message: &str) -> Result<(), AppError> {
    let url = format!("{}/error", CONFIG.server_url.trim_end_matches('/'));
    let payload = serde_json::json!({
        "mac": CONFIG.device_mac,
        "message": message,
    });
    let body = serde_json::to_string(&payload).map_err(|_| AppError::JsonSerializationFailed)?;

    let http_config = Configuration {
        timeout: Some(Duration::from_secs(15)),
        ..Default::default()
    };

    let connection =
        EspHttpConnection::new(&http_config).map_err(|_| AppError::HttpConnectionFailed)?;
    let mut client = HttpClient::wrap(connection);
    let headers = [("content-type", "application/json")];

    let mut request = client
        .request(Method::Post, &url, &headers)
        .map_err(|_| AppError::HttpRequestCreationFailed)?;
    request
        .write(body.as_bytes())
        .map_err(|_| AppError::HttpRequestSubmitFailed)?;

    let response = request
        .submit()
        .map_err(|_| AppError::HttpRequestSubmitFailed)?;
    let status = response.status();

    if !(200..=299).contains(&status) {
        return Err(AppError::HttpUnexpectedStatus(status));
    }

    Ok(())
}
