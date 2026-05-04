use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigModel {
    pub dirsearch_text: String,
}
