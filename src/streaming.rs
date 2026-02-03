use crate::tools::{ToolResult, execute_tool, parse_tool_calls};
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};

pub async fn stream_with_tools(
    client: &Client,
    model: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut full_response = String::new();
    let mut buffer = String::new();
    let start_time = std::time::Instant::now();
    let mut token_count = 0;

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
        return Err(format!("HTTP {}", response.status()).into());
    }

    let mut stream = response.bytes_stream();
    // Print status line
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║ ⏳ Generating response...                                ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    print!("AI: ");

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Ok(text) = std::str::from_utf8(&chunk) {
            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(json) = serde_json::from_str::<Value>(line) {
                    if let Some(response_text) = json.get("response").and_then(|r| r.as_str()) {
                        buffer.push_str(response_text);
                        token_count += 1;

                        // Update status line every 10 tokens
                        if token_count % 10 == 0 {
                            let elapsed = start_time.elapsed().as_secs_f64();
                            let tps = token_count as f64 / elapsed;
                            print!("\x1b[2A\r║ ⏳ {} tokens | {:.1} tok/s | {:.1}s elapsed{:20}║\x1b[2B\r", token_count, tps, elapsed, "");
                            io::stdout().flush().ok();
                        }
                        full_response.push_str(response_text);

                        // Check for complete tool calls
                        let tool_calls = parse_tool_calls(&buffer);
                        if !tool_calls.is_empty() {
                            println!("\n");
                            for call in tool_calls {
                                println!("🔧 Executing: {}", call.tool);
                                match execute_tool(&call).await {
                                    ToolResult::Success(result) => {
                                        println!(
                                            "✅ Result: {}",
                                            result.lines().take(5).collect::<Vec<_>>().join("\n")
                                        );
                                    }
                                    ToolResult::Error(err) => {
                                        println!("❌ Error: {}", err);
                                    }
                                }
                            }
                            buffer.clear();
                            print!("\nAI: ");
                            io::stdout().flush()?;
                        } else {
                            print!("{}", response_text);
                            io::stdout().flush()?;
                        }
                    }
                    if json.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                        println!("\n");
                        break;
                    }
                }
            }
        }
    }

    Ok(full_response)
}
