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
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Output { 
    disassembly: Vec<String>,
}

async fn handler(Json(payload): Json<Payload>) -> Response {
    let Payload { data } = payload;
    let res = disassemble(data);
    Json(res).into_response()
}

fn disassemble(_data: Vec<u8>) -> Output {
    // process the incoming data here
    
        Output { disassembly: ["0x00000000 48e7 2020   MOVEM.L D5,A5,-(A7)","0x00000004 7021        MOVEQ #$21, D0"].iter().map(|&s| s.into()).collect() }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_api_disassemble_ok() {
        const URL: &'static str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload { data: [0xe7, 0x48, 0x20, 0x20, 0x21, 0x70].to_vec() };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap(); 

        let expected: Output = Output { disassembly: ["0x00000000 48e7 2020   MOVEM.L D5,A5,-(A7)", "0x00000004 7021        MOVEQ #$21, D0"].iter().map(|&s| s.into()).collect() };
        assert_eq!(expected, res);
    }
}
