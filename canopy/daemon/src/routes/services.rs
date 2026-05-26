use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: String,
    pub status: String,
}

pub async fn list() -> impl IntoResponse {
    match crate::podman::list_containers().await {
        Ok(containers) => {
            let services: Vec<ServiceInfo> = containers
                .into_iter()
                .map(|c| ServiceInfo {
                    id: c.id.chars().take(12).collect(),
                    name: c.names.first().cloned().unwrap_or_default()
                        .trim_start_matches('/')
                        .to_string(),
                    image: c.image,
                    state: c.state,
                    status: c.status,
                })
                .collect();
            (StatusCode::OK, Json(services)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list containers: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
