use super::*;
use std::path::{Path, PathBuf};
use hyper::header::HeaderValue;
use tokio::fs::OpenOptions;
use tokio::io::AsyncReadExt;

const FRONTEND_FOLDER_PATH: &str = "frontend";
const RESOURCE_SETTINGS_FILE: &str = "resources.json";

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceSettings {
    pub pages: Vec<PathBuf>,
    pub scripts: Vec<PathBuf>,
    pub other: Vec<PathBuf>,
}

impl ResourceSettings {
    // todo: error handling
    pub async fn from_file() -> Self {
        let path = Path::new(RESOURCE_SETTINGS_FILE);
        let file_contents = Self::read_frontend_file(path).await;
        serde_json::from_slice::<Self>(&file_contents).unwrap()
    }

    // todo: error handling
    pub async fn into_resource_map(self) -> ResourceMap {
        let mut map = ResourceMap::new();

        for page_path in self.pages {
            Self::load_and_insert_resource(&mut map, page_path, "text/html").await;
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

        map
    }

    // todo: error handling
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

    fn parse_file_path(path: impl AsRef<Path>) -> String {
        let path = path.as_ref();

        if path.ends_with("index.html") || path.ends_with("home.html") {
            return String::from("/");
        }

        if !path.starts_with("/") {
            format!("/{}", path.display())
        } else {
            format!("{}", path.display())
        }
    }
}

fn file_name_to_mime(file_name: &str) -> &'static str {
    match file_name {
        _ => "application/octet-stream",
    }
}
