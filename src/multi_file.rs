use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEdit {
    pub path: String,
    pub content: String,
    pub operation: EditOperation,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EditOperation {
    Create,
    Modify,
    Delete,
}

pub struct MultiFileEditor {
    pending_edits: HashMap<String, FileEdit>,
}

impl MultiFileEditor {
    pub fn new() -> Self {
        Self {
            pending_edits: HashMap::new(),
        }
    }

    pub fn add_edit(&mut self, edit: FileEdit) {
        self.pending_edits.insert(edit.path.clone(), edit);
    }

    pub fn show_preview(&self) -> String {
        let mut preview = String::from("📋 Pending Changes:\n\n");

        for (path, edit) in &self.pending_edits {
            match edit.operation {
                EditOperation::Create => preview.push_str(&format!("✨ Create: {}\n", path)),
                EditOperation::Modify => preview.push_str(&format!("✏️  Modify: {}\n", path)),
                EditOperation::Delete => preview.push_str(&format!("🗑️  Delete: {}\n", path)),
            }
        }

        preview
    }

    pub async fn apply_all(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for (path, edit) in self.pending_edits.drain() {
            match edit.operation {
                EditOperation::Create | EditOperation::Modify => {
                    // Backup existing
                    if std::path::Path::new(&path).exists() {
                        let backup = format!("{}.backup", path);
                        tokio::fs::copy(&path, &backup).await?;
                    }

                    tokio::fs::write(&path, &edit.content).await?;
                    results.push(format!("✅ {}", path));
                }
                EditOperation::Delete => {
                    let backup = format!("{}.backup", path);
                    tokio::fs::rename(&path, &backup).await?;
                    results.push(format!("🗑️  {}", path));
                }
            }
        }

        Ok(results)
    }

    pub fn clear(&mut self) {
        self.pending_edits.clear();
    }

    pub fn has_pending(&self) -> bool {
        !self.pending_edits.is_empty()
    }
}
