use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumClient {
    pub description: String,
    pub id: String,
    pub ip: String,
    pub mac: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum KontentumVariableCollection {
    Basic(Vec<String>),
    Dictionary(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumExhibitInfo {
    pub client_id: i32,
    pub clients: Vec<KontentumClient>,
    pub liveupdate: bool,
    pub name: String,
    pub ping: i32,
    pub select: Vec<String>,
    pub variables: KontentumVariableCollection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumFile {
    pub credit: String,
    pub description: String,
    pub file: Option<String>,
    pub filename: String,
    pub id: String,
    pub identifier: String,
    pub modified: String,
    pub title: Option<String>,

    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumText {
    pub id: i32,
    pub identifier: String,
    pub text: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumLanguage {
    pub identifier: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KontentumExhibit<T> {
    pub app_id: String,
    pub content: T,
    pub exhibit: KontentumExhibitInfo,
    pub exhibit_id: String,
    pub files: Vec<KontentumFile>,
    pub languages: Vec<KontentumLanguage>,
    pub last_modified: String,
    pub liveupdate: String,
    pub name: String,
    pub success: bool,
    pub texts: Vec<KontentumText>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum KontentumValue {
    #[serde(rename = "text")]
    Text { id: i32 },

    #[serde(rename = "value")]
    Value { value: String },

    #[serde(rename = "file")]
    File { id: i32 },

    #[serde(rename = "filelist")]
    FileList { id: Vec<i32> },

    #[serde(rename = "element")]
    Element { id: i32 },

    #[serde(rename = "elementlist")]
    ElementList { id: Vec<i32>, source: String },

    #[serde(rename = "client")]
    Client { id: i32 },
}

pub const KONTENTUM_URL: &str = "https://kontentum.link";
pub const KONTENTUM_FILEVAULT_URL: &str = "https://kontentum.link/filevault";

impl<T> KontentumExhibit<T> {
    pub fn get_text_by_id(&self, lang: &str, id: i32) -> Option<String> {
        self.texts
            .iter()
            .find(|text| text.id == id)
            .map(|text| text.text[lang].clone())
    }

    pub fn get_text_by_identifier(&self, lang: &str, identifier: &str) -> Option<String> {
        self.texts
            .iter()
            .find(|text| text.identifier == identifier)
            .map(|text| text.text[lang].clone())
    }

    pub fn get_text_by_value(&self, lang: &str, value: &KontentumValue) -> Option<String> {
        if let KontentumValue::Text { id } = value {
            self.get_text_by_id(lang, *id)
        } else {
            None
        }
    }

    pub fn get_file_by_id(&self, id: i32) -> Option<KontentumFile> {
        self.files
            .iter()
            .find(|file| file.id == id.to_string())
            .cloned()
    }

    pub fn get_file_by_value(&self, value: &KontentumValue) -> Option<KontentumFile> {
        if let KontentumValue::File { id } = value {
            self.get_file_by_id(*id)
        } else {
            None
        }
    }
}
