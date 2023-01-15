
use axum::{
    extract::Path,
    routing::get,
    Json,
    Router,
};

use axum_extra::routing::SpaRouter;

use serde::{Deserialize, Serialize};

/// This is basicallty the simplest example:
#[tokio::main]
async fn main() {

    // Define router for static content - would prbably 
    let spa = SpaRouter::new("/static", "assets");

    // build our application 
    let app = Router::new()
        .merge(spa)
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello/:name", get(hello_name))
        .route("/hello/:name/:id", get(hello_name_and_number))
        .route("/hello-struct/:name/:number", get(hello_with_struct))
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


/// Simple param take iii - take 2 parameters mapped from same URL 
/// NB dont double-parenthesis on param, the inner ones are for the tuple
/// This is cheating a bit as we've "struct'd" the call on the function
/// Fair, the "struct-ing" from Path is not critical and not strictly necessary or convenient
async fn hello_with_struct(Path((id, n)): Path<(String, u32)>) -> Json<NameNumber >{

    let x = if n % 2 == 0 {
        Some(String::from("Even number provided"))
    } else {
        None
    };
    Json(NameNumber{ name: id, number: n, description: x })
}


/// Simple struct - can we map into path-params 
#[derive(Deserialize, Serialize, Debug)]
pub struct NameNumber {
    name: String,
    number: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>
}

