use axum::{Router, Extension, routing};

use crate::{service::{compilation::{SimpleCompilationService, ProjectCompilationService}, execution::ProcessExecutionService}, control::compile::{post_compile, post_project_compile}, repository::projects::HttpProjectRepository, constants::RESOURCE_MANAGEMENT_URL};

pub fn main_router() -> Router {
    let simple_compilation_service = SimpleCompilationService::new(ProcessExecutionService {});
    let project_compilation_service = ProjectCompilationService::new(
        ProcessExecutionService {}, 
        HttpProjectRepository::new(RESOURCE_MANAGEMENT_URL.clone() + "/projects/")
    );

    let compile_handler = routing::post(post_compile::<SimpleCompilationService<ProcessExecutionService>>);
    let projects_handler = routing::post(post_project_compile::<ProjectCompilationService<ProcessExecutionService, HttpProjectRepository>>);

    Router::new()
        .route("/compile", compile_handler)
        .layer(Extension(simple_compilation_service))
        .route("/projects/:project_id/pdf", projects_handler)
        .layer(Extension(project_compilation_service))
}
