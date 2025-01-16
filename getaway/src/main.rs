use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};

use axum::{Form, Router};
use serde::Deserialize;
use serde_json::json;
use std::sync::LazyLock;
use tera::Tera;

static TEMPLATES: LazyLock<tera::Tera> = LazyLock::new(|| {
    let root = include_str!("./templates/index.html");
    let words = include_str!("./templates/words.html");

    let mut tera = match Tera::new("./templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing errors: {e}");
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html"]);

    tera.add_raw_templates(vec![("index.html", root), ("words.html", words)])
        .expect("Template does not error");

    println!("Registered templates:");
    for template in tera.get_template_names() {
        println!("{template}");
    }
    tera
});

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    let builder = reqwest::ClientBuilder::new();
    builder.build().expect("ClientBuilder does not error")
});

const BACKEND_URL: &str = "http://localhost:3000/rob";

fn common_context() -> tera::Context {
    tera::Context::new()
}

async fn root() -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut context = common_context();
    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");

    Ok(TEMPLATES
        .render("index.html", &context)
        .map(Html)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())))
}

#[derive(Deserialize, Debug)]
pub struct Boggle {
    boggle: String,
}
async fn boggle_rob(Form(boggle): Form<Boggle>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let boggle = boggle.boggle.to_lowercase();
    println!("{boggle:?}");
    let mut context = common_context();
    let response = HTTP_CLIENT
        .post(BACKEND_URL)
        .header("Content-Type", "application/json")
        .header("User-Agent", "BoggleRobber")
        .body(json!(boggle).to_string())
        .send()
        .await
        .map_err(|e| {
            println!("Got an error querying backend: {e:?}");
            (e.status().unwrap(), "Backend failed".to_string())
        })?;

    println!("Response: {response:?}");
    let words = response.json::<Vec<String>>().await.unwrap();
    println!("Got words: {words:?}");
    context.insert("words", &words);

    Ok(TEMPLATES
        .render("words.html", &context)
        .map(Html)
        .map_err(|e| {
            println!("Got an error mapping the template to HTML: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/boggle/rob", post(boggle_rob));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
