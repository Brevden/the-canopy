use axum::{
    Json,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PullRequest {
    pub image: String,
}

#[derive(Serialize)]
pub struct PullResponse {
    pub success: bool,
    pub message: String,
}

pub async fn pull(Json(payload): Json<PullRequest>) -> impl IntoResponse {
    tracing::info!("Pulling image: {}", payload.image);
    match crate::podman::pull_image(&payload.image).await {
        Ok(()) => (StatusCode::OK, Json(PullResponse {
            success: true,
            message: format!("Pulled {}", payload.image),
        })).into_response(),
        Err(e) => {
            tracing::error!("Pull failed for {}: {}", payload.image, e);
            (StatusCode::OK, Json(PullResponse {
                success: false,
                message: e.to_string(),
            })).into_response()
        }
    }
}

use crate::podman;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

// The shape we send back to the Bun backend / GUI.
#[derive(Serialize)]
pub struct ImageResult {
    pub name: String,
    pub description: String,
    pub stars: u64,
    pub official: bool,
    pub registry: String,
}

pub async fn search(
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    match podman::search_images(&params.q).await {
        Ok(results) => {
            let mapped: Vec<ImageResult> = results
                .into_iter()
                .map(|r| ImageResult {
                    name: r.name,
                    description: r.description,
                    stars: r.stars,
                    official: r.official == "[OK]",
                    registry: r.index,
                })
                .collect();
            (StatusCode::OK, Json(mapped)).into_response()
        }
        Err(e) => {
            tracing::error!("Image search failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
