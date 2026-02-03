use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub server: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MCPRequest {
    jsonrpc: String,
    id: u32,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct MCPResponse {
    jsonrpc: String,
    id: u32,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

pub struct MCPClient {
    servers: Vec<MCPServer>,
    tools: Vec<MCPTool>,
}

impl MCPClient {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            tools: Vec::new(),
        }
    }

    pub async fn load_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = std::env::current_dir()?.join(".ocli").join("mcp_servers.json");
        
        if !config_path.exists() {
            // Create default config
            let default_config = serde_json::json!({
                "servers": []
            });
            tokio::fs::create_dir_all(config_path.parent().unwrap()).await?;
            tokio::fs::write(&config_path, serde_json::to_string_pretty(&default_config)?).await?;
            return Ok(());
        }

        let content = tokio::fs::read_to_string(&config_path).await?;
        let config: serde_json::Value = serde_json::from_str(&content)?;
        
        if let Some(servers) = config.get("servers").and_then(|s| s.as_array()) {
            for server in servers {
                if let Ok(mcp_server) = serde_json::from_value::<MCPServer>(server.clone()) {
                    self.servers.push(mcp_server);
                }
            }
        }

        Ok(())
    }

    pub async fn discover_tools(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for server in &self.servers {
            match self.list_tools(&server.name).await {
                Ok(tools) => {
                    for tool in tools {
                        self.tools.push(MCPTool {
                            name: tool.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                            description: tool.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                            server: server.name.clone(),
                        });
                    }
                }
                Err(e) => eprintln!("Failed to discover tools from {}: {}", server.name, e),
            }
        }
        Ok(())
    }

    async fn list_tools(&self, server_name: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let server = self.servers.iter().find(|s| s.name == server_name)
            .ok_or("Server not found")?;

        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "tools/list".to_string(),
            params: serde_json::json!({}),
        };

        let response = self.send_request(server, &request).await?;
        
        if let Some(result) = response.result {
            if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                return Ok(tools.clone());
            }
        }

        Ok(Vec::new())
    }

    async fn send_request(&self, server: &MCPServer, request: &MCPRequest) -> Result<MCPResponse, Box<dyn std::error::Error>> {
        let mut child = Command::new(&server.command)
            .args(&server.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let request_json = serde_json::to_string(request)?;
        stdin.write_all(request_json.as_bytes())?;
        stdin.write_all(b"\n")?;
        drop(stdin);

        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
        let reader = BufReader::new(stdout);
        
        for line in reader.lines() {
            let line = line?;
            if let Ok(response) = serde_json::from_str::<MCPResponse>(&line) {
                return Ok(response);
            }
        }

        Err("No valid response received".into())
    }

    pub fn list_available_tools(&self) -> Vec<&MCPTool> {
        self.tools.iter().collect()
    }

    pub async fn call_tool(&self, tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let tool = self.tools.iter().find(|t| t.name == tool_name)
            .ok_or("Tool not found")?;

        let server = self.servers.iter().find(|s| s.name == tool.server)
            .ok_or("Server not found")?;

        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: 2,
            method: "tools/call".to_string(),
            params: serde_json::json!({
                "name": tool_name,
                "arguments": params
            }),
        };

        let response = self.send_request(server, &request).await?;
        
        if let Some(result) = response.result {
            return Ok(result);
        }

        if let Some(error) = response.error {
            return Err(format!("MCP error: {:?}", error).into());
        }

        Err("No result from MCP server".into())
    }
}
