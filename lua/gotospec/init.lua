local M = {}

local options = {}
local defaults = {
  language_configs = {
    ["rb"] = {
      primary_source_dirs = { "app", "lib" },
      test_file_suffix = "_spec",
      test_file_matcher = "_spec.rb",
      test_folder = "spec",
      omit_source_dir_from_test_dir = true
    },
    ["ex"] = {
      primary_source_dirs = {"lib"},
      test_file_suffix = "_test",
      test_file_matcher = "_test.exs",
      test_folder = "test",
      omit_source_dir_from_test_dir = true,
      test_file_extension = "exs",
      source_file_extension = "ex"
    }
  }
}

function M.jump()
  local buffer_name = vim.api.nvim_buf_get_name(0)
  local work_dir = vim.fn.getcwd()

  if vim.fn.filereadable(buffer_name) ~= 1 then
    print("Current file does not exist")
    return
  end

  local suggestions = require("goto_backend").jump(buffer_name, work_dir, defaults)
  if #suggestions == 1 then
     vim.cmd("e " .. suggestions[1])
      return
  end

  vim.ui.input({
    prompt = ("Possible options: \n1: " .. table.concat(suggestions, "\n2: ") .. "\nEnter number: ")},
    function(input)
      local selection = tonumber(input)
      if selection then
        vim.cmd("e " .. suggestions[tonumber(input)])
      else
        print("Invalid selection")
      end
    end)
end

return M;
