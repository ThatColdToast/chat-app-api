use axum::{extract::State, response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use tinytemplate::TinyTemplate;

#[derive(Debug, Serialize)]
struct User {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    // author: User,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageTemplate {
    author: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Clone)]
struct AppState {
    db: Surreal<Client>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    // dotenv::dotenv().ok();

    let db = Surreal::new::<Ws>(format!(
        "{}:{}",
        std::env::var("SURREAL_HOST").expect("SURREAL_HOST not set"),
        std::env::var("SURREAL_PORT").expect("SURREAL_PORT not set")
    ))
    .await
    .expect("Failed to connect to server");

    db.signin(Root {
        username: std::env::var("SURREAL_USER")
            .expect("SURREAL_USER not set")
            .as_str(),
        password: std::env::var("SURREAL_PASS")
            .expect("SURREAL_PASS not set")
            .as_str(),
    })
    .await
    .expect("Failed to sign in");

    db.use_ns("chat")
        .use_db("chat")
        .await
        .expect("Failed to use namespace");

    let app_state = AppState { db };

    let app = Router::new()
        .route("/", get(root))
        .route("/api/chat", get(get_messages).post(post_message))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("html/index.html"))
}

async fn get_messages(State(app_state): State<AppState>) -> Html<String> {
    let messages: Vec<Message> = match app_state.db.select("message").await {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            return Html("Error fetching messages".to_string());
        }
    };

    let message = match messages.last() {
        Some(c) => c,
        None => return Html("No messages".to_string()),
    };

    let chat = MessageTemplate {
        author: "Username".to_string(),
        body: message.body.clone(),
    };

    let mut tt = TinyTemplate::new();
    tt.add_template("chat", include_str!("html/chat.html"))
        .unwrap();

    let result: String = messages
        .iter()
        .map(|m| MessageTemplate {
            author: "Username".to_string(),
            body: m.body.clone(),
        })
        .map(|mt| tt.render("chat", &mt).unwrap())
        .collect();

    Html(result)
}

async fn post_message(State(app_state): State<AppState>, req: String) -> Html<String> {
    // println!("{:?}", req);

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

    let messages: Vec<Record> = match app_state
        .db
        .create("message")
        .content(Message {
            // author: User {
            //     name: "Username".to_string(),
            // },
            body: req,
        })
        .await
    {
        Ok(c) => c,
        Err(e) => {
            println!("{:?}", e);
            return Html("Error".to_string());
        }
    };

    Html("Success".to_string())
}
