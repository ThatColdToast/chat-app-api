use std::time::Instant;

use axum::{routing::get, Router};
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Chat {
    name: String,
    body: String,
}

#[tokio::main]
async fn main() {
    // Create a new Axum router
    let app = Router::new()
        // Define a route for the root path that responds with "Hello, World!"
        .route("/api/chat", get(get_chat).post(post_chat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_chat() -> String {
    let start = Instant::now();

    let chat_template = include_str!("chat.html");
    let mut tt = TinyTemplate::new();
    tt.add_template("chat", chat_template).unwrap();

    let chat = Chat {
        name: "Username".to_string(),
        body: "Message 1".to_string(),
    };

    let result = tt.render("chat", &chat).unwrap();

    let duration = start.elapsed();
    println!("{:?}", duration);
    result
}

async fn post_chat() -> String {
    let start = Instant::now();

    let chat = Chat {
        name: "Username".to_string(),
        body: "Message 1".to_string(),
    };

    let duration = start.elapsed();
    println!("{:?}", duration);
    "Success".to_string()
}

// async fn get_chats() -> &'static str {
//     let chat_template = include_str!("chat.html");
//     let mut tt = TinyTemplate::new();
//     tt.add_template("chat", chat_template);

//     let chat = Chat {
//         name: "Username".to_string(),
//         body: "Message".to_string(),
//     };

//     tt.render("hello", &chat)
// }
