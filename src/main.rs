use axum::{
    extract::Path,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{fs, net::SocketAddr, path::PathBuf};

#[tokio::main]
async fn main() {
    // Build routes
    let app = Router::new()
        .route("/", get(serve_file))
        .route("/*path", get(serve_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Serving portfolio on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_file(uri: Uri, Path(path): Path<String>) -> impl IntoResponse {
    // Map URI to file path inside src/view
    let mut file_path = PathBuf::from("src/view");

    // Default to index.html
    if path.is_empty() {
        file_path.push("index.html");
    } else {
        file_path.push(&path);
    }

    // If path doesn’t end with .html, try adding it
    if !file_path.extension().is_some() {
        file_path.set_extension("html");
    }

    // Try reading the file
    match fs::read_to_string(&file_path) {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Html("<h1>404 - Page Not Found</h1><p>Oops! Page doesn't exist.</p>").into_response(),
        )
            .into_response(),
    }
}
