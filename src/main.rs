#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{ stringify!($func) }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::{display_andreanator_logo, get_user_response, languages_options};
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    display_andreanator_logo();
    let usr_req: String =
        get_user_response("Sup Dawg what webserver you want to ask Andreanator to build ?");
    let lang_opt: String = languages_options();
    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Failed to create managing agent");

    manage_agent.execute_project().await;

}
