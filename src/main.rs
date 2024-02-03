use axum::http::Method;
use axum::routing::method_routing::get;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    env_logger::builder().init();

    let trace = TraceLayer::new_for_http();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = axum::Router::new()
        // <routes here>
        .route("/", get(|| async { "hallo! ;)" }))
        .layer(tower::ServiceBuilder::new().layer(trace).layer(cors));

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
