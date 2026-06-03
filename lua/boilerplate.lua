local M = {}

local ns = vim.api.nvim_create_namespace('boilerplate')

local bg = '#2d2a3d'
local ctermbg = 237

local regions = {
  'boilerplateCode',
  'boilerplateCodeLine',
  'boilerplateInterpolation',
  'boilerplateInterpolationLine',
}

local function style_window(win)
  for _, group in ipairs(vim.fn.getcompletion('rust', 'highlight')) do
    local hl = vim.api.nvim_get_hl(0, { name = group, link = false })
    hl.italic, hl.bg, hl.ctermbg = true, bg, ctermbg
    hl.cterm = hl.cterm or {}
    hl.cterm.italic = true
    vim.api.nvim_set_hl(ns, group, hl)
  end

  local delimiter = vim.api.nvim_get_hl(0, { name = 'boilerplateDelimiter', link = false })
  if vim.tbl_isempty(delimiter) then
    delimiter = vim.api.nvim_get_hl(0, { name = 'PreProc', link = false })
  end
  delimiter.bg, delimiter.ctermbg = bg, ctermbg
  vim.api.nvim_set_hl(ns, 'boilerplateDelimiter', delimiter)

  vim.api.nvim_win_set_hl_ns(win, ns)

  for _, group in ipairs(regions) do
    vim.api.nvim_set_hl(0, group, { italic = true, bg = bg, ctermbg = ctermbg })
  end
end

function M.style()
  style_window(0)
end

function M.unstyle()
  vim.api.nvim_win_set_hl_ns(0, 0)
end

function M.restyle()
  for _, win in ipairs(vim.api.nvim_list_wins()) do
    if vim.bo[vim.api.nvim_win_get_buf(win)].syntax == 'boilerplate' then
      style_window(win)
    end
  end
end

return M
