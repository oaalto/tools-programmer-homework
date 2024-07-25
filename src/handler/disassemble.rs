use crate::model::disassembler::DisassemblyResult;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub disassembly: Vec<String>,
}

pub async fn disassemble(Extension(res): Extension<DisassemblyResult>) -> Response {
    let output = Output { disassembly: res.0 };
    Json(output).into_response()
}
