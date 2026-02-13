use axum::{
    extract::Json,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tempfile::Builder;
use tower_http::cors::CorsLayer;

use crate::pandoc::PandocWrapper;
use crate::styles::Profile;
use tracing::{debug, error};

#[derive(RustEmbed)]
#[folder = "web/dist/"]
struct Assets;

#[derive(Deserialize)]
pub struct ConvertRequest {
    pub markdown: String,
    pub density: Option<String>,
    pub two_cols: Option<bool>,
    pub latex_font: Option<bool>,
    pub alt_table: Option<bool>,
    pub pretty_code: Option<bool>,
    pub section_numbering: Option<bool>,
    pub outline: Option<bool>,
}

#[derive(Serialize)]
pub struct ConvertResponse {
    pub pdf_base64: String,
}

pub async fn start_server(port: u16, api_only: bool) -> anyhow::Result<()> {
    let mut app = Router::new()
        .route("/api/convert", post(handle_convert_pdf))
        .route("/api/convert/typ", post(handle_convert_typ))
        .route("/api/health", get(|| async { "OK" }));

    if !api_only {
        app = app.fallback(static_handler);
    }

    let app = app.layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Quoin server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn handle_convert_pdf(payload: Json<ConvertRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!("Received PDF conversion request");
    handle_convert(payload, true).await
}

async fn handle_convert_typ(payload: Json<ConvertRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    tracing::info!("Received Typst conversion request");
    handle_convert(payload, false).await
}

async fn handle_convert(Json(payload): Json<ConvertRequest>, is_pdf: bool) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut profile = Profile::new();
    profile.set_global_defaults();
    
    if let Some(density) = payload.density {
        profile.set_density(&density);
    }
    if let Some(true) = payload.two_cols {
        profile.set_two_cols(true);
    }
    if let Some(true) = payload.latex_font {
        profile.set_latex_font();
    }
    if let Some(true) = payload.alt_table {
        profile.set_alt_table();
    }
    if let Some(true) = payload.pretty_code {
        profile.set_pretty_code();
    }
    if let Some(true) = payload.section_numbering {
        profile.set_section_numbering(true);
    }
    if let Some(true) = payload.outline {
        profile.set_outline();
    }

    // Create a temporary directory for conversion
    let tmp_dir = Builder::new().prefix("quoin-web-").tempdir()
        .map_err(|e| {
            error!("Failed to create temporary directory: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    
    let input_path = tmp_dir.path().join("input.md");
    let ext = if is_pdf { "pdf" } else { "typ" };
    let output_path = tmp_dir.path().join(format!("output.{}", ext));

    std::fs::write(&input_path, payload.markdown)
        .map_err(|e| {
            error!("Failed to write markdown to temporary file: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    debug!("Running pandoc conversion to {:?}", output_path);
    if let Err(e) = PandocWrapper::convert(&profile, input_path.to_str().unwrap(), output_path.to_str().unwrap(), !is_pdf) {
        error!("Conversion failed: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }

    let bytes = std::fs::read(&output_path)
        .map_err(|e| {
            error!("Failed to read output file: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let content_type = if is_pdf { "application/pdf" } else { "text/plain" };

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(axum::body::Body::from(bytes))
        .unwrap())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == "index.html" {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(axum::body::Body::from(content.data))
                .unwrap()
        }
        None => {
            if path.contains('.') {
                StatusCode::NOT_FOUND.into_response()
            } else {
                index_html().await
            }
        }
    }
}

async fn index_html() -> Response {
    match Assets::get("index.html") {
        Some(content) => Html(content.data).into_response(),
        None => (StatusCode::NOT_FOUND, "Index file not found").into_response(),
    }
}
