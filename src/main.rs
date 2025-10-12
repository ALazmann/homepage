use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{fs, net::SocketAddr, path::PathBuf};

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/:page", get(serve_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Serving homepage at http://{}", addr);

    // Axum 0.7 server start
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for root ("/")
async fn index() -> impl IntoResponse {
    serve_file("index.html".to_string()).await
}

// Handler for other pages like "/about"
async fn serve_page(Path(page): Path<String>) -> impl IntoResponse {
    let filename = format!("{}.html", page);
    serve_file(filename).await
}

// Reads HTML file from src/view or returns 404
async fn serve_file(filename: String) -> impl IntoResponse {
    let mut file_path = PathBuf::from("src/view");
    file_path.push(filename);

    match fs::read_to_string(&file_path) {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Html("<h1>404 - Page Not Found</h1><p>That page doesn’t exist.</p>"),
        )
            .into_response(),
    }
}
