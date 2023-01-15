
use axum::{
    routing::get,
    Router,
    extract::Path
};


/// This is basicallty the simplest example:
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello/:name", get(hello_name))
        .route("/hello/:name/:id", get(hello_name_and_number))
        ;

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Simple param1: single parameter mapped from URL
async fn hello_name(Path(id): Path<String>) -> String {
    println!("I got {id}");
    format!("Hello {id} I hope you are well")
}


/// Simple param take ii - take 2 parameters mapped from same URL 
/// NB dont double-parenthesis on param, the inner ones are for the tuple
async fn hello_name_and_number(Path((id, n)): Path<(String, u32)>) -> String {
    println!("I got {id}/{n}");
    format!("Hello {id}, the number you supplied was {n}")
}