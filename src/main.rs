use axum::{
    extract::{Path, Query, Request, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tinytemplate::TinyTemplate;
use url::Url;

#[derive(Serialize, Clone, Debug)]
struct Chat {
    name: String,
    body: String,
}

#[derive(Serialize, Clone, Debug)]
struct AppState {
    chats: Vec<Chat>,
}

#[tokio::main]
async fn main() {
    // let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    // let mut chats: Vec<Chat> = Vec::new();

    let appState = AppState { chats: Vec::new() };

    let app = Router::new()
        .route("/", get(root))
        .route("/api/chat", get(get_messages).post(post_message))
        .with_state(appState);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("html/index.html"))
}

async fn get_messages(State(appState): State<AppState>) -> Html<String> {
    let start = Instant::now();

    println!("{:?}", appState.chats.len());

    let chat = match appState.chats.last() {
        Some(c) => c,
        None => return Html("No messages".to_string()),
    };

    let chat_template = include_str!("html/chat.html");
    let mut tt = TinyTemplate::new();
    tt.add_template("chat", chat_template).unwrap();

    let result = tt.render("chat", chat).unwrap();

    let duration = start.elapsed();
    println!("{:?}", duration);

    Html(result)
}

async fn post_message(
    State(mut appState): State<AppState>,
    req: String,
    // Form(chatbox): Form<String>,
    // Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    println!("{:?}", req);
    let req = match urlencoding::decode(&req) {
        Ok(r) => r
            .to_string()
            .chars()
            .skip("message-box=".len())
            .collect::<String>(),
        Err(e) => {
            println!("{:?}", e);
            return Html("Error".to_string());
        }
    };
    println! {"{:?}", req};

    let chat = Chat {
        name: "Username".to_string(),
        body: req,
    };

    appState.chats.push(chat);

    Html("Success".to_string())
}
