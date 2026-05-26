use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct SystemResponse {
    pub hostname: String,
    pub cpu: u8,
    pub memory: MemStats,
    pub disk: DiskStats,
}

#[derive(Serialize)]
pub struct MemStats {
    pub used: f64,
    pub total: f64,
}

#[derive(Serialize)]
pub struct DiskStats {
    pub used: f64,
    pub total: f64,
}

pub async fn info() -> impl IntoResponse {
    match crate::podman::system_info().await {
        Ok(s) => {
            let resp = SystemResponse {
                hostname: s.hostname,
                cpu: s.cpu,
                memory: MemStats { used: s.memory_used_gb, total: s.memory_total_gb },
                disk: DiskStats { used: s.disk_used_gb, total: s.disk_total_gb },
            };
            (StatusCode::OK, Json(resp)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to read system info: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
