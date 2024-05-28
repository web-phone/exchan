use axum::{extract::State, routing::get, routing::post, Router, body::Body, response::IntoResponse, response::Response, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;
use tokio::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:8888";
const ADDRESSES_PATH: &str = "/addresses";

#[tokio::main]
async fn main() {
    let userlist: Arc<Vec<UserInfo>> = Arc::new(vec![UserInfo {
        username: "haruki7049".to_string(),
        phone_hash: vec!["test_hash".to_string()],
    }]);

    let app: Router = Router::new()
        .route(ADDRESSES_PATH, get(addresses))
        .route("/", get(homepage))
        .route("/push_user", post(push_user))
        .with_state(userlist);

    let listener: TcpListener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

    println!("Serve in 'http://{}'...", ADDRESS);
    axum::serve(listener, app).await.unwrap();
}

async fn addresses(State(userlist): State<Arc<Vec<UserInfo>>>) -> impl IntoResponse {
    serde_json::to_string(&userlist).expect("failed to serialize")
}

async fn push_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

async fn homepage() -> impl IntoResponse {
    "there is no data... Please go to /addresses !!".to_string()
}

#[derive(Serialize, Debug, Clone)]
struct UserInfo {
    username: String,
    phone_hash: Vec<String>,
}
