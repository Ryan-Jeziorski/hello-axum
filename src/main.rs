use axum::{
    response::Html, 
    routing::get, 
    Router
};
use tower_http::services::ServeDir;

async fn footer() -> Html<&'static str> {
    Html(" \
<footer> \
<h1 style=\"font-size: small;\">Site Map</h1> \
    <a href=\"./index.html\">Home</a> \
    <a href=\"../page_1/index.html\">Test</a> \
</footer> \
    ")
}

#[tokio::main]
async fn main() {
    // build our application with a route
    // Set app to server static html website.
    // TODO: Note that I'm probably doing this the wrong way at the moment
    //       but I'm still getting used to Rust and Axum
    let app = Router::new()
        .route_service("/", ServeDir::new("site/homepage"))
        .nest_service("/homepage", ServeDir::new("site/homepage"))
        .nest_service("/index.html", ServeDir::new("site/homepage"))
        .nest_service("/page_1", ServeDir::new("site/page_1"))
        .nest_service("/page_1/index.html", ServeDir::new("site/page_1"))
        .route("/footer", get(footer));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}