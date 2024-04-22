use std::path::Path;

use axum::routing::post;
use axum::{routing::get, Router};
use axum::response::Html;
use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(say_hello_file))
    .route("/json", get(say_hello_json))
    .route("/post", post(say_hello_post));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                                .await
                                .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}

async fn say_hello_text() -> &'static str {
    "Hello, world!"
}

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

// static read
async fn say_hello_html_included() -> Html<&'static str> {
    const HTML: &str = include_str!("hello.html");
    Html(HTML)
}

// dynamic read... content would change if edited.. and you reload from browser
async fn say_hello_file() -> Html<String> {
    let path = Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}

async fn say_hello_post() -> &'static str {
    "Hello, POST!"
}