use std::collections::HashMap;

pub struct SuggestionEngine {
    command_usage: HashMap<String, u32>,
    last_commands: Vec<String>,
}

impl SuggestionEngine {
    pub fn new() -> Self {
        Self {
            command_usage: HashMap::new(),
            last_commands: Vec::new(),
        }
    }
    
    pub fn record_command(&mut self, cmd: &str) {
        *self.command_usage.entry(cmd.to_string()).or_insert(0) += 1;
        self.last_commands.push(cmd.to_string());
        if self.last_commands.len() > 10 {
            self.last_commands.remove(0);
        }
    }
    
    pub fn suggest_typo(&self, input: &str) -> Option<String> {
        let commands = vec![
            "help", "plan", "next", "show-plan", "todo", "done", "rule", "context",
            "read", "write", "preview", "apply", "rollback", "mcp", "config",
            "export", "stats", "monitor", "git", "version", "history", "alias",
            "perf", "clear", "exit",
        ];
        
        let mut best_match = None;
        let mut best_distance = usize::MAX;
        
        for cmd in commands {
            let distance = levenshtein(input, cmd);
            if distance < best_distance && distance <= 2 {
                best_distance = distance;
                best_match = Some(cmd.to_string());
            }
        }
        
        best_match
    }
    
    pub fn suggest_next(&self) -> Option<String> {
        if self.last_commands.is_empty() {
            return None;
        }
        
        let last = self.last_commands.last()?;
        
        match last.as_str() {
            "write" | "preview" => Some("/apply".to_string()),
            "apply" => Some("/git commit".to_string()),
            "read" => Some("/write".to_string()),
            "plan" => Some("/next".to_string()),
            "next" => Some("/next".to_string()),
            _ => None,
        }
    }
    
    pub fn suggest_workflow(&self, context: &str) -> Option<String> {
        if context.contains("modified") && context.contains("file") {
            Some("💡 You might want to /git commit".to_string())
        } else if context.contains("error") {
            Some("💡 Try /perf to check performance".to_string())
        } else {
            None
        }
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();
    
    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }
    
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    
    for i in 0..=a_len { matrix[i][0] = i; }
    for j in 0..=b_len { matrix[0][j] = j; }
    
    for (i, a_char) in a.chars().enumerate() {
        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1
                ),
                matrix[i][j] + cost
            );
        }
    }
    
    matrix[a_len][b_len]
}
