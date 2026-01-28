return {

	-- Disables rustAnalyzer.check.allTargets for nvim-lspconfig.
	-- For VsCode/Zed/etc., you'll need to set `rustAnalyzer.check.allTargets = false` using their config format.
	{
		"neovim/nvim-lspconfig",
		config = function()
			local rust_analyzer_overrides = {
				settings = {
					["rust-analyzer"] = {
						check = {
							allTargets = false,
						},
					},
				},
			}
			-- If you named your rust_analyzer lsp config something else, you'll need to change this too
			pcall(vim.lsp.config, "rust_analyzer", rust_analyzer_overrides)
		end,
	},
}
