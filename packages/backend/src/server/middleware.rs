use anyhow::anyhow;
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use http_body_util::BodyExt;
use reqwest::StatusCode;
use serde_json::Value;

use super::types::{AppError, Resp};

pub async fn wrap_response_middleware(request: Request, next: Next) -> Result<Response, AppError> {
    let response = next.run(request).await;

    if response.status() != StatusCode::OK {
        return Ok(response);
    }

    match response.headers().get("content-type") {
        Some(content_type) => {
            let Ok(content_type) = content_type.to_str() else {
                return Ok(response);
            };
            if !content_type.contains("application/json") {
                return Ok(response);
            }
        }
        None => {
            return Ok(response);
        }
    }

    let (parts, body) = response.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|_| anyhow!("[WrapResponseMiddleware] Failed to collect body"))?
        .to_bytes();

    let data = serde_json::from_slice::<Value>(&bytes)
        .map_err(|_| anyhow!("[WrapResponseMiddleware] Failed to parse body"))?;

    let new_response = Response::from_parts(
        parts,
        Body::from(
            serde_json::to_string(&Resp {
                code: 0,
                msg: "".to_string(),
                data: Some(data),
            })
            .unwrap(),
        ),
    );

    Ok(new_response)
}
