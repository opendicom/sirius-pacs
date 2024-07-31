
use axum::{
    body::Body, 
    extract::{Path, State}, 
    http::StatusCode, 
    response::IntoResponse, 
    Json, 
};
use tokio_util::io::ReaderStream;
use serde_json::{json, Value};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    database::Pool, 
    models::*, 
    AppError,
    error::database_error
};

// region: -- study_handler ----------------------------------------------------------------------------------

pub async fn study_handler(
    Path(_study_iuid): Path<String>, 
    State(pool): State<Pool>) 
    -> Result<impl IntoResponse, AppError>
{

     // Query database
     let mut conn = pool.get().await.unwrap();
     let res = crate::schema::patients::table
         .select(Patient::as_select())
         .load(&mut conn)
         .await
         .map_err(database_error)?;

    for pat in res {
        println!("{:#?}",pat);
    }
    
    // `File` implements `AsyncRead`
    // `file`` Will be rewriten to get binary data from DCKV storage
    let file = tokio::fs::File::open("samples/study.ekv").await?;
    
    // Convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    
    // Convert the `Stream` into an `axum::body::HttpBody`
    let body = Body::from_stream(stream);

    Ok(body)
}
// endregion: ------------------------------------------------------------------------------------

// region: -- study_thumbnails_handler ----------------------------------------------------------------------------------

pub async fn study_thumbnails_handler(
    Path(_study_iuid): Path<String>, 
    State(_pool): State<Pool>) 
    -> Json<Value> 
{
    
    let response = json!({
        "patID": "12345678",
        "patName": "Apellido1>Apellido2^Nombre1 Nombre2",
        "patBirthDate": "AAAAMMDD",
        "studyIuid": "<IUID>",
        "studyDate": "AAAAMMDD",
        "studyTime": "HH:MM",
        "studyDesc": "Descripcion del estudio",
        "studySeries": [{
            "seriesIuid": "<IUID>",
            "seriesDesc": "Descripcion de la serie",
            "seriesInstCount": 2,
            "seriesNumber": 1,
            "modality": "CR",
            "image": "<DCKV>"
        }]
    });

    Json(response)
    //Err(StatusCode::NOT_IMPLEMENTED)
}
// endregion: ------------------------------------------------------------------------------------

// region: -- series_handler ----------------------------------------------------------------------------------

pub async fn series_handler(Path((_study_iuid,_series_iuid)): 
    Path<(String,String)>, 
    State(_pool): State<Pool>) 
    -> impl IntoResponse 
{

    // TODO: Query database
    
    // `File` implements `AsyncRead`
    // `file Will be rewriten to get binary data from DCKV storage
    let file = match tokio::fs::File::open("samples/series.ekv").await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found")),
    };
    
    // Convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    
    // Convert the `Stream` into an `axum::body::HttpBody`
    let body = Body::from_stream(stream);

    Ok(body)
}
// endregion: ------------------------------------------------------------------------------------

// region: -- instance_handler ----------------------------------------------------------------------------------

pub async fn instance_handler(
    Path((_study_iuid, _series_iuid, _sop_instance_uid)): Path<(String, String, String)>, 
    State(_pool): State<Pool>) 
    -> impl IntoResponse 
{

    // TODO: Query database
    
    // `File` implements `AsyncRead`
    // `file Will be rewriten to get binary data from DCKV storage
    let file = match tokio::fs::File::open("samples/instance.ekv").await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found")),
    };
    
    // Convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    
    // Convert the `Stream` into an `axum::body::HttpBody`
    let body = Body::from_stream(stream);

    Ok(body)
}
// endregion: ------------------------------------------------------------------------------------

