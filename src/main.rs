use axum::{
    async_trait,
    extract::{
        rejection::MatchedPathRejection, 
        FromRef, 
        FromRequestParts, 
        MatchedPath, 
        Path
    },
    http::request::Parts,
    response::{IntoResponse, Html},
    routing::get,
    serve, Router, RequestPartsExt,
};

use axum_template::{engine::Engine, Key, RenderHtml};
use serde::Serialize;
use tera::Tera;
use tokio::net::TcpListener;


// Type alias for our engine using Tera templates
type AppEngine = Engine<Tera>;


#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}

#[tokio::main]
async fn main() {
    // Create a new Tera instance and add a template from a string
    let tera = Tera::new("site/*").unwrap();

    // Build the router with app state from axum templates
    let app = Router::new()
        .route("/", get(wip_page))
        .route("/:name", get(get_name))
        .with_state(AppState {
            engine: Engine::from(tera),
        });

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



// Because Tera::new loads an entire folder, we need to remove the `/` prefix
// and add a `.html` suffix. We can implement our own custom key extractor that
// transform the key
pub struct CustomKey(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for CustomKey
where
    S: Send + Sync,
{
    type Rejection = MatchedPathRejection;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let key = parts
            // `axum_template::Key` internally uses `axum::extract::MatchedPath`
            .extract::<MatchedPath>()
            .await?
            .as_str()
            // Cargo doesn't allow `:` as a file name
            .replace(":", "$")
            .chars()
            // Remove the first character `/`
            .skip(1)
            // Add the `.html` suffix
            .chain(".html".chars())
            .collect();
        Ok(CustomKey(key))
    }
}

#[derive(Debug, Serialize)]
pub struct Person {
    name: String,
}

async fn get_name(
    // Obtain the engine
    engine: AppEngine,
    // Extract the key
    CustomKey(template): CustomKey,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let person = Person { name };

    RenderHtml(template, engine, person)
}

async fn wip_page() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html> 
<html lang="en"> 
    <head> 
        <meta charset="UTF-8"> 
        <meta name="viewport" content="width=device-width, initial-scale=1.0"> 
        <title>WIP</title> 
    </head> 
    <body> 
        <h1> 
            Please come back soon, this page is a work in progress! 
        </h1> 
    </body> 
</html> 
    "#)
}