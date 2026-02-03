use crate::lcars::*;

pub struct DiffViewer {
    changes: Vec<FileDiff>,
}

#[derive(Clone)]
pub struct FileDiff {
    pub path: String,
    pub old_content: String,
    pub new_content: String,
}

impl DiffViewer {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }
    
    pub fn add_change(&mut self, path: String, old: String, new: String) {
        self.changes.push(FileDiff {
            path,
            old_content: old,
            new_content: new,
        });
    }
    
    pub fn show(&self) {
        if self.changes.is_empty() {
            println!("No changes to show");
            return;
        }
        
        println!("{}╔═══════════════════════════════════════════════════════════════╗{}", ORANGE, RESET);
        println!("{}║  {}DIFF VIEWER{}                                                     {}║{}", ORANGE, BLUE, ORANGE, ORANGE, RESET);
        println!("{}╚═══════════════════════════════════════════════════════════════╝{}", ORANGE, RESET);
        println!();
        
        for (i, change) in self.changes.iter().enumerate() {
            println!("{}{}. {}{}", PURPLE, i + 1, change.path, RESET);
            
            let old_lines: Vec<&str> = change.old_content.lines().collect();
            let new_lines: Vec<&str> = change.new_content.lines().collect();
            
            let max_len = old_lines.len().max(new_lines.len());
            
            for i in 0..max_len.min(10) {
                let old_line = old_lines.get(i).unwrap_or(&"");
                let new_line = new_lines.get(i).unwrap_or(&"");
                
                if old_line != new_line {
                    if !old_line.is_empty() {
                        println!("  {}-{} {}", RED, RESET, old_line);
                    }
                    if !new_line.is_empty() {
                        println!("  {}+{} {}", GREEN, RESET, new_line);
                    }
                }
            }
            
            if max_len > 10 {
                println!("  ... ({} more lines)", max_len - 10);
            }
            println!();
        }
        
        println!("💡 Use /diff accept or /diff reject");
    }
    
    pub fn accept_all(&mut self) -> Vec<FileDiff> {
        self.changes.drain(..).collect()
    }
    
    pub fn reject_all(&mut self) {
        self.changes.clear();
    }
    
    pub fn count(&self) -> usize {
        self.changes.len()
    }
}

const GREEN: &str = "\x1b[38;2;102;255;102m";
