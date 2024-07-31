/// # SIRIUS-RS
/// Private DICOM RESTful services for Sirius ecosystem

use axum::{
    routing::get, 
    Router
};
use tower_http::trace::TraceLayer;
use tracing::info;

mod database;
mod error;
mod schema;
mod models;
mod handlers;

use error::{AppError, not_found_error};
use handlers::*;


#[tokio::main]
async fn main() {

    dotenvy::dotenv().expect(".env File not found");

    // Create a new database connection pool
    let pool = database::init();

    // Setup tracing
    tracing_subscriber::fmt::init();

    // Application Builder
    let app = Router::new()
        // Study
        .route("/studies/:study_iuid", get(study_handler))
        .route("/studies/:study_iuid/thumbnails", get(study_thumbnails_handler))
        
        // Series
        .route("/studies/:study_iuid/series/:series_iuid", get(series_handler))

        // Instance
        .route("/studies/:study_iuid/series/:series_iuid/instance/:sop_instance_uid", get(instance_handler))

        .fallback(not_found_error)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Run Application
    info!("====================================================");
    info!("Starting SIRIUS-RS v{}",env!("CARGO_PKG_VERSION"));
    info!("====================================================");
    let bind="0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    info!("Listening on: {}",bind);
    axum::serve(listener, app).await.unwrap();
}
