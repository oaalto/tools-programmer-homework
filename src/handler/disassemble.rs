use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub disassembly: Vec<String>,
}

pub async fn disassemble(Extension(res): Extension<Vec<String>>) -> Response {
    let output = Output { disassembly: res };
    Json(output).into_response()
}
