use std::{ffi::OsStr, fmt::Debug};

use axum::async_trait;
use tokio::process::Command;
use tracing::{error, info};

pub enum ExecutionError {
    Unknown,
    MessageError(String),
}

#[async_trait]
pub trait ExecutionService {
    async fn execute<I, S>(
        &self,
        comm: impl AsRef<OsStr> + Debug + Send,
        args: I,
    ) -> Result<String, ExecutionError>
    where
        I: IntoIterator<Item = S> + Debug + Send,
        S: AsRef<OsStr>;
}

#[derive(Debug, Clone)]
pub struct ProcessExecutionService;

#[async_trait]
impl ExecutionService for ProcessExecutionService {
    #[tracing::instrument]
    async fn execute<I, S>(
        &self,
        comm: impl AsRef<OsStr> + Debug + Send,
        args: I,
    ) -> Result<String, ExecutionError>
    where
        I: IntoIterator<Item = S> + Debug + Send,
        S: AsRef<OsStr>,
    {
        info!("Received command.");

        let command = Command::new(comm).args(args).output();

        let out = match command.await {
            Ok(out) => out,
            Err(err) => {
                error!(%err);
                return Err(ExecutionError::Unknown);
            }
        };

        let msg = match String::from_utf8(out.stdout) {
            Ok(msg) => msg,
            Err(err) => {
                error!(%err);
                return Err(ExecutionError::Unknown);
            }
        };

        if !out.status.success() {
            Err(ExecutionError::MessageError(msg))
        } else {
            Ok(msg)
        }
    }
}
