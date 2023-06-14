use std::fmt::Debug;

use axum::{Extension, body::StreamBody, response::{IntoResponse, AppendHeaders}, extract::Path, TypedHeader};
use http::{header::{CONTENT_TYPE, CONTENT_DISPOSITION}, StatusCode};
use tokio_util::io::ReaderStream;
use tracing::info;

use crate::{service::compilation::{CompilationService, CompilationError}, extract::XUserId};

#[tracing::instrument(skip(service))]
pub async fn post_compile<T>(Extension(service): Extension<T>, raw_text: String) -> Result<impl IntoResponse, impl IntoResponse>
where 
    T: CompilationService + Debug,
    <T as CompilationService>::CompileOptions: From<String>,
{
    info!("Received compilation attempt");
    let file = match service.compile(raw_text.into()).await {
        Ok(file) => file,
        Err(CompilationError::Message(msg)) => return Err((StatusCode::UNPROCESSABLE_ENTITY, msg)),
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))
    };

    info!("Compiled file at {:?}", file);

    let headers = AppendHeaders([
        (CONTENT_TYPE, "application/pdf"),
        (CONTENT_DISPOSITION, "inline")
    ]);

    info!("Returning compiled file at {:?}", file);
    
    Ok((headers, StreamBody::new(ReaderStream::new(file))))
}

#[tracing::instrument(skip(service))]
pub async fn post_project_compile<T>(
    Extension(service): Extension<T>, 
    TypedHeader(XUserId(user_id)): TypedHeader<XUserId>, 
    Path(project_id): Path<i32>, 
    raw_text: String
) -> Result<impl IntoResponse, impl IntoResponse>
where 
    T: CompilationService,
    <T as CompilationService>::CompileOptions: From<(i32, i32, String)>,
{
    info!("Received compilation attempt");
    let file = match service.compile((user_id, project_id, raw_text).into()).await {
        Ok(file) => file,
        Err(CompilationError::Message(msg)) => return Err((StatusCode::UNPROCESSABLE_ENTITY, msg)),
        Err(CompilationError::NoAccess) => return Err((StatusCode::FORBIDDEN, "".to_string())),
        Err(CompilationError::ProjectNotFound) => return Err((StatusCode::NOT_FOUND, "".to_string())),
        Err(CompilationError::Unknown) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))
    };

    info!("Compiled file at {:?}", file);

    let headers = AppendHeaders([
        (CONTENT_TYPE, "application/pdf"),
        (CONTENT_DISPOSITION, "inline")
    ]);

    info!("Returning compiled file at {:?}", file);

    Ok((headers, StreamBody::new(ReaderStream::new(file))))
}
