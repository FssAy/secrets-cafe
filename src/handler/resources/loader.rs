// todo: refactor

use super::*;
use std::path::{Path, PathBuf};
use hyper::header::HeaderValue;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

const FRONTEND_FOLDER_PATH: &str = "frontend";
const RESOURCE_SETTINGS_FILE: &str = "resources.json";

/// Structure of the settings file with paths to frontend files to load and expose.
#[derive(Deserialize, Debug, Clone)]
pub struct ResourceSettings {
    /// HTML pages.
    pub pages: Vec<PathBuf>,
    /// CSS style files.
    pub styles: Vec<PathBuf>,
    /// JavaScript files.
    pub scripts: Vec<PathBuf>,
    /// Any other file that should be exposed.
    pub other: Vec<PathBuf>,
}

impl ResourceSettings {
    // todo: error handling
    /// Loads the resource settings from the file.
    pub async fn from_file() -> Self {
        let path = Path::new(RESOURCE_SETTINGS_FILE);
        let file_contents = Self::read_frontend_file(path).await;
        serde_json::from_slice::<Self>(&file_contents).unwrap()
    }

    // todo: error handling
    /// Reads through the settings and loads frontend files into the `ResourceMap`.
    pub async fn into_resource_map(self) -> ResourceMap {
        let mut map = ResourceMap::new();

        for page_path in self.pages {
            Self::load_and_insert_resource(&mut map, page_path, "text/html").await;
        }

        for style_path in self.styles {
            Self::load_and_insert_resource(&mut map, style_path, "text/css").await;
        }

        for script_path in self.scripts {
            Self::load_and_insert_resource(&mut map, script_path, "text/javascript").await;
        }

        for path in self.other {
            let mime = file_name_to_mime(
                path.file_name().unwrap().to_str().unwrap()
            );

            let resource = ResourceEndpoint {
                blob: vec![],
                mime: HeaderValue::from_str(mime).unwrap(),
            };

            map.insert(
                Self::parse_file_path(path),
                resource,
            );
        }

        #[cfg(debug_assertions)] {
            for (key, value) in &map {
                debug!("Exposing endpoint: [{}] with mime [{:?}]", key, value.mime);
            }
        }

        map
    }

    // todo: error handling
    /// Reads a specific frontend file and puts it inside the `ResourceMap`.
    async fn load_and_insert_resource(map: &mut ResourceMap, path: impl AsRef<Path>, mime: &'static str) {
        let path = path.as_ref();

        let resource = ResourceEndpoint {
            blob: Self::read_frontend_file(path).await,
            mime: HeaderValue::from_str(mime).unwrap(),
        };

        map.insert(
            Self::parse_file_path(path),
            resource,
        );
    }

    // todo: error handling
    /// Reads a specific frontend file.
    async fn read_frontend_file(path: impl AsRef<Path>) -> Vec<u8> {
        let path = Path::new(FRONTEND_FOLDER_PATH).join(path.as_ref());
        let mut buffer = Vec::new();

        debug!("Loading resource at: {}", path.display());

        OpenOptions::new()
            .read(true)
            .open(path)
            .await
            .unwrap()
            .read_to_end(&mut buffer)
            .await
            .unwrap();

        buffer
    }

    /// Converts the file path of a frontend file into a valid URI.
    fn parse_file_path(path: impl AsRef<Path>) -> String {
        let path = path.as_ref();

        if path.ends_with("index.html") || path.ends_with("home.html") {
            return String::from("/");
        }

        let endpoint = if !path.starts_with("/") {
            format!("/{}", path.display())
        } else {
            format!("{}", path.display())
        };

        if endpoint.ends_with(".html") {
            endpoint.replace(".html", "")
        } else {
            endpoint
        }
    }
}

// todo: add more MIME types
// todo: refactor into using file extensions and not file names
/// Returns a MIME type based on the file extension.
fn file_name_to_mime(file_name: &str) -> &'static str {
    match file_name {
        _ => "application/octet-stream",
    }
}
