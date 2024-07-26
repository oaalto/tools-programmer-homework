use crate::model::disassembler;
use crate::model::disassembler::{Architecture, Disassembler};
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};

pub async fn disassembler(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    let json: Json<Payload> = Json::from_bytes(&bytes)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?;

    let disassembler = disassembler::get_disassembler_for_architecture(json.0.architecture);
    let res = disassembler.disassemble(&json.0.data);

    let mut request = Request::from_parts(parts, Body::empty());
    request.extensions_mut().insert(res);

    Ok(next.run(request).await)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub data: Vec<u8>,
    pub architecture: Architecture,
}
