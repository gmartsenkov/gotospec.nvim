local M = {}

local loaded = false
local options = {}
local defaults = {
  language_configs = {
    ["rb"] = {
      primary_source_dirs = { "app", "lib" },
      test_file_suffix = "_spec",
      test_file_matcher = "_spec.rb",
      test_folder = "spec",
      omit_source_dir_from_test_dir = false
    }
  }
}

function M.test()
  return "hello"
end

function M.jump()
  if loaded ~= false then
    print("goto backend was not compiled")
    return
  end

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

function M.setup()
  local _, error = loadfile("../goto_backend.so")
  if error then
    loaded = true
  end
end

return M;
