use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting{
    pub tor_url: String,
    pub data_dir: PathBuf,
}

impl Setting{
    pub fn new() -> Result<&self> {
        let tor_url = std::env::var("TOR_URL").unwrap_or_else(|_| "socks5://127.0.0.1:9050".to_string());
        let data_dir = PathBuf::from(
            std::env::var("DATA_DIR").unwrap_or_else(|_| "./data".to_string())
        );

        Ok(self {
            tor_url,
            data_dir
        })
    }

    pub fn ensure_dir() -> Result<()>{
        std::fs::create_dir_all(&self.data_dir)?;
        Ok(())
    }
}