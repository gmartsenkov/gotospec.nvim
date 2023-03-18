mod config;

use std::path::{Path, PathBuf};

use crate::config::Config;
use mlua::prelude::*;

fn relative_file_path(file: &String, work_dir: String) -> String {
    let mut path = Path::new(&file);
    path = path.strip_prefix(work_dir).unwrap().parent().unwrap();

    return path.to_str().unwrap().to_string();
}

fn find_spec(file: String, work_dir: String, config: &Config) -> Vec<String> {
    let file_path = Path::new(&file);
    let extension = file_path.extension().unwrap().to_str().unwrap();
    let test_folder = config.test_folders.get(extension).unwrap();
    let test_file_name = config.target_to_test_name(&file);

    let mut suggestions: Vec<PathBuf> = Vec::new();
    suggestions.push(
        PathBuf::from(work_dir.clone())
            .join(test_folder)
            .join(relative_file_path(&file, work_dir))
            .join(test_file_name),
    );
    suggestions
        .into_iter()
        .map(|p| p.to_str().unwrap().to_string())
        .collect()
}

fn find_test_or_target(file: String, work_dir: String, config: &Config) -> Vec<String> {
    if config.is_test(&file) {
        return vec!["is spec".to_string()];
    }
    find_spec(file, work_dir, &config)
}

fn lua_goto(_: &Lua, (file, work_dir): (String, String)) -> LuaResult<Vec<String>> {
    Ok(find_test_or_target(file, work_dir, &Config::default()))
}

#[mlua::lua_module]
fn goto(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("goto", lua.create_function(lua_goto)?)?;
    Ok(exports)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    struct Test {
        file: &'static str,
        expected: Vec<&'static str>,
    }

    #[test]
    fn test_find_spec() {
        let work_dir = "/dev/backend";
        let config = Config::default();
        let tests = [Test {
            file: "/dev/backend/lib/api/header.rb",
            expected: vec!["/dev/backend/spec/lib/api/header_spec.rb"],
        }];

        for test in tests {
            let results = find_test_or_target(test.file.to_string(), work_dir.to_string(), &config);
            assert_eq!(results, test.expected)
        }
    }
}
