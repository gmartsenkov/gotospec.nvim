use std::{collections::HashMap, path::Path};

use regex::Regex;

#[derive(Debug)]
pub struct Config {
    rspec_primary_source_dirs: Vec<String>,
    test_file_mappings: HashMap<String, Regex>,
    test_file_suffixes: HashMap<String, String>,
    pub test_folders: HashMap<String, String>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            rspec_primary_source_dirs: vec!["app".to_string(), "lib".to_string()],
            test_file_suffixes: HashMap::from([("rb".to_string(), "_spec.rb".to_string())]),
            test_file_mappings: HashMap::from([(
                "rb".to_string(),
                Regex::new(r"_spec.rb").unwrap(),
            )]),
            test_folders: HashMap::from([("rb".to_string(), "spec".to_string())]),
        }
    }

    pub fn target_to_test_name(&self, file: &String) -> String {
        let path = Path::new(&file);
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        let suffix = self.test_file_mappings.get(extension).unwrap();
        format!("{}{}", file_name, suffix)
    }

    pub fn is_test(&self, file: &String) -> bool {
        let path = Path::new(&file);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        let test_regex = self.test_file_mappings.get(extension).unwrap();
        return test_regex.is_match(file_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_test() {
        let config = Config::default();
        let test_files = ["api_spec.rb", "db_spec.rb"];

        for file in test_files {
            assert!(config.is_test(&file.to_string()));
        }

        let target_files = ["api.rb", "db.rb"];

        for file in target_files {
            assert_eq!(config.is_test(&file.to_string()), false);
        }
    }
}
