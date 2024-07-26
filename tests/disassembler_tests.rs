use reqwest::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
struct InvalidArchitecturePayload {
    data: Vec<u8>,
    architecture: String,
}

#[tokio::test]
async fn test_api_invalid_architecture() {
    const URL: &str = "http://localhost:9999/";
    let client = reqwest::Client::builder().build().unwrap();

    let payload = InvalidArchitecturePayload {
        data: [0xa9, 0xbd, 0xa0, 0xbd, 0x20, 0x28, 0xba].to_vec(),
        architecture: "mos".to_string(),
    };

    let res = client.post(URL).json(&payload).send().await.unwrap();
    assert_eq!(StatusCode::BAD_REQUEST, res.status());
}
