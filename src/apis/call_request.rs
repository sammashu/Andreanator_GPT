#[allow(unused_imports)]
use crate::models::general::llm::{ApiResponse, ChatCompletion, Content, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

// Call Large Language Model (LLM) API CHAT-GPT4.1

pub async fn call_gpt(
    messages: Vec<Message>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();

    //Extract the API key from the environment variable
    let api_key: String =
        env::var("OPEN_ROUTER_AI_KEY").expect("OPEN_ROUTER_AI_KEY not set in .env file");

    // Configure endpoint URL
    let endpoint: &str = "https://openrouter.ai/api/v1/chat/completions";

    //create header
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    //Create client
    let client: Client = Client::builder().default_headers(headers).build()?;

    // Create the request body
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "openai/gpt-4.1".to_string(),
        messages,
        temperature: 0.1, // Adjust temperature as needed but consistent answers
    };

    let res: ApiResponse = client
        .post(endpoint)
        .json(&chat_completion)
        .send()
        .await?
        .json()
        .await?;

    let content = res
        .choices
        .into_iter()
        .next()
        .ok_or("No choices in response")?
        .message
        .content;

    // Send response back
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::general::llm::Message;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages: Vec<Message> = vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                r#type: "text".to_string(),
                text: "Hello, Andreanator this is a test. Give me a short response!".to_string(),
            }],
        }];

        let res = call_gpt(messages).await;
        if let Ok(response) = res {
            println!("Response: {}", response);
            assert!(!response.is_empty(), "Response should not be empty");
        } else {
            panic!("Failed to call GPT API");
        }
    }
}
