//! A service used to filter out the best suggestions
//! It consists of 3 steps
//! 1 - Return the files that exist
//! 2 - If none of the files exist return the ones where the parent folder exists
//! 3 - If none of the above are true just return the original suggestions

#[allow(unused_imports)]
use itertools::Itertools;
use std::path::PathBuf;

pub struct Filter {
    pub paths: Vec<PathBuf>,
}

impl Filter {
    pub fn call(&self) -> Vec<PathBuf> {
        let existing_files = self.existing_files();
        if !existing_files.is_empty() {
            return existing_files;
        }
        let existing_dirs = self.existing_dirs();
        if !existing_dirs.is_empty() {
            return existing_dirs;
        }

        self.paths.clone()
    }

    fn existing_files(&self) -> Vec<PathBuf> {
        for path in &self.paths {
            println!("{:?}", std::fs::canonicalize(path));
        }
        self.paths
            .iter()
            .filter(|path| path.exists())
            .map(|p| p.clone())
            .collect()
    }

    fn existing_dirs(&self) -> Vec<PathBuf> {
        self.paths
            .iter()
            .filter(|path| path.parent().unwrap().exists())
            .map(|p| p.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test<'a> {
        input: Vec<&'a str>,
        result: Vec<&'a str>,
    }

    #[test]
    fn test_call() {
        let tests = [
            Test {
                input: vec![
                    "./tests/fixtures/api/header.rb",
                    "./tests/fixtures/db/config.rb",
                    "./tests/fixtures/api/missing.rb",
                ],
                result: vec![
                    "./tests/fixtures/api/header.rb",
                    "./tests/fixtures/db/config.rb",
                ],
            },
            Test {
                input: vec![
                    "./tests/fixtures/api/missing.rb",
                    "./tests/fixtures/db/missing.rb",
                    "./tests/fixtures/missing/file.rb",
                ],
                result: vec![
                    "./tests/fixtures/api/missing.rb",
                    "./tests/fixtures/db/missing.rb",
                ],
            },
            Test {
                input: vec![
                    "./tests/fixtures/api/missing.rb",
                    "./tests/fixtures/db/missing.rb",
                ],
                result: vec![
                    "./tests/fixtures/api/missing.rb",
                    "./tests/fixtures/db/missing.rb",
                ],
            },
        ];

        for test in tests {
            let filter = Filter {
                paths: test.input.iter().map(|p| PathBuf::from(p)).collect(),
            };
            assert_eq!(
                filter
                    .call()
                    .iter()
                    .map(|p| p.to_str().unwrap())
                    .collect_vec(),
                test.result
            );
        }
    }
}
