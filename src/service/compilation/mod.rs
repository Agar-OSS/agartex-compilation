use std::{path::PathBuf, fmt::Debug, fs};

use axum::async_trait;
use tracing::error;

use crate::config::CONFIG;

use super::execution::{ExecutionService, ProcessExecutionService, ProcessExecutionError};

#[async_trait]
pub trait CompilationService {
    type CompileOptions;
    type CompilationError: Debug;

    async fn compile(&self, options: Self::CompileOptions) -> Result<PathBuf, Self::CompilationError>;
}

#[derive(Debug, Clone)]
pub struct SimpleCompilationService<T: ExecutionService> {
    executor: T,
}

impl<T: ExecutionService> SimpleCompilationService<T> {
    pub fn new(executor: T) -> Self {
        fs::create_dir_all(&CONFIG.tmp_file_dir).unwrap();
        Self {
            executor
        }
    }
}

#[derive(Debug)]
pub enum SimpleCompilationError {
    Unexpected,
    Message(String)
}

impl From<SimpleCompilationError> for String {
    fn from(err: SimpleCompilationError) -> String {
        match err {
            SimpleCompilationError::Unexpected => "UNKNOWN ERROR".to_owned(),
            SimpleCompilationError::Message(msg) => msg
        }
    }
}

#[async_trait]
impl CompilationService for SimpleCompilationService<ProcessExecutionService> {
    type CompileOptions = String;
    type CompilationError = SimpleCompilationError;
    
    #[tracing::instrument]
    async fn compile(&self, raw_text: Self::CompileOptions) -> Result<PathBuf, Self::CompilationError> {
        let rand_id = rand::random::<u32>();
        
        let input_path = CONFIG.tmp_file_dir.join(format!("{}.tex", rand_id));

        if let Err(err) = fs::write(&input_path, raw_text) {
            error!(%err);
            return Err(SimpleCompilationError::Unexpected);
        }

        let output_path = CONFIG.tmp_file_dir.join(rand_id.to_string());

        let args = [
            format!("-outdir={}", output_path.to_str().unwrap()),
            "-pdf".to_string(),
            "-logfilewarninglist".to_string(),
            input_path.to_str().unwrap().to_owned()
        ];
        
        match self.executor.execute(&CONFIG.latexmk_path, &args).await {
            Err(ProcessExecutionError::Unknown) => return Err(SimpleCompilationError::Unexpected),
            Err(ProcessExecutionError::StatusError(_, msg)) => return Err(SimpleCompilationError::Message(msg)),
            Ok(_) => ()
        };

        return Ok(output_path.join(format!("{}.pdf", rand_id)));
    }
}
