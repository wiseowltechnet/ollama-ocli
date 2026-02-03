## [0.3.2] - 2026-02-03

### 🎨 LCARS Dashboard & Live Progress

#### New Features
- **Live Progress Indicator** - Real-time stats while AI generates responses
  - Token count updates every 10 tokens
  - Tokens per second calculation
  - Elapsed time tracking
  - ANSI escape codes for in-place updates
  
- **`/dashboard` Command** - Full-screen LCARS TUI dashboard
  - Response time graphs (Unicode bar charts)
  - Memory usage gauge
  - Activity log (last 10 events with timestamps)
  - MCP tools status
  - Session statistics (turns, tokens, uptime)
  - Press Esc or F3 to exit
  
- **Real-time Stats Tracking**
  - Automatic tracking of all conversation metrics
  - Response times (last 10)
  - Memory usage monitoring
  - Activity logging
  - Turn and token counting

#### Technical
- Added `ratatui 0.26` for TUI framework
- Added `sysinfo 0.30` for system metrics
- LCARS orange (255,153,0) and blue (153,204,255) color scheme
- Minimal performance overhead (~0.1ms per update)

#### Impact
- Real-time visibility into AI performance
- Better understanding of system resource usage
- Enhanced user experience with live feedback

## [0.3.1] - 2026-02-03

### ⚡ Performance & Developer Experience

#### New Commands
- **`/write-direct <file> <content>`** - Write files instantly without LLM (100x faster)
- **`/append <file> <content>`** - Append to files without LLM call
- **`/build [go|rust|npm|auto]`** - Compile projects in OCLI with auto-detection
- **`/template <type> <path>`** - Generate code from templates (go-mcp-server, rust-cli, python-script)
- **`/model <name>`** - Switch between models (saves to config)

#### Impact
- 100x speed improvement for file operations
- Create MCP server in <5 seconds (vs timeout before)
- Build projects without exiting OCLI
- Instant code scaffolding from templates

## [0.3.0] - 2026-02-03

### 🎉 Major Release - Enterprise Features

#### Quick Wins
- **Command History** - Navigate with ↑/↓ arrows
- **Auto-completion** - Tab completion for all commands
- **Syntax Highlighting** - Colored code blocks
- **Progress Bars** - Visual feedback for operations
- **Keyboard Shortcuts** - Ctrl+C, Ctrl+D, Tab

#### Major Features
- **Smart Context Management** - Auto-summarize, search, prune
- **AI Suggestions** - Typo correction, workflow recommendations
- **Diff Viewer** - Side-by-side diffs with LCARS styling
- **Multi-Model Support** - Switch between Ollama models

#### Performance & Architecture
- **Performance Metrics** - `/perf` command for real-time stats
- **Design Patterns** - Command, Strategy, Builder patterns documented
- **Metrics Module** - Track command execution times

#### GitHub Actions
- **Matrix Builds** - Ubuntu + macOS
- **Dependency Caching** - 80% faster builds
- **Security Audits** - Automated vulnerability scanning
- **Auto Updates** - Weekly dependency updates
- **Performance Tracking** - Binary size monitoring

### Dependencies Added
- `rustyline` 13.0 - Command history & completion
- `syntect` 5.0 - Syntax highlighting
- `indicatif` 0.17 - Progress bars

### New Modules
- `readline.rs` - Command history & completion
- `syntax.rs` - Syntax highlighting
- `progress.rs` - Progress indicators
- `context_manager.rs` - Smart context management
- `suggestions.rs` - AI suggestions
- `diff_viewer.rs` - Diff viewing
- `models.rs` - Multi-model support
- `metrics.rs` - Performance tracking

### Documentation
- `DESIGN_PATTERNS.md` - Architecture patterns
- `PERFORMANCE.md` - Performance guide
- `FEATURES_V3.md` - Feature tracking
- `.github/WORKFLOWS.md` - CI/CD documentation

### Stats
- 16 source modules
- ~2,800 lines of Rust
- 9/9 features implemented
- 5 GitHub Actions workflows

---

**This is a major release with enterprise-grade features!** 🚀
## [0.2.1] - 2026-02-03

### Added
- `/history` command - View past conversation sessions
- `/alias` command - Create custom command shortcuts
  - `/alias set <name> <command>` - Create alias
  - `/alias list` - Show all aliases
  - `/alias remove <name>` - Remove alias
- Homebrew tap for easy installation

### Changed
- Updated help command to include new features
- Improved command organization

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
