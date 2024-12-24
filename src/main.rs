use askama_axum::Template;
use axum::{response::Html, routing::get, Router};

struct User {
    id: u32,
    name: String,
}

fn get_users() -> Vec<User> {
    let mut users: Vec<User> = Vec::new();
    for i in 1..32 {
        users.push(User {
            id: i,
            name: format!("Name {i}").to_owned(),
        })
    }
    users
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", get(users_handler))
        .route("/hello", get(hello))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn users_handler() -> Html<String> {
    let users = get_users();
    let mut html: String = String::from("<table><tr><th>ID</th><th>Name</th></tr>");
    for user in users {
        html.push_str(format!("<tr><td>{}</td><td>{}</td></tr>", user.id, user.name).as_str());
    }
    Html(html)
}

async fn handler() -> Html<String> {
    Html("<h1>Hello, World!</h1>".to_string())
}

async fn hello() -> HelloTemplate<'static> {
    HelloTemplate { name: "Tema" }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
