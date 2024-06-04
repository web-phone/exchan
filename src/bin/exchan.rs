use axum::{
    body::Body, extract::State, http::StatusCode, response::IntoResponse, response::Response,
    routing::get, routing::post, Json, Router,
};
use clap::Parser;
use exchan::CommandLineArgs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:8888";
const ADDRESSES_PATH: &str = "/addresses";

#[tokio::main]
async fn main() {
    let args: CommandLineArgs = CommandLineArgs::parse();

    match (args.debug, args.production) {
        (true, false) => debug_mode().await,
        (false, true) => production_mode().await,
        _ => panic!("You must choose one mode"),
    }
}

/// A Production mode server
async fn production_mode() {
    todo!("Production mode is not implemented yet");
}

/// A Debug mode server
async fn debug_mode() {
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

    println!("Serve in 'http://{}'...\nEnter 'exit' if you want to shutdown exchan...", ADDRESS);

    // Setup Ctrl+c handler
    tokio::spawn(async move {
        wait_exit().await;
        println!("Shutting down...");
        std::process::exit(0);
    });

    axum::serve(listener, app).await.unwrap();
}

/// A handler for GET /addresses
async fn addresses(State(userlist): State<Arc<Mutex<Vec<UserInfo>>>>) -> impl IntoResponse {
    let userlist = userlist.lock().unwrap();
    let json = serde_json::to_string(&*userlist).expect("failed to serialize");
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(json))
        .unwrap()
}

/// A handler for POST /push_user
async fn push_user(
    State(userlist): State<Arc<Mutex<Vec<UserInfo>>>>,
    Json(request): Json<UserInfo>,
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

/// A handler for GET /
async fn homepage() -> impl IntoResponse {
    "there is no data... Please go to /addresses !!".to_string()
}

/// Wait to enter "exit" to shutdown the server
async fn wait_exit() {
    let mut input = String::new();
    while input.trim() != "exit" {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

/// A User information
#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserInfo {
    username: String,
    phone_hash: Vec<String>,
}
