use async_trait::async_trait;
use std::error::Error;
use chrono::Local;
use crate::tools::Tool;

pub struct TimeTool;

#[async_trait]
impl Tool for TimeTool {
    fn name(&self) -> String {
        "time".to_string()
    }

    fn description(&self) -> String {
        "Get the current local time".to_string()
    }

    async fn call(&self, _input: &str) -> Result<String, Box<dyn Error>> {
        let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(format!("The current local time is: {}", current_time))
    }
}