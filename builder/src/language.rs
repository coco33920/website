use std::{fs, path::Path};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Language {
    pub key: String,
    pub template: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub supported_languages: Vec<Language>,
}

impl Default for Language {
    fn default() -> Self {
        Self {
            key: "en".to_string(),
            template: "[A-Za-z]+\\.md".to_string(),
        }
    }
}

impl Language {
    fn is_language_file(self, file: &Path) -> bool {
        let regex = Regex::new(&self.template).expect("Error while creating regex");
        return regex.is_match(
            file.file_name()
                .expect("Error while reading path")
                .to_str()
                .expect("Error while converting to string"),
        );
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            supported_languages: vec![Language::default()],
        }
    }
}

pub fn load_config() -> LanguageConfig {
    let content: String = fs::read_to_string("languages.toml").expect("Language file absent");
    println!("{content}");
    toml::from_str(&content).unwrap_or_default()
}
