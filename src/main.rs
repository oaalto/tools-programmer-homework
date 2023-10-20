use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    info!("{:<12} - {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    data: [u8; 10],
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output {
    data: [u8; 10],
}

async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload { data } = payload;
    let res = disassemble(&data);
    Json(res).into_response()
}

fn disassemble(_data: &[u8; 10]) -> Output {
    // process the incoming data here
    Output { data: [0; 10] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_disassemble_ok() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload { data: [0; 10] };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Output = Output { data: [0; 10] };
        assert_eq!(expected, res);
    }
}
