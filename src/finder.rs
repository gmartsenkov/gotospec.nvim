use crate::config::Config;
use itertools::Itertools;
use std::path::{Path, PathBuf};

pub struct Finder {
    pub file: String,
    pub work_dir: String,
    pub config: Config,
}

impl Finder {
    fn find_test(&self) -> Vec<String> {
        let file_path = Path::new(&self.file);
        let extension = file_path.extension().unwrap().to_str().unwrap();
        let test_folder = self.config.test_folders.get(extension).unwrap();
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
        suggestions
            .into_iter()
            .unique()
            .map(|p| p.to_str().unwrap().to_string())
            .collect()
    }

    fn find_target(&self) -> Vec<String> {
        let file_path = Path::new(&self.file);
        let target_file_name = self.config.test_to_target_name(&self.file);
        let extension = file_path.extension().unwrap().to_str().unwrap();
        let test_folder = self.config.test_folders.get(extension).unwrap();
        let mut suggestions: Vec<PathBuf> = Vec::new();

        for dir in self.config.primary_source_dirs(&extension.to_string()) {
            suggestions.push(
                PathBuf::from(self.work_dir.clone())
                    .join(&dir)
                    .join(
                        self.relative_file_path()
                            .strip_prefix(format!("{}/", test_folder).as_str())
                            .unwrap(),
                    )
                    .join(&target_file_name),
            )
        }

        suggestions
            .into_iter()
            .unique()
            .map(|p| p.to_str().unwrap().to_string())
            .collect()
    }

    pub fn find_test_or_target(&self) -> Vec<String> {
        if self.config.is_test(&self.file) {
            self.find_target()
        } else {
            self.find_test()
        }
    }

    fn relative_file_path(&self) -> String {
        let mut path = Path::new(&self.file);
        path = path.strip_prefix(&self.work_dir).unwrap().parent().unwrap();

        return path.to_str().unwrap().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        file: &'static str,
        expected: Vec<&'static str>,
    }

    #[test]
    fn test_find_spec() {
        let work_dir = "/dev/backend";
        let config = Config::default();
        let tests = [
            Test {
                file: "/dev/backend/lib/api/header.rb",
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                file: "/dev/backend/api/header.rb",
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                file: "/dev/backend/app/api/header.rb",
                expected: vec![
                    "/dev/backend/spec/app/api/header_spec.rb",
                    "/dev/backend/spec/lib/api/header_spec.rb",
                ],
            },
            Test {
                file: "/dev/backend/spec/api/header_spec.rb",
                expected: vec![
                    "/dev/backend/app/api/header.rb",
                    "/dev/backend/lib/api/header.rb",
                ],
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let finder = Finder {
                file: test.file.to_string(),
                work_dir: work_dir.to_string(),
                config: config.clone(),
            };
            assert_eq!(
                finder.find_test_or_target(),
                test.expected,
                "Test number {} failed",
                i
            )
        }
    }
}
