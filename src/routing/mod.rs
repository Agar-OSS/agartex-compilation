use axum::{routing, Extension, Router};

use crate::{
    constants::RESOURCE_MANAGEMENT_URL,
    control::compile::{post_compile, post_project_compile},
    repository::projects::HttpProjectRepository,
    service::{
        compilation::{ProjectCompilationService, SimpleCompilationService},
        execution::ProcessExecutionService,
    },
};

pub fn main_router() -> Router {
    let simple_compilation_service = SimpleCompilationService::new(ProcessExecutionService {});
    let project_compilation_service = ProjectCompilationService::new(
        ProcessExecutionService {},
        HttpProjectRepository::new(RESOURCE_MANAGEMENT_URL.clone() + "/projects"),
    );

    let compile_handler =
        routing::post(post_compile::<SimpleCompilationService<ProcessExecutionService>>)
            .layer(Extension(simple_compilation_service));

    let projects_handler = routing::post(
        post_project_compile::<
            ProjectCompilationService<ProcessExecutionService, HttpProjectRepository>,
        >,
    )
    .layer(Extension(project_compilation_service));

    Router::new()
        .route("/compile", compile_handler)
        .route("/projects/:project_id/pdf", projects_handler)
}
