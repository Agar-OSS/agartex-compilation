use std::{env, str::FromStr, fmt::Debug, net::{SocketAddr, Ipv4Addr, IpAddr}, path::PathBuf};

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
    pub static ref SERVER_URL: SocketAddr = load_env_or_default("SERVER_URL", SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3100));
    pub static ref TMP_FILE_DIR: PathBuf = load_env_or_default("TMP_FILE_DIR", PathBuf::from_str("/tmp/agartex-compilation/").unwrap());
    pub static ref LATEXMK_PATH: String = load_env_or_default("LATEXMK_PATH", String::from("latexmk"));
}
