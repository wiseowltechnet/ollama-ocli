use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug)]
pub enum ToolResult {
    Success(String),
    Error(String),
}

pub fn get_available_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "read_file".to_string(),
            description: "Read contents of a file".to_string(),
            parameters: vec![Parameter {
                name: "path".to_string(),
                param_type: "string".to_string(),
                description: "File path to read".to_string(),
                required: true,
            }],
        },
        Tool {
            name: "write_file".to_string(),
            description: "Write content to a file".to_string(),
            parameters: vec![
                Parameter {
                    name: "path".to_string(),
                    param_type: "string".to_string(),
                    description: "File path to write".to_string(),
                    required: true,
                },
                Parameter {
                    name: "content".to_string(),
                    param_type: "string".to_string(),
                    description: "Content to write".to_string(),
                    required: true,
                },
            ],
        },
        Tool {
            name: "execute_bash".to_string(),
            description: "Execute a bash command".to_string(),
            parameters: vec![Parameter {
                name: "command".to_string(),
                param_type: "string".to_string(),
                description: "Bash command to execute".to_string(),
                required: true,
            }],
        },
        Tool {
            name: "search_files".to_string(),
            description: "Search for files matching a pattern".to_string(),
            parameters: vec![
                Parameter {
                    name: "pattern".to_string(),
                    param_type: "string".to_string(),
                    description: "Glob pattern to search".to_string(),
                    required: true,
                },
                Parameter {
                    name: "directory".to_string(),
                    param_type: "string".to_string(),
                    description: "Directory to search in".to_string(),
                    required: false,
                },
            ],
        },
        Tool {
            name: "list_directory".to_string(),
            description: "List contents of a directory".to_string(),
            parameters: vec![Parameter {
                name: "path".to_string(),
                param_type: "string".to_string(),
                description: "Directory path".to_string(),
                required: true,
            }],
        },
    ]
}

pub async fn execute_tool(call: &ToolCall) -> ToolResult {
    match call.tool.as_str() {
        "read_file" => execute_read_file(call).await,
        "write_file" => execute_write_file(call).await,
        "execute_bash" => execute_bash_command(call).await,
        "search_files" => execute_search_files(call).await,
        "list_directory" => execute_list_directory(call).await,
        _ => ToolResult::Error(format!("Unknown tool: {}", call.tool)),
    }
}

async fn execute_read_file(call: &ToolCall) -> ToolResult {
    let path = match call.parameters.get("path").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::Error("Missing 'path' parameter".to_string()),
    };

    match tokio::fs::read_to_string(path).await {
        Ok(content) => ToolResult::Success(content),
        Err(e) => ToolResult::Error(format!("Failed to read {}: {}", path, e)),
    }
}

async fn execute_write_file(call: &ToolCall) -> ToolResult {
    let path = match call.parameters.get("path").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::Error("Missing 'path' parameter".to_string()),
    };

    let content = match call.parameters.get("content").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return ToolResult::Error("Missing 'content' parameter".to_string()),
    };

    // Create backup if file exists
    if Path::new(path).exists() {
        let backup = format!("{}.backup", path);
        if let Err(e) = tokio::fs::copy(path, &backup).await {
            return ToolResult::Error(format!("Failed to create backup: {}", e));
        }
    }

    match tokio::fs::write(path, content).await {
        Ok(_) => ToolResult::Success(format!("Wrote to {}", path)),
        Err(e) => ToolResult::Error(format!("Failed to write {}: {}", path, e)),
    }
}

async fn execute_bash_command(call: &ToolCall) -> ToolResult {
    let command = match call.parameters.get("command").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return ToolResult::Error("Missing 'command' parameter".to_string()),
    };

    let output = Command::new("sh").arg("-c").arg(command).output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            let result = format!(
                "Exit: {}\nStdout: {}\nStderr: {}",
                out.status.code().unwrap_or(-1),
                stdout,
                stderr
            );
            ToolResult::Success(result)
        }
        Err(e) => ToolResult::Error(format!("Failed to execute: {}", e)),
    }
}

async fn execute_search_files(call: &ToolCall) -> ToolResult {
    let pattern = match call.parameters.get("pattern").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::Error("Missing 'pattern' parameter".to_string()),
    };

    let directory = call
        .parameters
        .get("directory")
        .and_then(|v| v.as_str())
        .unwrap_or(".");

    let output = Command::new("find")
        .arg(directory)
        .arg("-name")
        .arg(pattern)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            ToolResult::Success(stdout.to_string())
        }
        Err(e) => ToolResult::Error(format!("Failed to search: {}", e)),
    }
}

async fn execute_list_directory(call: &ToolCall) -> ToolResult {
    let path = match call.parameters.get("path").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::Error("Missing 'path' parameter".to_string()),
    };

    match tokio::fs::read_dir(path).await {
        Ok(mut entries) => {
            let mut result = String::new();
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(name) = entry.file_name().into_string() {
                    result.push_str(&format!("{}\n", name));
                }
            }
            ToolResult::Success(result)
        }
        Err(e) => ToolResult::Error(format!("Failed to list {}: {}", path, e)),
    }
}

pub fn parse_tool_calls(text: &str) -> Vec<ToolCall> {
    let mut calls = Vec::new();

    // Look for tool calls in format: <tool_call>{tool:name,parameters:{...}}</tool_call>
    let mut start = 0;
    while let Some(begin) = text[start..].find("<tool_call>") {
        let begin = start + begin + "<tool_call>".len();
        if let Some(end) = text[begin..].find("</tool_call>") {
            let json_str = &text[begin..begin + end];
            if let Ok(call) = serde_json::from_str::<ToolCall>(json_str) {
                calls.push(call);
            }
            start = begin + end + "</tool_call>".len();
        } else {
            break;
        }
    }

    calls
}

pub fn tools_to_prompt() -> String {
    let tools = get_available_tools();
    let mut prompt = String::from("You have access to these tools:\n\n");

    for tool in tools {
        prompt.push_str(&format!("Tool: {}\n", tool.name));
        prompt.push_str(&format!("Description: {}\n", tool.description));
        prompt.push_str("Parameters:\n");
        for param in tool.parameters {
            let req = if param.required {
                "required"
            } else {
                "optional"
            };
            prompt.push_str(&format!(
                "  - {} ({}): {} [{}]\n",
                param.name, param.param_type, param.description, req
            ));
        }
        prompt.push_str("\n");
    }

    prompt.push_str("To use a tool, output: <tool_call>{\"tool\":\"tool_name\",\"parameters\":{...}}</tool_call>\n");
    prompt.push_str("You can call multiple tools in sequence.\n\n");

    prompt
}
