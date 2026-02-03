use serde_json::Value;
use std::collections::HashMap;

pub struct ContextManager {
    messages: Vec<Value>,
    token_count: usize,
    max_tokens: usize,
}

impl ContextManager {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            messages: Vec::new(),
            token_count: 0,
            max_tokens,
        }
    }
    
    pub fn add_message(&mut self, msg: Value) {
        let tokens = estimate_tokens(&msg);
        self.token_count += tokens;
        self.messages.push(msg);
        
        if self.token_count > self.max_tokens {
            self.prune_old();
        }
    }
    
    pub fn prune_old(&mut self) {
        while self.token_count > self.max_tokens && self.messages.len() > 2 {
            let msg = self.messages.remove(1);
            self.token_count -= estimate_tokens(&msg);
        }
    }
    
    pub fn summarize(&self) -> String {
        let mut summary = String::from("📝 Context Summary:\n\n");
        summary.push_str(&format!("Messages: {}\n", self.messages.len()));
        summary.push_str(&format!("Tokens: ~{}\n", self.token_count));
        summary.push_str(&format!("Max tokens: {}\n", self.max_tokens));
        
        let usage = (self.token_count as f64 / self.max_tokens as f64 * 100.0) as u32;
        summary.push_str(&format!("Usage: {}%\n", usage));
        
        summary
    }
    
    pub fn search(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for msg in &self.messages {
            if let Some(content) = msg.get("content").and_then(|v| v.as_str()) {
                if content.to_lowercase().contains(&query_lower) {
                    results.push(content.to_string());
                }
            }
        }
        
        results
    }
    
    pub fn token_count(&self) -> usize {
        self.token_count
    }
}

fn estimate_tokens(msg: &Value) -> usize {
    if let Some(content) = msg.get("content").and_then(|v| v.as_str()) {
        content.split_whitespace().count() * 4 / 3
    } else {
        0
    }
}
