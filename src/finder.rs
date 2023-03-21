use crate::config::Config;
use itertools::Itertools;
use std::path::PathBuf;

pub struct Finder {
    pub file: PathBuf,
    pub work_dir: PathBuf,
    pub config: Config,
}

impl Finder {
    fn find_test(&self) -> Vec<PathBuf> {
        let extension = self.file.extension().unwrap().to_str().unwrap();
        let test_folder = &self.config.language_configs.get(extension).unwrap().test_folder;
        let test_file_name = self.config.target_to_test_name(&self.file);
        let relative_path = self.relative_file_path();
        let mut suggestions: Vec<PathBuf> = Vec::new();

        for dir in self.config.primary_source_dirs(&extension.to_string()) {
            suggestions.push(
                PathBuf::from(self.work_dir.clone())
                    .join(&test_folder)
                    .join(&dir)
                    .join(self.config.strip_primary_source_dirs_from_path(
                        &relative_path,
                        &extension.to_string(),
                    ))
                    .join(&test_file_name),
            );
        }

        suggestions.into_iter().unique().collect()
    }

    fn find_target(&self) -> Vec<PathBuf> {
        let target_file_name = self.config.test_to_target_name(&self.file);
        let extension = self.file.extension().unwrap().to_str().unwrap();
        let test_folder = PathBuf::from(&self.config.language_configs.get(extension).unwrap().test_folder);
        let mut suggestions: Vec<PathBuf> = Vec::new();

        for dir in self.config.primary_source_dirs(&extension.to_string()) {
            let relative_without_test_folder = PathBuf::from(
                self.relative_file_path()
                    .strip_prefix(&test_folder)
                    .unwrap(),
            );

            suggestions.push(
                PathBuf::from(self.work_dir.clone())
                    .join(&dir)
                    .join(
                        &relative_without_test_folder
                            .strip_prefix(dir)
                            .unwrap_or_else(|_| &relative_without_test_folder),
                    )
                    .join(&target_file_name),
            )
        }

        suggestions.into_iter().unique().collect()
    }

    pub fn find_test_or_target(&self) -> Vec<PathBuf> {
        if self.config.is_test(&self.file) {
            self.find_target()
        } else {
            self.find_test()
        }
    }

    fn relative_file_path(&self) -> PathBuf {
        self.file
            .strip_prefix(&self.work_dir)
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::config::LanguageConfig;

    use super::*;

    struct Test {
        config: Config,
        file: PathBuf,
        expected: Vec<&'static str>,
    }

    #[test]
    fn test_find_spec() {
        let work_dir = PathBuf::from("/dev/backend");
        let ruby_lib_config = LanguageConfig {
            primary_source_dirs: vec!["lib".to_string()],
            test_file_suffix: "_spec".to_string(),
            test_file_mappings: "_spec.rb".to_string(),
            test_folder: "spec".to_string()
        };
        let ruby_empty_source_config = LanguageConfig {
            primary_source_dirs: vec![],
            test_file_suffix: "_spec".to_string(),
            test_file_mappings: "_spec.rb".to_string(),
            test_folder: "spec".to_string()
        };
        let tests = [
            Test {
                config: Config::default(),
                file: PathBuf::from("/dev/backend/lib/api/header.rb"),
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                config: Config::default(),
                file: PathBuf::from("/dev/backend/api/header.rb"),
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                config: Config::default(),
                file: PathBuf::from("/dev/backend/app/api/header.rb"),
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                config: Config::default(),
                file: PathBuf::from("/dev/backend/spec/api/header_spec.rb"),
                expected: vec![
                    "/dev/backend/app/api/header.rb",
                    "/dev/backend/lib/api/header.rb",
                ],
            },
            Test {
                config: Config{language_configs: HashMap::from([("rb".to_string(), ruby_lib_config.clone())])},
                file: PathBuf::from("/dev/backend/lib/header.rb"),
                expected: vec!["/dev/backend/spec/lib/header_spec.rb"],
            },
            Test {
                config: Config{language_configs: HashMap::from([("rb".to_string(), ruby_lib_config.clone())])},
                file: PathBuf::from("/dev/backend/spec/lib/header_spec.rb"),
                expected: vec!["/dev/backend/lib/header.rb"],
            },
            Test {
                config: Config{language_configs: HashMap::from([("rb".to_string(), ruby_empty_source_config.clone())])},
                file: PathBuf::from("/dev/backend/header.rb"),
                expected: vec!["/dev/backend/spec/header_spec.rb"],
            },
            Test {
                config: Config{language_configs: HashMap::from([("rb".to_string(), ruby_empty_source_config.clone())])},
                file: PathBuf::from("/dev/backend/spec/header_spec.rb"),
                expected: vec!["/dev/backend/header.rb"],
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let finder = Finder {
                file: test.file.clone(),
                work_dir: work_dir.clone(),
                config: test.config.clone(),
            };
            assert_eq!(
                finder
                    .find_test_or_target()
                    .iter()
                    .map(|x| x.to_str().unwrap())
                    .collect_vec(),
                test.expected,
                "Test number {} failed",
                i
            )
        }
    }
}
