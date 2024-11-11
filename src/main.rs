use ollama_rs::Ollama;
use tokio::io::{stdout, AsyncWriteExt};

use rusty::{generate_response, tools::time_tool::TimeTool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let mut stdout = stdout();
    let mut context = None;
    let time_tool = TimeTool;

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let (response, new_context) = generate_response(&ollama, input, context, &time_tool).await?;
        stdout.write_all(response.as_bytes()).await?;
        stdout.flush().await?;

        context = new_context;
    }

    Ok(())
}