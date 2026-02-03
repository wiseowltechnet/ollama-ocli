use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            name: "deepseek-coder:6.7b".to_string(),
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 4096,
        }
    }
}

pub struct ModelManager {
    current: String,
    configs: HashMap<String, ModelConfig>,
    available: Vec<String>,
}

impl ModelManager {
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        configs.insert(
            "deepseek-coder:6.7b".to_string(),
            ModelConfig::default(),
        );
        
        Self {
            current: "deepseek-coder:6.7b".to_string(),
            configs,
            available: vec![
                "deepseek-coder:6.7b".to_string(),
                "codellama:latest".to_string(),
                "llama2:latest".to_string(),
                "mistral:latest".to_string(),
            ],
        }
    }
    
    pub fn list(&self) -> &Vec<String> {
        &self.available
    }
    
    pub fn current(&self) -> &str {
        &self.current
    }
    
    pub fn switch(&mut self, model: &str) -> Result<(), String> {
        if !self.available.contains(&model.to_string()) {
            return Err(format!("Model {} not available", model));
        }
        
        self.current = model.to_string();
        
        if !self.configs.contains_key(model) {
            self.configs.insert(
                model.to_string(),
                ModelConfig {
                    name: model.to_string(),
                    ..Default::default()
                },
            );
        }
        
        Ok(())
    }
    
    pub fn get_config(&self, model: &str) -> Option<&ModelConfig> {
        self.configs.get(model)
    }
    
    pub fn set_config(&mut self, model: &str, config: ModelConfig) {
        self.configs.insert(model.to_string(), config);
    }
    
    pub fn current_config(&self) -> &ModelConfig {
        self.configs.get(&self.current).unwrap()
    }
}
