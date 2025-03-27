use axum::{
    routing::post,
    Router,
    extract::Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct EchoRequest {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/echo", post(echo));

    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    println!("Received payload: {:?}", payload);
    Json(payload)
}
