use crate::helpers::command_line::PrintCommand;
use crate::{
    apis::call_request::call_gpt,
    models::general::llm::{self, Content, Message},
};

use std::error::Error;
use std::fs;
use std::vec;

use reqwest::Client;
use serde::de::DeserializeOwned;

// Make sure the path is correct relative to this file, or create the file if missing.
const CODE_TEMPLATE_RUST_PATH: &str = "web_template_rust/src/code_template.rs";
const EXEC_MAIN_RUST_PATH: &str = "web_template_rust/src/main.rs";
pub const WEBSERVER_RUST_PATH: &str = "web_template_rust/";
const API_SCHEMA_PATH: &str = "schemas/api_schema.json";

pub fn extend_ai_funtion(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_output = ai_func(func_input);
    // Extend the string to encourage only printing the output
    let msg: String = format!(
        "FUNCTION {} INSTRUCTION: You are a function printer. 
    You ONLY print the results of functions. Nothing else. No commentary. 
    Here is the input to the function: {}. Print out what the function will return.",
        ai_function_output, func_input
    );
    //Return the message
    Message {
        role: "system".to_string(),
        content: vec![Content {
            r#type: "text".to_string(),
            text: msg,
        }],
    }
}

// Performs calls to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //Extend Ai function
    let func_msg: Message = extend_ai_funtion(function_pass, &msg_context);
    // Print current status
    PrintCommand::AICall.print_agent_msg(agent_position, agent_operation);

    // Get LLM response
    let llm_response_result: Result<String, Box<dyn Error + Send + Sync + 'static>> =
        call_gpt(vec![func_msg.clone()]).await;

    match llm_response_result {
        Ok(response) => response,
        Err(_) => call_gpt(vec![func_msg.clone()])
            .await
            .expect("Failed to call GPT"),
    }
}

// Performs calls to LLM GPT with a decode function
pub async fn ai_task_request_decode<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;

    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode response from LLM with serde");

    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_RUST_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_RUST_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

//Save New BackEnd Code
pub fn save_rust_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_RUST_PATH);
    fs::write(path, contents).expect("Failed to write main.rs")
}

// Save Json Api Endpoint Schema
pub fn save_api_endpoint(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API endpoints")
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_function() {
        let extended_msg: Message = extend_ai_funtion(convert_user_input_to_goal, "dummy variable");
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api requests".to_string();
        let response: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Define user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(!response.is_empty(), "Response should not be empty");
        assert!(
            response.len() > 30,
            "Response should be longer than 30 characters"
        );
    }
}
