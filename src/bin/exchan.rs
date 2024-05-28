use axum::{routing::get, Router};
use tokio::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:8888";
const ADDRESSES_PATH: &str = "/addresses";

#[tokio::main]
async fn main() {
    let userlist: Vec<UserInfo> = vec![
        UserInfo {
            username: "haruki7049".to_string(),
            phone_hash: vec!["test_hash".to_string()],
        },
    ];

    let app: Router = Router::new()
        .route(ADDRESSES_PATH, get(addresses))
        .route("/", get(homepage));

    let listener: TcpListener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();

    println!("Serve in 'http://{}'...", ADDRESS);
    axum::serve(listener, app).await.unwrap();
}

async fn addresses() -> String {
    "No addresses".to_string()
}

async fn homepage() -> String {
    "there is no data... Please go to /addresses !!".to_string()
}

#[derive(Debug)]
struct UserInfo {
    username: String,
    phone_hash: Vec<String>,
}
