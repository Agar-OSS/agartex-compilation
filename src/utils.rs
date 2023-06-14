use std::path::PathBuf;

use crate::constants::FILE_DIR_PATH;

#[tracing::instrument]
pub fn get_project_path(project_id: i32) -> PathBuf {
    FILE_DIR_PATH.join(project_id.to_string())
}
