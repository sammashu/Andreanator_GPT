use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};

use crate::helpers::general::{
    check_status_code, read_code_template_contents, read_exec_main_contents, save_api_endpoint,
    save_rust_backend_code, WEBSERVER_RUST_PATH,
};

use crate::helpers::hammer_loader::HammerLoader;

use crate::helpers::command_line::{PrintCommand, confirm_safe_code};
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions, RouteObject};
use async_trait::async_trait;
use reqwest::Client;
use tokio::io::stderr;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops backend code for webserver and json database".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }

    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template_str: String = read_code_template_contents();

        //Concatenate Instructions
        let mut msg_context: String = format!(
            "CODE_TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;

        save_rust_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "CODE_TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;

        save_rust_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fixe_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let mut msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE< JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;

        save_rust_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_extract_rest_api_endpoints(&self) -> String {
        let backend_code: String = read_exec_main_contents();

        //Structure message context
        let msg_context: String = format!("CODE_INPUT: {}", backend_code);
        let ai_response: String = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;

        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let loader = HammerLoader::new();

        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    loader
                        .execute_with_loading(
                            async {
                                self.call_initial_backend_code(factsheet).await;
                                Ok(()) as Result<(), Box<dyn std::error::Error>>
                            },
                            "Discovering backend architecture",
                        )
                        .await?;
                    println!("✅ Backend architecture discovered!");
                    self.attributes.state = AgentState::Working;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        loader
                            .execute_with_loading(
                                async {
                                    self.call_improved_backend_code(factsheet).await;
                                    Ok(()) as Result<(), Box<dyn std::error::Error>>
                                },
                                "Improving backend code",
                            )
                            .await?;
                        println!("✅ Backend code improved!");
                    } else {
loader
                            .execute_with_loading(
                                async {
                                    self.call_fixe_code_bugs(factsheet).await;
                                    Ok(()) as Result<(), Box<dyn std::error::Error>>
                                },
                                "Fixing code bugs",
                            )
                            .await?;
                        println!("✅ Code bugs fixed!");
                    }
                    self.attributes.state = AgentState::UnitTesting;
                }
                AgentState::UnitTesting => {
                    // Guard insures AI safety
                    PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Backend Code Unit Testing: Insured code is AI");
                    let is_safe_code: bool = confirm_safe_code();
                    if !is_safe_code {
                        panic!("❌ Unsafe code detected! Aborting unit tests.");
                    }
loader
                    
                        .execute_with_loading(
                            async {
                                // Add any unit testing logic here if needed
                                PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Building project...");
                                let build_backend_server: std::process::Output = Command::new("cargo")
                                    .arg("build")
                                    .current_dir(WEBSERVER_RUST_PATH)
                                    .stdout(Stdio::piped())
                                    .stderr(Stdio::piped())
                                    .output()
                                    .expect("Failed to build backend server");

                                if build_backend_server.status.success() {
                                    self.bug_count = 0;
                                    PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Test server build successful...");
                                } else {
                                    let update_cargo: std::process::Output = Command::new("cargo")
                                    .arg("update")
                                    .current_dir(WEBSERVER_RUST_PATH)
                                    .stdout(Stdio::piped())
                                    .stderr(Stdio::piped())
                                    .output()
                                    .expect("Failed to update cargo");
                                    if update_cargo.status.success() {
                                        PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Cargo updated successfully. Retrying build...");
                                    } else {
                                        PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Failed to update cargo. Aborting unit tests.");
                                        panic!("❌ Failed to update cargo. Aborting unit tests.");
                                    }

                                    let error_arr: Vec<u8> = build_backend_server.stderr;
                                    let error_str = String::from_utf8_lossy(&error_arr);

                                    // Update error stats
                                    self.bug_count += 1;
                                    self.bug_errors = Some(error_str.to_string());

                                    if self.bug_count > 2 {
                                        PrintCommand::Issue.print_agent_msg(self.attributes.position.as_str(), "Too many bugs found in code.");
                                        panic!("❌ Too many bugs found in code. Aborting unit tests.");
                                    }
                                    self.attributes.state = AgentState::Working;
                                }

                                Ok(()) as Result<(), Box<dyn std::error::Error>>
                            },
                            "Running unit tests",
                        )
                        .await?;
                    if self.attributes.state == AgentState::Working {
                        continue;
                    };

                    // Extract REST API endpoints
                    let api_endpoints_str: String = self.call_extract_rest_api_endpoints().await;
                    let api_endpoints: Vec<RouteObject> =
                        serde_json::from_str(api_endpoints_str.as_str())
                            .expect("Failed to decode API endpoints");

                    let check_endpoint: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| { 
                            route_object.method == "get" && route_object.is_route_dynamic == "false" 
                        })
                        .cloned()
                        .collect();

                    factsheet.api_endpoint_schema = Some(check_endpoint.clone());
                    
                    // RUn Backend app
                    PrintCommand::UnitTest.print_agent_msg(self.attributes.position.as_str(), "Starting webserver...");
                    let mut run_backend_server: std::process::Child = Command::new("cargo")
                        .arg("run")
                        .current_dir(WEBSERVER_RUST_PATH)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to run backend server");

                    PrintCommand::Issue.print_agent_msg(self.attributes.position.as_str(), "Launching tests on server in 5 secs");

                    let second_sleep: Duration = Duration::from_secs(5);
                    time::sleep(second_sleep).await;

                    // Check status code
                    for endpoint in check_endpoint {
                        let testing_msg: String = format!(
                            "Testing endpoint: {} with method: {}",
                            endpoint.route, endpoint.method
                        );
                        PrintCommand::UnitTest.print_agent_msg(
                            self.attributes.position.as_str(), 
                            testing_msg.as_str()
                        );

                        //create client
                        let client: Client = Client::builder()
                            .timeout(Duration::from_secs(5))
                            .build()
                            .unwrap();

                        // test url
                        let url: String = format!(
                            "http://localhost:8080{}",
                            endpoint.route
                        );
                        match check_status_code(&client, &url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    let err_msg = format!(
                                        "❌ Endpoint {} returned status code: {}",
                                        endpoint.route, status_code
                                    );
                                    PrintCommand::Issue.print_agent_msg(
                                        self.attributes.position.as_str(),
                                        err_msg.as_str(),
                                    );
                                } 
                            }
                            Err(e) => {
                                let err_msg = format!("Error checking backend endpoint: {}", e);
                                PrintCommand::Issue.print_agent_msg(
                                    self.attributes.position.as_str(),
                                    err_msg.as_str(),
                                );
                            }
                        }

                    }
                    run_backend_server.kill().expect("Failed to kill backend server");
                    println!("✅ Unit tests completed!");
                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_backend_developer() {
        let mut agent: AgentBackendDeveloper = AgentBackendDeveloper::new();

        let factsheet_str: &str = r#"
      {
        "project_description": "build a website which return the current time.",
        "project_scope": {
          "is_crud_required": false,
          "is_user_login_and_logout": false,
          "is_external_urls_required": false
        },
        "external_urls": [
          "http://worldtimeapi.org/api/timezone"
        ],
        "backend_code": null,
        "api_endpoint_schema": null
      }"#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent.attributes.state = AgentState::UnitTesting;
        agent
            .execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer agent");
    }
}
