use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    pub number: usize,
    pub description: String,
    pub completed: bool,
    pub result: Option<String>,
}

impl Plan {
    pub fn new(goal: String, steps: Vec<String>) -> Self {
        Self {
            goal,
            steps: steps
                .into_iter()
                .enumerate()
                .map(|(i, desc)| PlanStep {
                    number: i + 1,
                    description: desc,
                    completed: false,
                    result: None,
                })
                .collect(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn display(&self) -> String {
        let mut output = format!("📋 Plan: {}\n\n", self.goal);

        for step in &self.steps {
            let status = if step.completed { "✅" } else { "⬜" };
            output.push_str(&format!(
                "{}. {} {}\n",
                step.number, status, step.description
            ));
            if let Some(result) = &step.result {
                output.push_str(&format!("   → {}\n", result));
            }
        }

        output
    }

    pub fn complete_step(&mut self, number: usize, result: String) {
        if let Some(step) = self.steps.get_mut(number - 1) {
            step.completed = true;
            step.result = Some(result);
        }
    }

    pub fn next_step(&self) -> Option<&PlanStep> {
        self.steps.iter().find(|s| !s.completed)
    }

    pub fn is_complete(&self) -> bool {
        self.steps.iter().all(|s| s.completed)
    }

    pub async fn save(&self, session: &str) -> Result<(), Box<dyn std::error::Error>> {
        let plan_dir = std::env::current_dir()?.join(".ocli").join("plans");
        tokio::fs::create_dir_all(&plan_dir).await?;

        let plan_file = plan_dir.join(format!("{}.json", session));
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(plan_file, json).await?;

        Ok(())
    }

    pub async fn load(session: &str) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let plan_file = std::env::current_dir()?
            .join(".ocli")
            .join("plans")
            .join(format!("{}.json", session));

        if !plan_file.exists() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(plan_file).await?;
        let plan = serde_json::from_str(&content)?;
        Ok(Some(plan))
    }
}
