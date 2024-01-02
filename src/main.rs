use std::future::Future;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/what", get(what_is_this))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!\n"
}
async fn what_is_this() -> &'static str {
    "keep clone\n"
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    // insert your application logic here

    if payload.username.is_empty() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let user = User {
        id: 1337,
        username: payload.username,
    };
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(user)))
    // Err::<StatusCode>(StatusCode::ALREADY_REPORTED)
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    #[serde(rename = "name")]
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
