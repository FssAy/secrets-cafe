use std::net::SocketAddr;
use std::path::Path;
use once_cell::sync::OnceCell;
use tokio::fs::OpenOptions;
use tokio::io::*;

const CONFIG_FILE_PATH: &'static str = "config.json";

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_address: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    }
}

impl Config {
    async fn load() -> Result<Self> {
        let config_path = Path::new(CONFIG_FILE_PATH);

        let mut config = if !config_path.is_file() {
            warn!("Missing config file [{}], creating a default one.", config_path.display());
            let config = Config::default();

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(config_path)
                .await?;

            file.write_all(
                serde_json::to_string_pretty(&config)
                    .unwrap()
                    .as_bytes()
            ).await?;

            config
        } else {
            let mut buffer = String::new();

            OpenOptions::new()
                .read(true)
                .open(config_path)
                .await?
                .read_to_string(&mut buffer)
                .await?;

            match serde_json::from_str::<Self>(buffer.as_str()) {
                Ok(config) => config,
                Err(err) => return Err(
                    Error::new(ErrorKind::Other, format!("Invalid JSON structure for the config file: {}", err))
                ),
            }
        };

        config.fix();

        Ok(config)
    }

    /// Used to fix minor config settings.
    fn fix(&mut self) {}

    /// Tries to initialize the config.
    ///
    /// Run only once!
    /// Some weird behaviour might occur when calling this function twice!
    pub async fn init() -> Result<&'static Self> {
        if let Some(config) = CONFIG.get() {
            return Ok(config);
        }

        let config = Self::load().await?;
        let _ = CONFIG.set(config);

        Ok(CONFIG.get().unwrap())
    }

    /// Returns the config.
    ///
    /// # Errors
    /// None variant will be returned only if the config hasn't been initialized yet.
    pub async fn get() -> Option<&'static Self> {
        CONFIG.get()
    }
}
