use std::path::Path;

pub fn suggest_file(path: &str) -> String {
    if !Path::new(path).exists() {
        let parent = Path::new(path).parent();
        if let Some(p) = parent {
            if p.exists() {
                return format!(
                    "File not found: {}\n💡 Directory exists. Did you mean a file in {}?",
                    path,
                    p.display()
                );
            }
        }
        return format!("File not found: {}\n💡 Check the path and try again", path);
    }
    path.to_string()
}

pub fn suggest_command(cmd: &str) -> Option<String> {
    let commands = vec![
        "help",
        "plan",
        "next",
        "show-plan",
        "todo",
        "done",
        "rule",
        "context",
        "read",
        "write",
        "preview",
        "apply",
        "rollback",
        "clear",
        "exit",
        "mcp",
        "config",
        "export",
        "stats",
        "monitor",
        "git",
        "version",
    ];

    // Simple Levenshtein distance
    let mut best_match = None;
    let mut best_distance = usize::MAX;

    for &command in &commands {
        let distance = levenshtein(cmd, command);
        if distance < best_distance && distance <= 2 {
            best_distance = distance;
            best_match = Some(command);
        }
    }

    best_match.map(|s| s.to_string())
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for (i, a_char) in a.chars().enumerate() {
        for (j, b_char) in b.chars().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[a_len][b_len]
}

pub fn format_mcp_error(error: &str) -> String {
    if error.contains("not found") {
        format!("❌ {}\n💡 Use /mcp list to see available tools", error)
    } else if error.contains("connection") || error.contains("timeout") {
        format!("❌ {}\n💡 Check if MCP server is running", error)
    } else {
        format!("❌ {}", error)
    }
}
