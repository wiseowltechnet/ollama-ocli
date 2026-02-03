use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileChange {
    pub path: String,
    pub operation: String,
    pub timestamp: String,
    pub backup_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationContext {
    pub working_files: HashMap<String, String>,
    pub file_changes: Vec<FileChange>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            working_files: HashMap::new(),
            file_changes: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_file(&mut self, path: String, content: String) {
        self.working_files.insert(path, content);
    }

    pub fn track_change(&mut self, change: FileChange) {
        self.file_changes.push(change);
    }

    pub fn add_message(&mut self, role: String, content: String) {
        self.messages.push(Message {
            role,
            content,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }

    pub fn get_context_summary(&self) -> String {
        let mut summary = String::new();

        if !self.working_files.is_empty() {
            summary.push_str("Working files:\n");
            for path in self.working_files.keys() {
                summary.push_str(&format!("  - {}\n", path));
            }
        }

        if !self.file_changes.is_empty() {
            summary.push_str("\nRecent changes:\n");
            for change in self.file_changes.iter().rev().take(5) {
                summary.push_str(&format!("  - {} {}\n", change.operation, change.path));
            }
        }

        summary
    }

    pub async fn save(&self, session_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let context_dir = std::env::current_dir()?.join(".ocli").join("context");
        tokio::fs::create_dir_all(&context_dir).await?;

        let context_file = context_dir.join(format!("{}.json", session_name));
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(context_file, json).await?;

        Ok(())
    }

    pub async fn load(session_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let context_file = std::env::current_dir()?
            .join(".ocli")
            .join("context")
            .join(format!("{}.json", session_name));

        if !context_file.exists() {
            return Ok(Self::new());
        }

        let content = tokio::fs::read_to_string(context_file).await?;
        let context = serde_json::from_str(&content)?;
        Ok(context)
    }

    pub async fn rollback_last_change(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(change) = self.file_changes.pop() {
            if let Some(backup) = &change.backup_path {
                tokio::fs::copy(backup, &change.path).await?;
                tokio::fs::remove_file(backup).await?;
                return Ok(format!("Rolled back: {}", change.path));
            }
        }
        Ok("No changes to rollback".to_string())
    }
}
