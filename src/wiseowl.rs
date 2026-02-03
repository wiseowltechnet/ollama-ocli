use std::path::PathBuf;

pub struct WiseOwl {
    root: PathBuf,
}

impl WiseOwl {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let root = std::env::current_dir()?.join("wiseowl");
        tokio::fs::create_dir_all(&root).await?;

        let owl = Self { root };
        owl.create_structure().await?;

        Ok(owl)
    }

    async fn create_structure(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create TODO.md if not exists
        let todo_file = self.root.join("TODO.md");
        if !todo_file.exists() {
            let content = "# TODO\n\n## Active Tasks\n\n- [ ] \n\n## Completed\n\n- [x] \n";
            tokio::fs::write(&todo_file, content).await?;
        }

        // Create RULES.md if not exists
        let rules_file = self.root.join("RULES.md");
        if !rules_file.exists() {
            let content = "# Project Rules\n\n## Coding Standards\n\n- \n\n## Architecture\n\n- \n\n## Conventions\n\n- \n";
            tokio::fs::write(&rules_file, content).await?;
        }

        // Create CONTEXT.md if not exists
        let context_file = self.root.join("CONTEXT.md");
        if !context_file.exists() {
            let content =
                "# Project Context\n\n## Overview\n\n\n## Current State\n\n\n## Next Steps\n\n";
            tokio::fs::write(&context_file, content).await?;
        }

        Ok(())
    }

    pub async fn add_todo(&self, task: &str) -> Result<(), Box<dyn std::error::Error>> {
        let todo_file = self.root.join("TODO.md");
        let mut content = tokio::fs::read_to_string(&todo_file).await?;

        // Find "## Active Tasks" and add after it
        if let Some(pos) = content.find("## Active Tasks\n") {
            let insert_pos = pos + "## Active Tasks\n\n".len();
            content.insert_str(insert_pos, &format!("- [ ] {}\n", task));
            tokio::fs::write(&todo_file, content).await?;
        }

        Ok(())
    }

    pub async fn complete_todo(&self, task: &str) -> Result<(), Box<dyn std::error::Error>> {
        let todo_file = self.root.join("TODO.md");
        let content = tokio::fs::read_to_string(&todo_file).await?;

        let updated = content.replace(&format!("- [ ] {}", task), &format!("- [x] {}", task));

        tokio::fs::write(&todo_file, updated).await?;
        Ok(())
    }

    pub async fn add_rule(&self, rule: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rules_file = self.root.join("RULES.md");
        let mut content = tokio::fs::read_to_string(&rules_file).await?;

        if let Some(pos) = content.find("## Coding Standards\n") {
            let insert_pos = pos + "## Coding Standards\n\n".len();
            content.insert_str(insert_pos, &format!("- {}\n", rule));
            tokio::fs::write(&rules_file, content).await?;
        }

        Ok(())
    }

    pub async fn update_context(
        &self,
        section: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context_file = self.root.join("CONTEXT.md");
        let mut file_content = tokio::fs::read_to_string(&context_file).await?;

        let section_header = format!("## {}\n", section);
        if let Some(pos) = file_content.find(&section_header) {
            let start = pos + section_header.len();
            if let Some(next_section) = file_content[start..].find("\n## ") {
                file_content
                    .replace_range(start..start + next_section, &format!("\n{}\n", content));
            } else {
                file_content.push_str(&format!("\n{}\n", content));
            }
            tokio::fs::write(&context_file, file_content).await?;
        }

        Ok(())
    }

    pub async fn get_context(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut context = String::new();

        let todo = tokio::fs::read_to_string(self.root.join("TODO.md")).await?;
        let rules = tokio::fs::read_to_string(self.root.join("RULES.md")).await?;
        let ctx = tokio::fs::read_to_string(self.root.join("CONTEXT.md")).await?;

        context.push_str("=== WISEOWL CONTEXT ===\n\n");
        context.push_str(&todo);
        context.push_str("\n\n");
        context.push_str(&rules);
        context.push_str("\n\n");
        context.push_str(&ctx);

        Ok(context)
    }
}
