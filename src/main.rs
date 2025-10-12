use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{env, fs, net::SocketAddr, path::PathBuf};

#[tokio::main]
async fn main() {
    // Define routes
    let app = Router::new()
        .route("/", get(index))
        .route("/:page", get(serve_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Serving homepage at http://{}", addr);

    // Start the Axum 0.7 server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Serve root / (homepage/index.html)
async fn index() -> impl IntoResponse {
    serve_file("index.html".to_string(), true).await
}

// Serve other pages like /about or /projects
async fn serve_page(Path(page): Path<String>) -> impl IntoResponse {
    let filename = format!("{}.html", page);
    serve_file(filename, false).await
}

// Generic HTML file loader
async fn serve_file(filename: String, root: bool) -> impl IntoResponse {
    // Get the current project directory
    let project_root = env::current_dir().unwrap();

    // Build the correct file path
    let mut file_path = if root {
        project_root.join("index.html") // homepage/index.html
    } else {
        project_root.join("src/view").join(filename)
    };

    // Try reading file
    match fs::read_to_string(&file_path) {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => {
            eprintln!("⚠️ File not found: {:?}", file_path);
            (
                StatusCode::NOT_FOUND,
                Html("<h1>404 - Page Not Found</h1><p>Oops! That page doesn’t exist.</p>"),
            )
                .into_response()
        }
    }
}
