use std::fmt::Debug;

use axum::async_trait;
use tokio::fs::{self, File};
use tracing::error;

use crate::{
    constants::{LATEXMK_PATH, MAIN_DOC_NAME, TMP_FILE_DIR},
    repository::{projects::ProjectRepository, PutError},
    utils::get_project_path,
};

use super::execution::{ExecutionError, ExecutionService};

#[derive(Debug)]
pub enum CompilationError {
    Unknown,
    ProjectNotFound,
    NoAccess,
    Message(String),
}

#[async_trait]
pub trait CompilationService {
    type CompileOptions;

    async fn compile(&self, options: Self::CompileOptions) -> Result<File, CompilationError>;
}

#[derive(Debug, Clone)]
pub struct SimpleCompilationService<T: ExecutionService> {
    executor: T,
}

impl<T: ExecutionService> SimpleCompilationService<T> {
    pub fn new(executor: T) -> Self {
        Self { executor }
    }
}

#[async_trait]
impl<T: ExecutionService + Send + Sync> CompilationService for SimpleCompilationService<T> {
    type CompileOptions = String;

    #[tracing::instrument(skip(self))]
    async fn compile(&self, raw_text: Self::CompileOptions) -> Result<File, CompilationError> {
        let rand_id = rand::random::<u32>();

        let input_path = TMP_FILE_DIR.join(format!("{}.tex", rand_id));

        if let Err(err) = fs::write(&input_path, raw_text).await {
            error!(%err);
            return Err(CompilationError::Unknown);
        }

        let output_path = TMP_FILE_DIR.join(rand_id.to_string());

        let args = [
            format!("-outdir={}", output_path.to_str().unwrap()),
            "-pdf".to_string(),
            "-logfilewarninglist".to_string(),
            input_path.to_str().unwrap().to_owned(),
        ];

        match self.executor.execute(LATEXMK_PATH.as_str(), &args).await {
            Err(ExecutionError::Unknown) => return Err(CompilationError::Unknown),
            Err(ExecutionError::MessageError(msg)) => return Err(CompilationError::Message(msg)),
            Ok(_) => (),
        };

        File::open(output_path.join(format!("{}.pdf", rand_id)))
            .await
            .map_err(|err| {
                error!(%err);
                CompilationError::Unknown
            })
    }
}

#[derive(Clone)]
pub struct ProjectCompilationService<E, P>
where
    E: ExecutionService + Send + Sync,
    P: ProjectRepository + Send + Sync,
{
    executor: E,
    project_repository: P,
}

impl<E, P> ProjectCompilationService<E, P>
where
    E: ExecutionService + Send + Sync,
    P: ProjectRepository + Send + Sync,
{
    pub fn new(executor: E, project_repository: P) -> Self {
        Self {
            executor,
            project_repository,
        }
    }
}

#[async_trait]
impl<E, P> CompilationService for ProjectCompilationService<E, P>
where
    E: ExecutionService + Send + Sync,
    P: ProjectRepository + Send + Sync,
{
    type CompileOptions = (i32, i32, String);

    #[tracing::instrument(skip(self))]
    async fn compile(
        &self,
        (user_id, project_id, raw_text): Self::CompileOptions,
    ) -> Result<File, CompilationError> {
        let rand_id = rand::random::<u32>();

        let input_path = get_project_path(project_id).join(MAIN_DOC_NAME.as_str());

        match self
            .project_repository
            .put_doc_content(user_id, project_id, raw_text)
            .await
        {
            Ok(_) => (),
            Err(PutError::Missing) => return Err(CompilationError::ProjectNotFound),
            Err(PutError::NoAccess) => return Err(CompilationError::NoAccess),
            Err(PutError::Unknown) => return Err(CompilationError::Unknown),
        };

        let output_path = TMP_FILE_DIR.join(rand_id.to_string());

        let args = [
            format!("-outdir={}", output_path.to_str().unwrap()),
            "-pdf".to_string(),
            "-logfilewarninglist-".to_string(),
            input_path.to_str().unwrap().to_owned(),
        ];

        match self.executor.execute(LATEXMK_PATH.as_str(), &args).await {
            Err(ExecutionError::Unknown) => return Err(CompilationError::Unknown),
            Err(ExecutionError::MessageError(msg)) => return Err(CompilationError::Message(msg)),
            Ok(_) => (),
        };

        File::open(output_path.join("main.pdf"))
            .await
            .map_err(|err| {
                error!(%err);
                CompilationError::Unknown
            })
    }
}
