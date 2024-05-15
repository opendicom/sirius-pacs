/// # SIRIUS-RS
/// Private DICOM RESTful services for Sirius ecosystem

use axum::{
    body::Bytes, extract::Path, http::StatusCode, routing::get, Router
};
use tower_http::trace::TraceLayer;
use tracing::{debug, info, instrument};


#[tokio::main]
async fn main() {

    // Setup tracing
    tracing_subscriber::fmt::init();

    // Application Builder
    let app = Router::new()
        .route("/studies/:study_iuid/thumbnails", get(study_thumbnails))
        .layer(TraceLayer::new_for_http());

    // Run Application
    info!("====================================================");
    info!("Starting SIRIUS-RS v{}",env!("CARGO_PKG_VERSION"));
    info!("====================================================");
    let bind="0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    info!("Listening on: {}",bind);
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn study_thumbnails(Path(study_iuid): Path<String>) -> Result<Bytes, StatusCode> {
    
    let response = Bytes::from("dckv");
    debug!(?response);

    Ok(response)
    //Err(StatusCode::NOT_IMPLEMENTED)
}