# gotospec.nvim

Easily switch between implementation and test files.
What I was missing from similar plugins was the ability to automatically create the test/source file if one did not exist.

## Installation ##
Most of the source code is written in Rust so in order to build the plugin Rust needs to be installed on the system. 

Using [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
  "gmartsenkov/gotospec.nvim",
  lazy = false,
  build = "make",
  dependencies = { 'jghauser/mkdir.nvim' }
  config = function()
    require("gotospec").setup({})
  end
}
```

Assign to a keybinding

```lua
["<leader>tt"] = {
  function()
    require("gotospec").jump()
    end,
  "switch between test/implementation"
},
```
## Configuration ##
Custom configuration can set on a per project basis by creating a `.gotospec` file in the root of the project.

Example:
```json
{
  "language_configs": {
    "rb": {
      "primary_source_dirs": ["apps", "lib"],
      "test_file_suffix": "_spec",
      "test_file_matcher": "_spec.rb",
      "test_folder":  "spec",
      "omit_source_dir_from_test_dir": true
    }
  }
}
```

## Custom cwd (current working directory)
The plugin uses `vim.fn.getcwd()` by default to determine the project root, however that won't always be accurate.  
To fix this I've created [root.nvim](https://github.com/gmartsenkov/root.nvim) which providers a helper function that returns the project root.  
The `jump` method will take an optional `cwd` path, if that returns nil it'll default to `vim.fn.getcwd()`  
Example usage -  
```lua
["<leader>tt"] = {
  function()
    require("gotospec").jump(
      require("root").find()
    )
  end
}
```
