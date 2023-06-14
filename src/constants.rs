use std::{env, str::FromStr, fmt::Debug, net::{SocketAddr, Ipv4Addr, IpAddr}, path::PathBuf};

use http::HeaderName;
use lazy_static::lazy_static;

fn load_env_or_default<T>(var: &str, default: T) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    match env::var(var) {
        Ok(val) => T::from_str(&val).unwrap(),
        Err(_) => default
    }
}

// implicit environment variables used:
// - PGHOST
// - PGPORT
// - PGDATABASE
// - PGUSER
// - PGPASSWORD
lazy_static! {
    pub static ref SERVER_URL: SocketAddr = load_env_or_default("SERVER_URL", SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3300));
    pub static ref RESOURCE_MANAGEMENT_URL: String = load_env_or_default("RESOURCE_MANAGEMENT_URL", String::from("http://localhost:3200"));
    pub static ref USER_ID_HEADER: String = load_env_or_default("USER_ID_HEADER", String::from("X-User-Id"));
    pub static ref USER_ID_HEADER_NAME: HeaderName = HeaderName::from_str(USER_ID_HEADER.as_str()).unwrap();
    
    pub static ref LATEXMK_PATH: String = load_env_or_default("LATEXMK_PATH", String::from("latexmk"));
    pub static ref TMP_FILE_DIR: PathBuf = load_env_or_default("TMP_FILE_DIR", PathBuf::from_str("/tmp/agartex-compilation/").unwrap());
    pub static ref FILE_DIR_PATH: PathBuf =
        load_env_or_default("FILE_DIR_PATH", PathBuf::from(r"blobs"));
    pub static ref MAIN_DOC_NAME: String = load_env_or_default("MAIN_DOC_NAME", String::from("main.tex"));
}
