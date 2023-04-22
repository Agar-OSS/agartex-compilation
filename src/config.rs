use std::{io::BufReader, fs::File, path::{Path, PathBuf}, net::SocketAddr};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub tmp_file_dir: PathBuf,
    pub latexmk_path: PathBuf
}

const CONFIG_PATH: &str = "config.json";

pub const SERVER_ADDR_ENV_VAR: &str = "SERVER_ADDR";
pub const TMP_FILE_DIR_ENV_VAR: &str = "TMP_FILE_DIR";
pub const LATEXMK_PATH_ENV_VAR: &str = "LATEXMK_PATH";

fn load_config() -> Config {
    let mut cfg: Config = serde_json::from_reader(BufReader::new(File::open(Path::new(CONFIG_PATH)).unwrap())).unwrap();

    if let Ok(server_addr) = std::env::var(SERVER_ADDR_ENV_VAR) {
        cfg.server_addr = server_addr.parse().unwrap();
    }

    if let Ok(tmp_file_dir) = std::env::var(TMP_FILE_DIR_ENV_VAR) {
        cfg.tmp_file_dir = tmp_file_dir.parse().unwrap();
    }

    if let Ok(latexmk_path) = std::env::var(LATEXMK_PATH_ENV_VAR) {
        cfg.latexmk_path = latexmk_path.parse().unwrap();
    }

    cfg
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}
