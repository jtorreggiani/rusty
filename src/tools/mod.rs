pub mod time_tool;

use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    async fn call(&self, input: &str) -> Result<String, Box<dyn Error>>;
}