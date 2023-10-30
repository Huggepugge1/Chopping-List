use std::fs::{read_to_string, File};
use std::io::Write;

use tokio;
use axum::{
    routing::get,
    routing::post,
    Router,
    response::Json,
};
use serde::{
    Deserialize,
    Serialize,
};
use serde_json;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
enum Unit {
    G,
    HG,
    KG,

    ML,
    CL,
    DL,
    L,

    ST,
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
struct Grocery {
    typ: String,
    amount: i32,
    unit: Unit,
}

macro_rules! static_file {
    ($name:ident, $path:literal, $ctype:expr) => {
        async fn $name() -> (axum::http::StatusCode, axum::http::HeaderMap, Vec<u8>) {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static($ctype),
            );
            #[cfg(not(debug_assertions))]
            let file = include_bytes!($path).to_vec();
            #[cfg(debug_assertions)]
            let file = tokio::fs::read(concat!("static/", $path))
                .await
                .expect(concat!(
                    "Program is in debug mode and the ",
                    $path,
                    " file was not found!"
                ));
            (axum::http::StatusCode::OK, headers, file)
        }
    };
}

async fn get_groceries() -> Json<String> {
    let json = read_to_string("./groceries.json").unwrap();
    Json(json)
}

fn json_to_groceries(json: String) -> Result<Vec<Grocery>, serde_json::Error> {
    serde_json::from_str::<Vec<Grocery>>(&json)
}

async fn add_grocery(body: axum::extract::Json<Grocery>) -> Json<String> {
    println!("alksdjf");
    println!("{:?}", body);
    Json(String::from("Hello World!"))
}

#[tokio::main]
async fn main() {
    static_file!(root, "index.html", "text/html");
    static_file!(inventory, "inventory.html", "text/html");
    static_file!(style, "css/style.css", "text/css");
    static_file!(grocery, "js/grocery.js", "application/javascript");
    static_file!(inventory_js, "js/inventory.js", "application/javascript");

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/inventory", get(inventory))
        .route("/style.css", get(style))
        .route("/grocery.js", get(grocery))
        .route("/inventory.js", get(inventory_js))
        .route("/get_groceries", get(get_groceries))
        .route("/add_grocery", post(move |body| {println!("Hello"); add_grocery(body)}));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
