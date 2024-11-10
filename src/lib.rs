use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext,
    },
    Ollama,
};
use tokio_stream::StreamExt;

pub async fn generate_response(
    ollama: &Ollama,
    input: &str,
    context: Option<GenerationContext>,
) -> Result<(String, Option<GenerationContext>), Box<dyn std::error::Error>> {
    let mut request = GenerationRequest::new("mistral-nemo:latest".into(), input.to_string());
    if let Some(context) = context.clone() {
        request = request.context(context);
    }
    let mut stream = ollama.generate_stream(request).await?;

    let mut response = String::new();
    let mut new_context = None;

    while let Some(Ok(res)) = stream.next().await {
        for ele in res {
            response.push_str(&ele.response);
            if ele.context.is_some() {
                new_context = ele.context;
            }
        }
    }

    Ok((response, new_context))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_response() {
        let ollama = Ollama::default();
        let input = "Hello, how are you?";
        let (response, context) = generate_response(&ollama, input, None).await.unwrap();

        assert!(!response.is_empty(), "Response should not be empty");
        assert!(context.is_some(), "Context should be returned");

        // Test with context
        let (response2, context2) = generate_response(&ollama, "What did I just ask?", context).await.unwrap();

        assert!(!response2.is_empty(), "Response should not be empty");
        assert!(context2.is_some(), "Context should be returned");
        assert!(response2.contains("asked") || response2.contains("Hello") || response2.contains("how are you"),
                "Response should reference the previous question");
    }
}