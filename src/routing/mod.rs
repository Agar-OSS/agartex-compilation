use axum::{Router, Extension};

use crate::{service::{compilation::SimpleCompilationService, execution::ProcessExecutionService}, control::compile::post_compile};

pub fn main_router() -> Router {
    let simple_compilation_service = SimpleCompilationService::new(ProcessExecutionService {});
    
    let compilation_handler = axum::routing::post(post_compile::<SimpleCompilationService<ProcessExecutionService>>);

    Router::new()
        .route("/compile", compilation_handler)
        .layer(Extension(simple_compilation_service))
}
