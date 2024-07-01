use axum::{
    // response::Html, 
    // routing::get, 
    Router
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Set static html directory to server from
    // build our application with a route
    let app = Router::new()
        .route_service("/", ServeDir::new("static/homepage"))
        .nest_service("/hompage", ServeDir::new("static/homepage"))
        .nest_service("/index.html", ServeDir::new("static/homepage"))
        .nest_service("/page_1", ServeDir::new("static/page_1"))
        .nest_service("/page_1/index.html", ServeDir::new("static/page_1"));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}