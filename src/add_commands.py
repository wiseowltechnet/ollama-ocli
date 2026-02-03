with open("main.rs", "r") as f:
    content = f.read()

# Add /mcp call after list
mcp_call = """                "call" => {
                    if parts.len() < 3 {
                        println!("Usage: /mcp call <tool_name> [params]");
                        return Ok(true);
                    }
                    
                    let mut mcp_client = crate::mcp::MCPClient::new();
                    mcp_client.load_config().await?;
                    mcp_client.discover_tools().await?;
                    
                    let tool_name = parts[2];
                    let params = if parts.len() > 3 {
                        serde_json::json!({"input": parts[3..].join(" ")})
                    } else {
                        serde_json::json!({})
                    };
                    
                    match mcp_client.call_tool(tool_name, params).await {
                        Ok(result) => println!("✅ Result: {:?}", result),
                        Err(e) => println!("❌ Error: {}", e),
                    }
                }
"""

content = content.replace(
    """                _ => println!("Unknown mcp command"),""",
    mcp_call + """                _ => println!("Unknown mcp command"),"""
)

# Add /config before version
config_cmd = """        "config" => {
            if parts.len() < 2 {
                println!("Usage: /config <get|set|list>");
                return Ok(true);
            }
            
            let config_file = std::env::current_dir()?.join(".ocli").join("config.json");
            
            match parts[1] {
                "list" => {
                    if config_file.exists() {
                        let content = tokio::fs::read_to_string(&config_file).await?;
                        println!("📋 Configuration:\\n{}", content);
                    } else {
                        println!("No config file found");
                    }
                }
                "set" => {
                    if parts.len() < 4 {
                        println!("Usage: /config set <key> <value>");
                        return Ok(true);
                    }
                    let key = parts[2];
                    let value = parts[3..].join(" ");
                    
                    tokio::fs::create_dir_all(config_file.parent().unwrap()).await?;
                    let mut config = if config_file.exists() {
                        let c = tokio::fs::read_to_string(&config_file).await?;
                        serde_json::from_str(&c).unwrap_or(serde_json::json!({}))
                    } else {
                        serde_json::json!({})
                    };
                    
                    config[key] = serde_json::json!(value);
                    tokio::fs::write(&config_file, serde_json::to_string_pretty(&config)?).await?;
                    println!("✅ Set {} = {}", key, value);
                }
                "get" => {
                    if parts.len() < 3 {
                        println!("Usage: /config get <key>");
                        return Ok(true);
                    }
                    if config_file.exists() {
                        let c = tokio::fs::read_to_string(&config_file).await?;
                        let config: serde_json::Value = serde_json::from_str(&c)?;
                        if let Some(value) = config.get(parts[2]) {
                            println!("{} = {:?}", parts[2], value);
                        } else {
                            println!("Key not found");
                        }
                    } else {
                        println!("No config file");
                    }
                }
                _ => println!("Unknown config command"),
            }
        }
        
"""

content = content.replace(
    """        "version" => {""",
    config_cmd + """        "version" => {"""
)

# Add /export before exit
export_cmd = """        "export" => {
            let export_file = if parts.len() > 1 {
                parts[1].to_string()
            } else {
                format!("conversation_{}.md", chrono::Local::now().format("%Y%m%d_%H%M%S"))
            };
            
            let mut content = String::from("# OCLI Conversation Export\\n\\n");
            content.push_str(&format!("Exported: {}\\n\\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
            content.push_str(&format!("Session: {}\\n\\n", context.session_name));
            content.push_str(&format!("Total messages: {}\\n\\n", context.messages.len()));
            
            tokio::fs::write(&export_file, content).await?;
            println!("✅ Exported to {}", export_file);
        }
        
"""

content = content.replace(
    """        "exit" => return Ok(false),""",
    export_cmd + """        "exit" => return Ok(false),"""
)

with open("main.rs", "w") as f:
    f.write(content)

print("✅ Added commands")
