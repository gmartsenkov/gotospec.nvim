mod config;
mod filter;
mod finder;

use std::path::PathBuf;

use crate::config::Config;
use crate::finder::Finder;
use filter::Filter;
use mlua::prelude::*;

fn find_test_or_target(file: String, work_dir: String, config: Config) -> Vec<String> {
    if config.is_test(&PathBuf::from(&file)) {
        return vec!["is spec".to_string()];
    }
    let suggestions = Finder {
        file: PathBuf::from(&file),
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

fn lua_goto(_: &Lua, (file, work_dir): (String, String)) -> LuaResult<Vec<String>> {
    Ok(find_test_or_target(file, work_dir, Config::default()))
}

#[mlua::lua_module]
fn goto(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("goto", lua.create_function(lua_goto)?)?;
    Ok(exports)
}
