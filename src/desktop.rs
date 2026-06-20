//! Desktop shell bootstrap for MediaVault.

use crate::error::{Result, VaultError};
use tauri::http::{header::CONTENT_TYPE, Response, StatusCode};

const PROTOCOL_SCHEME: &str = "mediavault";

const INDEX_HTML: &str = include_str!("../dist/index.html");
const APP_JS: &str = include_str!("../dist/app.js");
const STYLES_CSS: &str = include_str!("../dist/styles.css");

/// Starts the Tauri desktop shell.
pub(crate) fn run() -> Result<()> {
    tauri::Builder::default()
        .register_uri_scheme_protocol(PROTOCOL_SCHEME, |_context, request| {
            let path = request.uri().path();

            match path {
                "/" | "/index.html" => {
                    response(StatusCode::OK, "text/html; charset=utf-8", INDEX_HTML)
                }
                "/app.js" => response(StatusCode::OK, "application/javascript; charset=utf-8", APP_JS),
                "/styles.css" => response(StatusCode::OK, "text/css; charset=utf-8", STYLES_CSS),
                _ => response(StatusCode::NOT_FOUND, "text/plain; charset=utf-8", "Not Found"),
            }
        })
        .run(tauri::generate_context!())
        .map_err(|error| VaultError::AppStartup(error.to_string()))
}

fn response(status: StatusCode, content_type: &str, body: &str) -> Response<Vec<u8>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, content_type)
        .body(body.as_bytes().to_vec())
        .expect("response construction should succeed")
}
