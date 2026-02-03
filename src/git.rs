use std::process::Command;

pub struct GitHelper;

impl GitHelper {
    pub fn status() -> Result<String, String> {
        let output = Command::new("git")
            .args(&["status", "--short"])
            .output()
            .map_err(|e| format!("Git error: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn diff() -> Result<String, String> {
        let output = Command::new("git")
            .args(&["diff"])
            .output()
            .map_err(|e| format!("Git error: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn commit(message: &str) -> Result<String, String> {
        let output = Command::new("git")
            .args(&["commit", "-am", message])
            .output()
            .map_err(|e| format!("Git error: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn log(count: usize) -> Result<String, String> {
        let output = Command::new("git")
            .args(&["log", &format!("-{}", count), "--oneline"])
            .output()
            .map_err(|e| format!("Git error: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
