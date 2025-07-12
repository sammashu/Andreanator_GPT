use ai_functions::ai_function;

#[ai_function]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// IMPORTANT: The backend code is ONLY an example. If the Project Description requires it, make as many changes as you like.
    /// IMPORTANT: You do not need to follow the backend code exactly. Write functions that make sense for the users request if required.
    /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
    /// IMPORTANT: The following libraries are already installed
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// No other external libraries should be used. Write functions that fit with the description from the PROJECT_DESCRIPTION
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// FUNCTION: Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality
    ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
    ///   3. ONLY writes the code. No commentary.
    /// IMPORTANT: The following libraries are already installed. Does not use ANY libraries other than what was provided in the template
    ///   reqwest, serde, serde_json, tokio, actix-web, async-trait
    println!(OUTPUT)
}

#[ai_function]
pub fn print_fixed_code(_broken_code_with_bugs: &str) {
    /// INPUT: Takes in Rust BROKEN_CODE and the ERROR_BUGS found
    /// FUNCTION: Removes bugs from code
    /// IMPORTANT: Only prints out the new and improved code. No commentary or anything else
    println!(OUTPUT)
}

#[ai_function]
pub fn print_rest_api_endpoints(_code_input: &str) {
    /// INPUT: Takes in Rust webserver CODE_INPUT based on actix-web
    /// FUNCTION: Prints out the JSON schema for url endpoints and their respective types
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///   "route": This represents the url path of the endpoint
    ///   "is_route_dynamic": if a route has curly braces in it such as {symbol} or {id} as an example, then this will be set to true
    ///   "method": This represents the method being called
    ///   "request_body": This represents the body of a post method request
    ///   "response": This represents the output based upon the structs in the code and understanding the functions
    /// IMPORTANT: Only prints out the JSON schema. No commentary or anything else.
    /// MUST READ: All keys are strings. Even bool should be wrapped in double quotes as "bool"
    /// EXAMPLE:
    /// INPUT_CODE:
    /// ...
    /// pub struct Item {
    ///   pub id: u64,
    ///   pub name: String,
    ///   pub completed: bool,
    /// }
    /// pub struct User {
    ///   pub id: u64,
    ///   pub username: String,
    ///   pub password: String,
    /// }
    /// ...
    /// HttpServer::new(move || {
    ///   App::new()
    ///       .app_data(data.clone())
    ///       .route("/item", web::post().to(create_item))
    ///       .route("/item/{id}", web::get().to(read_item))
    ///       .route("/item/{id}", web::put().to(update_item))
    ///       .route("/item/{id}", web::delete().to(delete_item))
    ///       .route("/signup", web::post().to(signup))
    ///       .route("/crypto", web::get().to(crypto))
    /// PRINTS JSON FORMATTED OUTPUT:
    /// [
    ///   {
    ///     "route": "/item/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "get"
    ///     "request_body": "None",
    ///     "response": {
    ///       "id": "number",
    ///       "name": "string",
    ///       "completed": "bool",
    ///     }
    ///   },
    ///   {
    ///     "route": "/item",
    ///     "is_route_dynamic": "false",
    ///     "method": "post",
    ///     "request_body": {
    ///       "id": "number",
    ///       "name": "string",
    ///       "completed": "bool",
    ///     },
    ///     "response": "None"
    ///   },
    ///   {
    ///     "route": "/item/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "delete",
    ///     "request_body": "None",
    ///     "response": "None"
    ///   },
    ///   {
    ///     "route": "/crypto",
    ///     "is_route_dynamic": "false",
    ///     "method": "get",
    ///     "request_body": "None",
    ///     "response": "not_provided"
    ///   },
    ///   ... // etc
    /// ]
    println!(OUTPUT)
}

////////////////// SPRING BOOT JAVA
///
#[ai_function]
pub fn print_java_springboot_webapi_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a Java Spring Boot Web API build
    /// IMPORTANT: The backend code is ONLY an example. If the Project Description requires it, make as many changes as you like.
    /// IMPORTANT: You do not need to follow the backend code exactly. Write classes, methods, and configurations that make sense for the user's request if required.
    /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in the PROJECT_DESCRIPTION
    /// IMPORTANT: The following dependencies are already included in the project:
    ///   spring-boot-starter-web      // For building REST APIs
    ///   spring-boot-starter-data-jpa // For database access (optional, if needed)
    ///   spring-boot-starter-security // For security (optional, if needed)
    ///   spring-boot-starter-json     // For JSON parsing and serialization
    ///   lombok                       // For reducing boilerplate code
    ///   h2                           // For in-memory database (optional, if needed)
    /// No other external dependencies should be used. Write code that fits with the description from the PROJECT_DESCRIPTION
    /// OUTPUT: Print ONLY the Java code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_springboot_webapi_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a Java Spring Boot Web API build
    /// FUNCTION: Performs the following tasks:
    ///   1. Removes any bugs in the code and adds minor additional functionality
    ///   2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No code should be implemented later. Everything should be written now.
    ///   3. ONLY writes the code. No commentary.
    /// IMPORTANT: The following dependencies are already included in the project. Do not use ANY libraries other than what was provided in the template:
    ///   spring-boot-starter-web      // For building REST APIs
    ///   spring-boot-starter-data-jpa // For database access (optional, if needed)
    ///   spring-boot-starter-security // For security (optional, if needed)
    ///   spring-boot-starter-json     // For JSON parsing and serialization
    ///   lombok                       // For reducing boilerplate code
    ///   h2                           // For in-memory database (optional, if needed)
    println!(OUTPUT)
}

#[ai_function]
pub fn print_springboot_rest_api_endpoints(_code_input: &str) {
    /// INPUT: Takes in Java Spring Boot REST controller CODE_INPUT
    /// FUNCTION: Prints out the JSON schema for url endpoints and their respective types
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///   "route": This represents the url path of the endpoint
    ///   "is_route_dynamic": if a route has curly braces in it such as {id} or {username}, then this will be set to "true"
    ///   "method": This represents the HTTP method being called (get, post, put, delete, etc.)
    ///   "request_body": This represents the body of a POST/PUT method request, based on the DTO or entity class used
    ///   "response": This represents the output based upon the response type in the controller method
    /// IMPORTANT: Only prints out the JSON schema. No commentary or anything else.
    /// MUST READ: All keys are strings. Even bool should be wrapped in double quotes as "bool"
    /// EXAMPLE:
    /// INPUT_CODE:
    /// ...
    /// @RestController
    /// @RequestMapping("/users")
    /// public class UserController {
    ///     @PostMapping
    ///     public User createUser(@RequestBody User user) { ... }
    ///
    ///     @GetMapping("/{id}")
    ///     public User getUser(@PathVariable Long id) { ... }
    ///
    ///     @PutMapping("/{id}")
    ///     public User updateUser(@PathVariable Long id, @RequestBody User user) { ... }
    ///
    ///     @DeleteMapping("/{id}")
    ///     public void deleteUser(@PathVariable Long id) { ... }
    /// }
    ///
    /// public class User {
    ///     private Long id;
    ///     private String username;
    ///     private String email;
    ///     private Boolean active;
    ///     // getters and setters
    /// }
    /// PRINTS JSON FORMATTED OUTPUT:
    /// [
    ///   {
    ///     "route": "/users",
    ///     "is_route_dynamic": "false",
    ///     "method": "post",
    ///     "request_body": {
    ///       "id": "number",
    ///       "username": "string",
    ///       "email": "string",
    ///       "active": "bool"
    ///     },
    ///     "response": {
    ///       "id": "number",
    ///       "username": "string",
    ///       "email": "string",
    ///       "active": "bool"
    ///     }
    ///   },
    ///   {
    ///     "route": "/users/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "get",
    ///     "request_body": "None",
    ///     "response": {
    ///       "id": "number",
    ///       "username": "string",
    ///       "email": "string",
    ///       "active": "bool"
    ///     }
    ///   },
    ///   {
    ///     "route": "/users/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "put",
    ///     "request_body": {
    ///       "id": "number",
    ///       "username": "string",
    ///       "email": "string",
    ///       "active": "bool"
    ///     },
    ///     "response": {
    ///       "id": "number",
    ///       "username": "string",
    ///       "email": "string",
    ///       "active": "bool"
    ///     }
    ///   },
    ///   {
    ///     "route": "/users/{id}",
    ///     "is_route_dynamic": "true",
    ///     "method": "delete",
    ///     "request_body": "None",
    ///     "response": "None"
    ///   }
    /// ]
    println!(OUTPUT)
}
