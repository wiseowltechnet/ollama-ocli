# OCLI v0.3.0 Demo Script

## Recording Setup
- Terminal: 120x30
- Font: Monaco 14pt
- Theme: Dark background
- Tool: asciinema or terminalizer

## Demo Flow (60 seconds)

### Scene 1: Startup (5s)
```bash
ocli
```
**Show**: LCARS banner, model info, MCP status

### Scene 2: Help Command (5s)
```bash
/help
```
**Show**: Organized command list with LCARS styling

### Scene 3: Command History (5s)
```bash
# Type /h then press UP arrow
# Show history navigation
# Press TAB to complete
```
**Show**: Auto-completion working

### Scene 4: File Operations (10s)
```bash
/read src/main.rs
```
**Show**: Syntax highlighted code

### Scene 5: Planning (10s)
```bash
/plan Add error handling to config module
```
**Show**: AI generating step-by-step plan

### Scene 6: Stats & Performance (5s)
```bash
/stats
/perf
```
**Show**: Session statistics and performance metrics

### Scene 7: Multi-Model (5s)
```bash
/models
```
**Show**: Available models list

### Scene 8: Context Management (5s)
```bash
/context
```
**Show**: Smart context with token counts

### Scene 9: Monitor (10s)
```bash
/monitor
```
**Show**: Real-time statistics dashboard

### Scene 10: Exit (5s)
```bash
exit
```
**Show**: Goodbye message

## Key Points to Highlight
1. Beautiful LCARS interface
2. Fast command execution
3. Syntax highlighting
4. Smart suggestions
5. Clean, organized output

## GIF Creation Commands

### Using asciinema
```bash
# Record
asciinema rec demo.cast

# Convert to GIF
agg demo.cast demo.gif
```

### Using terminalizer
```bash
# Record
terminalizer record demo

# Render
terminalizer render demo
```

### Using ttygif
```bash
# Record
ttyrec demo.tty

# Convert
ttygif demo.tty
```

## Screenshot Locations
1. Startup screen - LCARS banner
2. Help command - Command list
3. Syntax highlighting - Code display
4. Monitor dashboard - Real-time stats
5. Planning mode - AI-generated plan

## Social Media Captions

**Short (Twitter):**
"OCLI v0.3.0 - Enterprise AI coding assistant with Star Trek LCARS styling. Command history, syntax highlighting, smart context. Built with Rust. cargo install ocli"

**Medium (Reddit):**
"Demo of OCLI v0.3.0 showing command history, syntax highlighting, planning mode, and real-time monitoring. Enterprise features for local AI coding."

**Long (Dev.to):**
"Watch OCLI v0.3.0 in action: LCARS-themed interface, command completion, syntax highlighting, AI planning, and performance monitoring. Built with Rust for speed and reliability."
