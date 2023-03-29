mod config;
mod filter;
mod finder;

use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::finder::Finder;
use filter::Filter;
use mlua::prelude::*;

fn find_test_or_target(file: String, work_dir: String, config: Config) -> Vec<String> {
    let file = PathBuf::from(&file);

    match file.extension() {
        Some(extension) => {
            let extension = extension.to_str().unwrap();
            if !config.language_configs.contains_key(extension) {
                return vec![];
            }
        }
        None => return vec![],
    }

    let suggestions = Finder {
        file,
        work_dir: PathBuf::from(work_dir),
        config,
    }
    .find_test_or_target();

    Filter { paths: suggestions }
        .call()
        .iter()
        .map(|path| path.to_str().unwrap().to_string())
        .collect()
}

fn work_dir_config(work_dir: &String) -> Option<Config> {
    let path = PathBuf::from(work_dir).join(".gotospec");

    if path.exists() {
        match fs::read_to_string(path) {
            Ok(file) => return Some(serde_json::from_str::<Config>(file.as_str()).unwrap()),
            Err(_) => return None,
        }
    }

    None
}

fn lua_goto(
    lua: &Lua,
    (file, work_dir, conf): (String, String, LuaValue),
) -> LuaResult<Vec<String>> {
    let mut config: Config = lua.from_value(conf)?;

    match work_dir_config(&work_dir) {
        Some(work_dir_config) => {
            for (k, v) in work_dir_config.language_configs {
                config.language_configs.insert(k, v);
            }
        }
        None => {}
    };

    Ok(find_test_or_target(file, work_dir, config))
}

#[mlua::lua_module]
fn goto_backend(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("jump", lua.create_function(lua_goto)?)?;
    Ok(exports)
}
