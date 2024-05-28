use axum::{
    body::Body, extract::State, http::StatusCode, response::IntoResponse, response::Response,
    routing::get, routing::post, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::{
    net::TcpListener,
    //sync::Mutex
};

const ADDRESS: &str = "127.0.0.1:8888";
const ADDRESSES_PATH: &str = "/addresses";

#[tokio::main]
async fn main() {
    let userlist: Arc<Mutex<Vec<UserInfo>>> = Arc::new(Mutex::new(vec![UserInfo {
        username: "haruki7049".to_string(),
        phone_hash: vec!["test_hash".to_string()],
    }]));

    let app: Router = Router::new()
        .route(ADDRESSES_PATH, get(addresses))
        .route("/", get(homepage))
        .route("/push_user", post(push_user))
        .with_state(userlist);

    let listener: TcpListener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

    println!("Serve in 'http://{}'...", ADDRESS);
    axum::serve(listener, app).await.unwrap();
}

async fn addresses(State(userlist): State<Arc<Mutex<Vec<UserInfo>>>>) -> impl IntoResponse {
    serde_json::to_string(&userlist).expect("failed to serialize")
}

async fn push_user(
    State(userlist): State<Arc<Mutex<Vec<UserInfo>>>>,
    Json(request): Json<Arc<UserInfo>>,
) -> impl IntoResponse {
    let mut userlist = userlist.lock().unwrap();
    userlist.push(UserInfo {
        username: request.username.clone(),
        phone_hash: request.phone_hash.clone(),
    });

    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

async fn homepage() -> impl IntoResponse {
    "there is no data... Please go to /addresses !!".to_string()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserInfo {
    username: String,
    phone_hash: Vec<String>,
}
