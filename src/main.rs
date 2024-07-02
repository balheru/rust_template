use axum::{
    http::StatusCode,
    Json,
    Router, routing::{get, post},
};
use serde::{Deserialize, Serialize};
use lib_a::{really_complicated_code_a, test_blinky};
use lib_b::really_complicated_code_b;

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    x: i32
}

#[tokio::main]
async fn main() {
    really_complicated_code_a(1, 1);
    really_complicated_code_b(1, 1);
    println!("Hello, world!");
    test_blinky();

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:13000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root() -> &'static str {
    "Hello, World!"
}
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

