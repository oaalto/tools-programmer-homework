use axum::{routing::post, Router};
use tokio::net::TcpListener;
use tools_programmer_homework::{handler, middleware};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let routes = Router::new()
        .route("/", post(handler::disassemble))
        .route_layer(axum::middleware::from_fn(middleware::disassembler));

    let addr = format!("127.0.0.1:{}", 9999);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("{:<15} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
