use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Resp<T>
where
    T: Serialize,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Resp::<()> {
                code: -1,
                msg: self.0.to_string(),
                data: None,
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;
