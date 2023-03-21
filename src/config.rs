use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub primary_source_dirs: Vec<String>,
    pub test_file_matcher: String,
    pub test_file_suffix: String,
    pub test_folder: String,
    pub omit_source_dir_from_test_dir: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language_configs: HashMap<String, LanguageConfig>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            language_configs: HashMap::from([(
                "rb".to_string(),
                LanguageConfig {
                    primary_source_dirs: vec!["app".to_string(), "lib".to_string()],
                    test_file_suffix: "_spec".to_string(),
                    test_file_matcher: "_spec.rb".to_string(),
                    test_folder: "spec".to_string(),
                    omit_source_dir_from_test_dir: false,
                },
            )]),
        }
    }
}
impl Config {
    pub fn primary_source_dirs(&self, extension: &String) -> Vec<String> {
        let dirs = &self
            .language_configs
            .get(extension)
            .unwrap()
            .primary_source_dirs;

        if dirs.len() == 0 {
            vec!["".to_string()]
        } else {
            dirs.clone()
        }
    }

    pub fn strip_primary_source_dirs_from_path(
        &self,
        path: &PathBuf,
        extension: &String,
    ) -> PathBuf {
        let mut path = Path::new(path);
        let dirs = &self
            .language_configs
            .get(extension)
            .unwrap()
            .primary_source_dirs;

        for dir in dirs {
            path = path.strip_prefix(dir).unwrap_or_else(|_| path);
        }

        path.to_path_buf()
    }

    pub fn test_to_target_name(&self, file: &PathBuf) -> String {
        let file_name = file.file_stem().unwrap().to_str().unwrap();
        let extension = file.extension().unwrap().to_str().unwrap();
        let suffix = &self
            .language_configs
            .get(extension)
            .unwrap()
            .test_file_suffix;

        format!("{}.{}", file_name.strip_suffix(suffix).unwrap(), extension)
    }

    pub fn target_to_test_name(&self, file: &PathBuf) -> String {
        let file_name = file.file_stem().unwrap().to_str().unwrap();
        let extension = file.extension().unwrap().to_str().unwrap();
        let suffix = &self
            .language_configs
            .get(extension)
            .unwrap()
            .test_file_suffix;
        format!("{}{}.{}", file_name, suffix, extension)
    }

    pub fn is_test(&self, file: &PathBuf) -> bool {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let extension = file.extension().unwrap().to_str().unwrap();
        let test_regex = &self
            .language_configs
            .get(extension)
            .unwrap()
            .test_file_matcher;

        return Regex::new(&test_regex).unwrap().is_match(file_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_test() {
        let config = Config::default();
        let test_files = [PathBuf::from("api_spec.rb"), PathBuf::from("db_spec.rb")];

        for file in test_files {
            assert!(config.is_test(&file));
        }

        let target_files = [PathBuf::from("api.rb"), PathBuf::from("db.rb")];

        for file in target_files {
            assert_eq!(config.is_test(&file), false);
        }
    }

    #[test]
    fn test_strip_primary_source_dirs_from_path() {
        let config = Config {
            language_configs: HashMap::from([(
                "rb".to_string(),
                LanguageConfig {
                    primary_source_dirs: vec!["lib".to_string()],
                    ..Default::default()
                },
            )]),
        };

        let result = config.strip_primary_source_dirs_from_path(
            &PathBuf::from("lib/bob/header_spec.rb"),
            &"rb".to_string(),
        );
        assert_eq!(result, PathBuf::from("bob/header_spec.rb"));
    }
}
