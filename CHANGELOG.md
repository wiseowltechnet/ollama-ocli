# Changelog

## [0.2.0] - 2026-02-03

### Added
- **MCP (Model Context Protocol) Support**
  - Load external MCP servers for extended functionality
  - `/mcp list` - Show available MCP tools
  - `/mcp call <tool>` - Invoke MCP tools directly
  - AI automatically knows about available MCP tools
  - Configure servers in `.ocli/mcp_servers.json`

- **Configuration Management**
  - `/config set <key> <value>` - Set configuration
  - `/config get <key>` - Get configuration value
  - `/config list` - Show all settings
  - Persistent config in `.ocli/config.json`

- **Conversation Export**
  - `/export [filename]` - Export conversation to markdown
  - Auto-generates timestamped filenames

- **Enhanced UI**
  - LCARS-styled `/help` command with color-coded categories
  - Startup banner showing MCP tool count
  - Improved command organization

- **Distribution**
  - Comprehensive README.md with examples
  - Homebrew formula for easy installation
  - Version bumped to 0.2.0

### Changed
- System prompts now include MCP tool information
- Help command redesigned with LCARS styling
- Startup shows available MCP tools count

## [0.1.0] - 2026-02-02

### Initial Release
- AI-powered development with autonomous tool calling
- Built-in tools: read_file, write_file, execute_bash, search_files, list_directory
- Planning mode with `/plan`, `/next`, `/show-plan`
- WiseOwl project management system
- LCARS Star Trek interface styling
- Terminal UI with `/monitor` command
- Git integration
- Statistics tracking
- Self-improvement capability
