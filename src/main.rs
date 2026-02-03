mod tui;
mod lcars;
mod git;
mod stats;
mod wiseowl;
mod planning;
mod prompts;
mod tools;
mod context;
mod streaming;
mod multi_file;

use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};
use std::path::Path;
use futures_util::StreamExt;
use context::{ConversationContext, FileChange};
use planning::Plan;
use multi_file::{MultiFileEditor, FileEdit, EditOperation};
use streaming::stream_with_tools;
use prompts::get_system_prompt;

#[derive(Parser)]
#[command(name = "ocli")]
#[command(about = "Ollama CLI - A Claude Code-like interface")]
struct Args {
    #[arg(short, long, default_value = "deepseek-coder:6.7b")]
    model: String,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Chat {
        #[arg(long)]
        session: Option<String>,
    },
    Init,
    Plan {
        goal: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    
    match args.command {
        Some(Commands::Plan { goal }) => {
            plan_mode(&client, &args.model, &goal).await?;
        }
        Some(Commands::Init) => {
            init_project_mode(&client, &args.model).await?;
        }
        Some(Commands::Chat { session }) => {
            chat_mode(&client, &args.model, session.as_deref()).await?;
        }
        None => {
            chat_mode(&client, &args.model, None).await?;
        }
    }
    
    Ok(())
}

async fn chat_mode(client: &Client, model: &str, session: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let owl = crate::wiseowl::WiseOwl::init().await?;
    let session_name = session.unwrap_or("default");
    let mut context = ConversationContext::load(session_name).await?;
    let mut editor = MultiFileEditor::new();
    
    println!("{}", crate::lcars::header());
    println!("{}", crate::lcars::status_bar(&format!("Model: {}", model), &format!("Session: {}", session_name)));
    println!("Type 'exit' or Ctrl+C to end");
    println!("💡 Use /help for commands\n");
    
    if !context.messages.is_empty() {
        println!("📜 Loaded {} messages", context.messages.len());
    }
    
    loop {
        print!("You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {},
            Err(_) => break,
        }
        
        let input = input.trim();
        
        if input.is_empty() { continue; }
        if matches!(input.to_lowercase().as_str(), "exit" | "quit" | "q") {
            context.save(session_name).await?;
            println!("💾 Session saved. Goodbye!");
            break;
        }
        
        if input.starts_with('/') {
            if !handle_slash_command(&owl, client, model, input, &mut context, &mut editor).await? {
                break;
            }
            continue;
        }
        
        context.add_message("user".to_string(), input.to_string());
        
        let system_prompt = format!("{}\n{}", get_system_prompt(), context.get_context_summary());
        let full_prompt = format!("{}\n\nUser: {}", system_prompt, input);
        
        let response = stream_with_tools(client, model, &full_prompt).await?;
        context.add_message("assistant".to_string(), response);
        
        if context.messages.len() % 5 == 0 {
            context.save(session_name).await?;
        }
    }
    
    Ok(())
}

async fn handle_slash_command(owl: &crate::wiseowl::WiseOwl, 
    client: &Client,
    model: &str,
    input: &str,
    context: &mut ConversationContext,
    editor: &mut MultiFileEditor,
) -> Result<bool, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = input[1..].split_whitespace().collect();
    if parts.is_empty() { return Ok(true); }
    
    match parts[0] {
        "help" => {
            println!("📋 Commands:");
            println!("/plan <goal> - Create execution plan");
            println!("/todo <task> - Add to TODO list");
            println!("/done <task> - Mark task complete");
            println!("/rule <rule> - Add project rule");
            println!("/context - Show wiseowl context");
            println!("/version - Show version");
            println!("/next - Execute next plan step");
            println!("/show-plan - Display current plan");
            println!("📋 Commands:");
            println!("/help - Show this");
            println!("/read <file> - Read file into context");
            println!("/write <file> - Write file with AI");
            println!("/preview - Show pending changes");
            println!("/apply - Apply pending changes");
            println!("/rollback - Undo last change");
            println!("/clear - Clear context");
            println!("/exit - Exit");
        }
        
        "read" => {
            if parts.len() < 2 {
                println!("❌ Usage: /read <file>");
                return Ok(true);
            }
            let path = parts[1];
            match tokio::fs::read_to_string(path).await {
                Ok(content) => {
                    context.add_file(path.to_string(), content.clone());
                    println!("✅ Loaded {} ({} bytes)", path, content.len());
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        
        "write" => {
            if parts.len() < 2 {
                println!("❌ Usage: /write <file>");
                return Ok(true);
            }
            let path = parts[1];
            println!("What should I write to {}?", path);
            print!("You: ");
            io::stdout().flush()?;
            
            let mut request = String::new();
            io::stdin().read_line(&mut request)?;
            
            let prompt = format!("{}\n\nWrite content for file '{}' based on: {}\n\nProvide ONLY the file content.", 
                get_system_prompt(), path, request.trim());
            
            let content = get_complete_response(client, model, &prompt).await?;
            
            editor.add_edit(FileEdit {
                path: path.to_string(),
                content,
                operation: if Path::new(path).exists() { EditOperation::Modify } else { EditOperation::Create },
            });
            
            println!("✅ Added to pending changes. Use /preview to review, /apply to save.");
        }
        
        "preview" => {
            if editor.has_pending() {
                println!("{}", editor.show_preview());
            } else {
                println!("📭 No pending changes");
            }
        }
        
        "apply" => {
            if !editor.has_pending() {
                println!("📭 No pending changes");
                return Ok(true);
            }
            
            println!("{}", editor.show_preview());
            print!("\nApply these changes? (y/N): ");
            io::stdout().flush()?;
            
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;
            
            if confirm.trim().to_lowercase() == "y" {
                match editor.apply_all().await {
                    Ok(results) => {
                        for result in results {
                            println!("{}", result);
                            context.track_change(FileChange {
                                path: result.clone(),
                                operation: "apply".to_string(),
                                timestamp: chrono::Utc::now().to_rfc3339(),
                                backup_path: Some(format!("{}.backup", result)),
                            });
                        }
                    }
                    Err(e) => println!("❌ Error: {}", e),
                }
            } else {
                println!("❌ Cancelled");
            }
        }
        
        "rollback" => {
            match context.rollback_last_change().await {
                Ok(msg) => println!("✅ {}", msg),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        
        "clear" => {
            *context = ConversationContext::new();
            editor.clear();
            println!("🗑️  Context cleared");
        }
        
        "plan" => {
            if parts.len() < 2 {
                println!("❌ Usage: /plan <goal>");
            println!("/todo <task> - Add to TODO list");
            println!("/done <task> - Mark task complete");
            println!("/rule <rule> - Add project rule");
            println!("/context - Show wiseowl context");
            println!("/version - Show version");
                return Ok(true);
            }
            let goal = parts[1..].join(" ");
            println!("🎯 Creating plan: {}", goal);
    println!("⏳ This may take 30-60 seconds...");
            
            let prompt = format!("Create a detailed step-by-step plan to: {}

Provide 5-10 concrete steps as a numbered list.", goal);
            let response = get_complete_response(client, model, &prompt).await?;
            
            let steps: Vec<String> = response.lines()
                .filter(|line| line.trim().chars().next().map_or(false, |c| c.is_numeric()))
                .map(|line| line.split_once(".").map(|(_, rest)| rest.trim().to_string()).unwrap_or_else(|| line.trim().to_string()))
                .collect();
            
            if !steps.is_empty() {
                let plan = crate::planning::Plan::new(goal, steps);
                println!("
{}", plan.display());
                plan.save("current").await?;
                println!("💾 Plan saved. Use /next to execute steps.");
            }
        }
        
        "next" => {
            if let Some(mut plan) = crate::planning::Plan::load("current").await? {
                if let Some(step) = plan.next_step() {
                    println!("📍 Step {}: {}", step.number, step.description);
                    println!("
Executing...");
                    
                    let prompt = format!("Execute this step: {}

Use tools as needed and provide the result.", step.description);
                    let response = crate::streaming::stream_with_tools(client, model, &prompt).await?;
                    
                    plan.complete_step(step.number, "Completed".to_string());
                    plan.save("current").await?;
                    
                    if plan.is_complete() {
                        println!("
🎉 Plan complete!");
                    } else {
                        println!("
✅ Step complete. Use /next for next step.");
                    }
                } else {
                    println!("✅ All steps completed!");
                }
            } else {
                println!("❌ No active plan. Use /plan to create one.");
            }
        }
        
        "todo" => {
            if parts.len() < 2 {
                println!("❌ Usage: /todo <task>");
                return Ok(true);
            }
            let task = parts[1..].join(" ");
            owl.add_todo(&task).await?;
            println!("✅ Added to TODO");
        }
        
        "done" => {
            if parts.len() < 2 {
                println!("❌ Usage: /done <task>");
                return Ok(true);
            }
            let task = parts[1..].join(" ");
            owl.complete_todo(&task).await?;
            println!("✅ Marked as done");
        }
        
        "rule" => {
            if parts.len() < 2 {
                println!("❌ Usage: /rule <rule>");
                return Ok(true);
            }
            let rule = parts[1..].join(" ");
            owl.add_rule(&rule).await?;
            println!("✅ Added to RULES");
        }
        
        "git" => {
            if parts.len() < 2 {
                println!("Usage: /git <status|diff|log|commit>");
                return Ok(true);
            }
            match parts[1] {
                "status" => {
                    match crate::git::GitHelper::status() {
                        Ok(s) => println!("📊 Git Status:
{}", s),
                        Err(e) => println!("❌ {}", e),
                    }
                }
                "diff" => {
                    match crate::git::GitHelper::diff() {
                        Ok(d) => println!("📝 Git Diff:
{}", d),
                        Err(e) => println!("❌ {}", e),
                    }
                }
                "log" => {
                    match crate::git::GitHelper::log(10) {
                        Ok(l) => println!("📜 Git Log:
{}", l),
                        Err(e) => println!("❌ {}", e),
                    }
                }
                "commit" => {
                    if parts.len() < 3 {
                        println!("Usage: /git commit <message>");
                        return Ok(true);
                    }
                    let msg = parts[2..].join(" ");
                    match crate::git::GitHelper::commit(&msg) {
                        Ok(r) => println!("✅ {}", r),
                        Err(e) => println!("❌ {}", e),
                    }
                }
                _ => println!("Unknown git command"),
            }
        }
        
        "stats" => {
            let stats = crate::stats::SessionStats::new();
            println!("{}", stats.display());
        }
        
        "monitor" => {
            use crossterm::event::{self, Event, KeyCode};
            use std::time::Duration;
            
            crate::tui::TUI::init()?;
            crate::tui::TUI::clear()?;
            
            loop {
                crate::tui::TUI::draw_header("OCLI MONITOR")?;
                crate::tui::TUI::print_at(2, 3, "Session Statistics", crossterm::style::Color::Cyan)?;
                crate::tui::TUI::print_at(4, 5, &format!("Messages: {}", context.messages.len()), crossterm::style::Color::White)?;
                crate::tui::TUI::print_at(4, 6, &format!("Files: {}", context.working_files.len()), crossterm::style::Color::White)?;
                crate::tui::TUI::print_at(4, 7, &format!("Changes: {}", context.file_changes.len()), crossterm::style::Color::White)?;
                
                crate::tui::TUI::draw_status_line("Press q to exit")?;
                
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('q') {
                            break;
                        }
                    }
                }
            }
            
            crate::tui::TUI::cleanup()?;
        }
        
        "version" => {
            println!("🦉 OCLI v0.1.0");
            println!("Claude Code-like interface for Ollama");
        }
        
        "context" => {
            let ctx = owl.get_context().await?;
            println!("{}", ctx);
        }
        
        "exit" => return Ok(false),
        
        _ => println!("❌ Unknown command: /{}", parts[0]),
    }
    
    Ok(true)
}

async fn init_project_mode(client: &Client, model: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 OCLI - Initializing project");
    println!("⏳ Analyzing project structure...");
    
    let current_dir = std::env::current_dir()?;
    println!("📁 Analyzing: {}", current_dir.display());
    
    let project_info = analyze_project(&current_dir).await?;
    
    let prompt = format!("Analyze this project:\n\n{}\n\nProvide insights on structure, tech stack, and recommendations.", project_info);
    
    send_prompt_and_stream_response(client, model, &prompt).await?;
    
    let ocli_dir = current_dir.join(".ocli");
    tokio::fs::create_dir_all(&ocli_dir).await?;
    
    let context_file = ocli_dir.join("project.json");
    let context_data = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "directory": current_dir.to_string_lossy(),
        "analysis": project_info
    });
    
    tokio::fs::write(context_file, serde_json::to_string_pretty(&context_data)?).await?;
    println!("\n💾 Project context saved to .ocli/project.json");
    
    Ok(())
}

async fn analyze_project(dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut analysis = String::new();
    
    let project_type = detect_project_type(dir).await;
    analysis.push_str(&format!("Project Type: {}\n\n", project_type));
    
    analysis.push_str("Key Files:\n");
    let mut entries = tokio::fs::read_dir(dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        
        if is_important_file(&name) {
            let metadata = entry.metadata().await?;
            analysis.push_str(&format!("- {} ({} bytes)\n", name, metadata.len()));
        }
    }
    
    Ok(analysis)
}

async fn detect_project_type(dir: &Path) -> String {
    if dir.join("Cargo.toml").exists() {
        "Rust"
    } else if dir.join("package.json").exists() {
        "Node.js"
    } else if dir.join("requirements.txt").exists() || dir.join("pyproject.toml").exists() {
        "Python"
    } else if dir.join("go.mod").exists() {
        "Go"
    } else {
        "Unknown"
    }.to_string()
}

fn is_important_file(name: &str) -> bool {
    matches!(name, 
        "Cargo.toml" | "package.json" | "requirements.txt" | "pyproject.toml" |
        "go.mod" | "README.md" | "LICENSE" | ".gitignore" | "Dockerfile"
    )
}

async fn send_prompt_and_stream_response(client: &Client, model: &str, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    print!("AI: ");
    io::stdout().flush()?;
    
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        println!("Error: HTTP {}", response.status());
        return Ok(());
    }
    
    let mut stream = response.bytes_stream();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Ok(text) = std::str::from_utf8(&chunk) {
            for line in text.lines() {
                if line.trim().is_empty() { continue; }
                if let Ok(json) = serde_json::from_str::<Value>(line) {
                    if let Some(response_text) = json.get("response").and_then(|r| r.as_str()) {
                        print!("{}", response_text);
                        io::stdout().flush()?;
                    }
                    if json.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                        println!("\n");
                        break;
                    }
                }
            }
        }
    }
    
    Ok(())
}

async fn get_complete_response(client: &Client, model: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await?;

    let json: Value = response.json().await?;
    Ok(json.get("response").and_then(|r| r.as_str()).unwrap_or("").to_string())
}

async fn plan_mode(client: &Client, model: &str, goal: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Planning Mode: {}", goal);
    println!("Generating plan...\n");
    
    let prompt = format!(
        "Create a detailed step-by-step plan to accomplish this goal: {}\n\n\
        Provide 5-10 concrete, actionable steps.\n\
        Format as a numbered list:\n\
        1. First step\n\
        2. Second step\n\
        etc.",
        goal
    );
    
    let response = get_complete_response(client, model, &prompt).await?;
    
    let steps: Vec<String> = response
        .lines()
        .filter(|line| line.trim().starts_with(char::is_numeric))
        .map(|line| {
            line.split_once('.').map(|(_, rest)| rest.trim().to_string())
                .unwrap_or_else(|| line.trim().to_string())
        })
        .collect();
    
    if steps.is_empty() {
        println!("❌ Could not generate plan");
        return Ok(());
    }
    
    let plan = Plan::new(goal.to_string(), steps);
    println!("{}", plan.display());
    
    plan.save("current").await?;
    println!("💾 Plan saved. Use 'ocli chat --session current' to execute it.");
    
    Ok(())
}
