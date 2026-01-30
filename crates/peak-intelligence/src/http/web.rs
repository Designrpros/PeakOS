// WASM HTTP implementation using reqwest and wasm-streams
use super::{HttpError, HttpResponse};
use bytes::Bytes;
use futures::StreamExt;
use serde::Serialize;

pub async fn get_with_headers(
    url: &str,
    headers: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let mut client = reqwest::Client::new().get(url);
    for (key, value) in headers {
        client = client.header(key, value);
    }

    let response = client
        .send()
        .await
        .map_err(|e| HttpError::WasmError(e.to_string()))?;

    let status = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .map_err(|e| HttpError::WasmError(e.to_string()))?
        .to_vec();

    Ok(HttpResponse { status, body })
}

pub async fn post_json_with_headers<T: Serialize>(
    url: &str,
    body: &T,
    headers: std::collections::HashMap<String, String>,
) -> Result<HttpResponse, HttpError> {
    let mut client = reqwest::Client::new().post(url).json(body);
    for (key, value) in headers {
        client = client.header(key, value);
    }

    let response = client
        .send()
        .await
        .map_err(|e| HttpError::WasmError(e.to_string()))?;

    let status = response.status().as_u16();
    let response_body = response
        .bytes()
        .await
        .map_err(|e| HttpError::WasmError(e.to_string()))?
        .to_vec();

    Ok(HttpResponse {
        status,
        body: response_body,
    })
}

pub async fn post_json_stream<T: Serialize>(
    url: &str,
    body: &T,
    headers: std::collections::HashMap<String, String>,
) -> Result<impl futures::Stream<Item = Result<Bytes, String>>, HttpError> {
    let mut client = reqwest::Client::new().post(url).json(body);
    for (key, value) in headers {
        client = client.header(key, value);
    }

    let response = client
        .send()
        .await
        .map_err(|e| HttpError::WasmError(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(HttpError::RequestFailed(format!(
            "[DEBUG] Streaming request failed with status: {} - {}",
            status, error_text
        )));
    }

    // On WASM, reqwest's bytes_stream might not be available or behave differently.
    // We use wasm-streams to bridge the browser's ReadableStream if needed,
    // but reqwest 0.12 with "stream" feature SHOULD provide it.
    // However, sometimes we need to manually bridge it.

    // Attempt to use reqwest's stream first
    let stream = response
        .bytes_stream()
        .map(|res| res.map_err(|e| e.to_string()));

    Ok(stream)
}
