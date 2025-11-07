use crate::consts;
use crate::keyboard::{LedColor, MediaCode, Modifier, MouseAction, MouseButton, WellKnownCode};
use crate::mapping::Mapping;
use crate::options::Options;
use crate::{find_device, open_keyboard};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use anyhow::Context as _;
use strum::EnumMessage as _;
use strum::IntoEnumIterator as _;

// Response types
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}

// Custom error type that converts to HTTP responses
struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error_msg = format!("{:#}", self.0);
        let response = ApiResponse::<()>::error(error_msg);
        (StatusCode::BAD_REQUEST, Json(response)).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Request/Response types
#[derive(Serialize, Deserialize)]
pub struct KeysResponse {
    pub modifiers: Vec<String>,
    pub keys: Vec<String>,
    pub media_keys: Vec<String>,
    pub mouse_actions: Vec<String>,
    pub custom_key_syntax: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateRequest {
    pub config_json: String,
    pub product_id: Option<u16>,
    pub device_connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProgramRequest {
    pub config_json: String,
}

#[derive(Serialize, Deserialize)]
pub struct LedRequest {
    pub index: u8,
    pub layer: u8,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct ReadQuery {
    pub layer: Option<u8>,
}

// Handlers

/// GET /api/keys - Get all supported keys and modifiers
async fn get_keys() -> Result<Json<ApiResponse<KeysResponse>>, ApiError> {
    let modifiers: Vec<String> = Modifier::iter()
        .map(|m| m.get_serializations().join(" / "))
        .collect();

    let keys: Vec<String> = WellKnownCode::iter()
        .map(|c| c.to_string())
        .collect();

    let media_keys: Vec<String> = MediaCode::iter()
        .map(|c| c.get_serializations().join(" / "))
        .collect();

    let mut mouse_actions = vec![
        MouseAction::WheelDown.to_string(),
        MouseAction::WheelUp.to_string(),
    ];
    mouse_actions.extend(MouseButton::iter().map(|b| b.to_string()));

    let response = KeysResponse {
        modifiers,
        keys,
        media_keys,
        mouse_actions,
        custom_key_syntax: "Use decimal code: <110>".to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/device - Check if device is connected
async fn get_device() -> Result<Json<ApiResponse<DeviceInfo>>, ApiError> {
    let device_result = find_device(consts::VENDOR_ID, None);

    let info = if let Ok((_device, _desc, product_id)) = device_result {
        DeviceInfo {
            vendor_id: consts::VENDOR_ID,
            product_id,
            connected: true,
        }
    } else {
        DeviceInfo {
            vendor_id: consts::VENDOR_ID,
            product_id: 0,
            connected: false,
        }
    };

    Ok(Json(ApiResponse::success(info)))
}

/// POST /api/validate - Validate configuration
async fn validate_config(
    Json(payload): Json<ValidateRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    // Write the config to a temporary file
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("macropad_validate.ron");
    std::fs::write(&config_path, &payload.config_json)?;

    if payload.device_connected {
        if let Ok(device) = find_device(consts::VENDOR_ID, None) {
            Mapping::validate(config_path.to_str().unwrap(), Some(device.2))
                .context("validating configuration file with connected device")?;
        } else {
            return Err(anyhow::anyhow!(
                "Unable to find connected device with vendor id: 0x{:02x}",
                consts::VENDOR_ID
            ).into());
        }
    } else if let Some(pid) = payload.product_id {
        Mapping::validate(config_path.to_str().unwrap(), Some(pid))
            .context("validating configuration file against specified product id")?;
    } else {
        Mapping::validate(config_path.to_str().unwrap(), None)
            .context("generic validation of configuration file")?;
    }

    Ok(Json(ApiResponse::success("Configuration is valid".to_string())))
}

/// POST /api/program - Program device with configuration
async fn program_device(
    Json(payload): Json<ProgramRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    // Write the config to a temporary file
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("macropad_program.ron");
    std::fs::write(&config_path, &payload.config_json)?;

    let config = Mapping::read(config_path.to_str().unwrap());
    let options = Options::default();
    let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
    keyboard.program(&config).context("programming macropad")?;

    Ok(Json(ApiResponse::success("Device programmed successfully".to_string())))
}

/// GET /api/read - Read configuration from device
async fn read_config(Query(params): Query<ReadQuery>) -> Result<Json<ApiResponse<String>>, ApiError> {
    let options = Options::default();
    let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
    let layer = params.layer.unwrap_or(0);
    let macropad_config = keyboard
        .read_macropad_config(&layer)
        .context("reading macropad configuration")?;

    // Serialize the config to RON format
    let ron_string = ron::ser::to_string_pretty(&macropad_config, ron::ser::PrettyConfig::default())?;

    Ok(Json(ApiResponse::success(ron_string)))
}

/// POST /api/led - Set LED color
async fn set_led(Json(payload): Json<LedRequest>) -> Result<Json<ApiResponse<String>>, ApiError> {
    let options = Options::default();
    let mut keyboard = open_keyboard(&options).context("opening keyboard")?;

    let color = if let Some(color_str) = &payload.color {
        match color_str.to_lowercase().as_str() {
            "red" => LedColor::Red,
            "orange" => LedColor::Orange,
            "yellow" => LedColor::Yellow,
            "green" => LedColor::Green,
            "cyan" => LedColor::Cyan,
            "blue" => LedColor::Blue,
            "purple" => LedColor::Purple,
            _ => LedColor::Red,
        }
    } else {
        LedColor::Red
    };

    keyboard
        .set_led(payload.index, payload.layer, color)
        .context("programming LED on macropad")?;

    Ok(Json(ApiResponse::success("LED set successfully".to_string())))
}

/// Create and configure the router
pub fn create_router() -> Router {
    // CORS configuration for local development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/keys", get(get_keys))
        .route("/api/device", get(get_device))
        .route("/api/validate", post(validate_config))
        .route("/api/program", post(program_device))
        .route("/api/read", get(read_config))
        .route("/api/led", post(set_led))
        .layer(cors)
        // Serve the frontend static files
        .fallback_service(ServeDir::new("Webapp/dist"))
}

/// Start the API server
pub async fn run_server(port: u16) -> anyhow::Result<()> {
    let app = create_router();
    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("🚀 Macrocli API server running on http://{}", addr);
    println!("📱 Frontend available at http://{}", addr);
    println!("🔌 API endpoints at http://{}/api/*", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
