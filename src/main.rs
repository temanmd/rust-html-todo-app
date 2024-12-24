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
    let result = UsersTemplate { users: &users };
    Html(result.render().unwrap())
}

async fn handler() -> Html<String> {
    Html("<h1>Hello, World!</h1>".to_string())
}

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate<'a> {
    users: &'a [User],
}
