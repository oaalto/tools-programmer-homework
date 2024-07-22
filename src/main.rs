mod line;
mod opcodes;

use crate::line::Line;
use crate::opcodes::{opcodes, unknown_op_code};
use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::mem;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let routes = Router::new().route("/", post(handler));

    let addr = format!("127.0.0.1:{}", 9999);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
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

enum State {
    ReadOpCode,
    ReadByte,
}

struct Disassembler {
    current_memory_location: u64,
    lines: Vec<Line>,
    current_line: Line,
    state: State,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            current_memory_location: 0,
            lines: vec![],
            current_line: Line::new(0),
            state: State::ReadOpCode,
        }
    }
}

fn disassemble(data: Vec<u8>) -> Output {
    // process the incoming data here and return type Output

    let disassembler = data
        .iter()
        .fold(Disassembler::new(), |mut disassembler, b| {
            match disassembler.state {
                State::ReadOpCode => {
                    disassembler
                        .current_line
                        .set_op_code(opcodes().get(b).unwrap_or(unknown_op_code()).clone());

                    disassembler.current_line.set_op_code_byte(*b);
                    disassembler.state = State::ReadByte;
                }
                State::ReadByte => disassembler.current_line.add_byte(*b),
            }

            disassembler.current_memory_location += 1;

            if disassembler.current_line.bytes.len()
                == disassembler.current_line.op_code.num_bytes as usize
            {
                disassembler
                    .lines
                    .push(mem::take(&mut disassembler.current_line));

                disassembler.current_line.memory_location = disassembler.current_memory_location;
                disassembler.state = State::ReadOpCode;
            }

            disassembler
        });

    Output {
        disassembly: disassembler.lines.iter().map(ToString::to_string).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[tokio::test]
    async fn test_api_disassemble_ok() {
        const URL: &str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let payload = Payload {
            data: [0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba].to_vec(),
        };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Output = Output {
            disassembly: [
                "0x0000 a9 bd        LDA #$bd",
                "0x0002 a0 bd        LDY #$bd",
                "0x0004 20 28 ba     JSR $ba28",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };
        assert_eq!(expected, res);
    }

    #[tokio::test]
    async fn test_api_disassemble_file1() {
        const URL: &str = "http://localhost:9999/";
        let client = reqwest::Client::builder().build().unwrap();

        let contents = fs::read("test-bin/test1.bin").unwrap();

        let payload = Payload { data: contents };

        let res: Output = client
            .post(URL)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let expected: Output = Output {
            disassembly: [
                "0x0000 48           PHA",
                "0x0001 e7           ???",
                "0x0002 20 20 70     JSR $7020",
                "0x0005 21 61        AND ($61,X)",
                "0x0007 00           BRK",
                "0x0008 f8           SED",
                "0x0009 ee 61 e6     INC $e661",
                "0x000C 61 00        ADC ($00,X)",
                "0x000E 04           ???",
                "0x000F 02           ???",
                "0x0010 22           ???",
                "0x0011 6e 00 84     ROR $8400",
                "0x0014 41 e9        EOR ($e9,X)",
                "0x0016 00           BRK",
                "0x0017 16 74        ASL $74,X",
                "0x0019 07           ???",
                "0x001A 0c           ???",
                "0x001B 00           BRK",
                "0x001C 00           BRK",
                "0x001D 44           ???",
                "0x001E 67           ???",
                "0x001F 18           CLC",
                "0x0020 41 e8        EOR ($e8,X)",
                "0x0022 00           BRK",
                "0x0023 20 74 06     JSR $0674",
                "0x0026 0c           ???",
                "0x0027 00           BRK",
                "0x0028 00           BRK",
                "0x0029 41 67        EOR ($67,X)",
                "0x002B 0c           ???",
                "0x002C 45 e9        EOR $e9",
                "0x002E 00           BRK",
                "0x002F 06 0c        ASL $0c",
                "0x0031 00           BRK",
                "0x0032 00           BRK",
                "0x0033 55 67        EOR $67,X",
                "0x0035 1e 60 38     ASL $3860,X",
            ]
            .iter()
            .map(|&s| s.into())
            .collect(),
        };
        assert_eq!(expected, res);
    }
}
