use ollama_rs::Ollama;
use tokio::io::{stdout, AsyncWriteExt};

mod lib;
use lib::generate_response;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let mut stdout = stdout();
    let mut context = None;

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let (response, new_context) = generate_response(&ollama, input, context).await?;
        stdout.write_all(response.as_bytes()).await?;
        stdout.flush().await?;

        context = new_context;
    }

    Ok(())
}