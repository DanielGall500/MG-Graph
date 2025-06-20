use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use std::path::{PathBuf};

#[derive(Serialize, Deserialize)]
pub struct MGExample {
    pub title: String,
    pub lang: String,
    pub grammar: Vec<String>
}

pub type MGCollection = Vec<MGExample>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub db_addr: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

pub struct DataManager;

impl DataManager {

    const SETTINGS_PATH: &str = "settings.json";
    const MG_COLLECTION_PATH: &str = "mg.json";

    pub fn get_data_path(filename: &str) -> PathBuf {

        // Get the directory of the running executable
        let exe_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src"); 

        // Construct the data path relative to exe_dir
        exe_dir.join("data").join("json").join(filename)
    }

    pub async fn ensure_file_exists(path: &PathBuf) -> std::io::Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Check if file exists, create if it doesn't
        if !path.exists() {
            let mut file = fs::File::create(path).await?;
            // Optionally write initial content, e.g. empty JSON object
            file.write_all(b"{}").await?;
        }

        Ok(())
    }

    pub async fn save_settings<T: Serialize>(settings: &T) -> std::io::Result<()> {
        Self::save_one_to_file(Self::SETTINGS_PATH, settings).await
    }

    pub async fn _edit_settings_key<K: Into<String>, V: Serialize>(
        key: K,
        value: V,
    ) -> std::io::Result<()> {
        // load current settings as JSON
        let mut settings: Value = match Self::load_from_file(Self::SETTINGS_PATH).await {
            Ok(val) => val,
            Err(_) => json!({}), // Start fresh if file missing or empty
        };

        // ensure it's an object
        if !settings.is_object() {
            settings = json!({});
        }

        // insert or update key
        settings[key.into()] = serde_json::to_value(value)?; 

        // save updated JSON
        Self::save_one_to_file(Self::SETTINGS_PATH, &settings).await
    }

    pub async fn save_mg_collection(mgs: &MGCollection) -> std::io::Result<()> {
        Self::save_many_to_file(Self::MG_COLLECTION_PATH, mgs).await
    }

    pub async fn save_one_to_file<T: Serialize>(path: &str, obj: &T) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(obj)?;
        Self::save_to_file(path, json).await 
    }

    pub async fn save_many_to_file<T: Serialize>(path: &str, objs: &[T]) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(objs)?;
        Self::save_to_file(path, json).await
    }

    async fn save_to_file(filename: &str, json: String) -> std::io::Result<()> {
        let path: PathBuf = Self::get_data_path(filename);

        Self::ensure_file_exists(&path).await?;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .await?;

        file.write_all(json.as_bytes()).await?;
        file.write_all(b"\n").await?;

        Ok(())
    }

    pub async fn load_settings<T: DeserializeOwned>() -> std::io::Result<T> {
        Self::load_from_file(Self::SETTINGS_PATH).await
    }

    pub async fn load_mg_collection<MGCollection: DeserializeOwned>() -> std::io::Result<MGCollection> {
        Self::load_from_file(Self::MG_COLLECTION_PATH).await
    }

    pub async fn load_from_file<T: DeserializeOwned>(filename: &str) -> std::io::Result<T> {
        let path: PathBuf = Self::get_data_path(filename);
        let contents = fs::read_to_string(path).await?;
        let data = serde_json::from_str::<T>(&contents)?;
        Ok(data)
    }

}