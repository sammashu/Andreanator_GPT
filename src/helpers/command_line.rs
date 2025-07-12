use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};

use std::io::{Stdout, Write, stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_msg(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: Stdout = stdout();

        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent's position and statement in the specified color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);
        //Reset the color to default
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

// get user request input for the prompt
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = std::io::stdout();

    //Print the question in a specific color this one is blue
    stdout.execute(SetForegroundColor(Color::Green)).unwrap();
    println!("");
    println!("{}: ", question);

    // Reset the color to default
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read user response");

    // Trim the input to remove any trailing newline characters
    return user_response.trim().to_string();
}

pub fn display_andreanator_logo() {
    // ANSI color codes
    let reset: &'static str = "\x1b[0m";
    let red: &'static str = "\x1b[31m";
    let yellow: &'static str = "\x1b[33m";
    let blue: &'static str = "\x1b[34m";
    let cyan: &'static str = "\x1b[36m";
    let bold: &'static str = "\x1b[1m";
    let white: &'static str = "\x1b[97m";

    let logo: String = format!(
        r#"{bold}{yellow}                 Andreanator is back!{reset}

                       ______
                     <((((((\\\
                     /      . }}\
                     ;--..--._|}}
  (\                 '--/\--'  )
   \\                | '-'  :'{red}O{reset}{white}|{reset}
    \\               . -==- .-|
     \\               \.__.'   \--._
     [\\          __.--|       //  _/'--.
     \ \\       .'-._ ('-----'/ __/      \
      \ \\     /   __>|      | '--.       |
       \ \\   |   \   |     /    /       /
        \ '\ /     \  |     |  _/       /
         \  \       \ |     | /        /
          \  \      \        /
             {yellow}*{reset}{cyan}⚡{reset}{yellow}*{reset}{cyan}⚡{reset}{yellow}*{reset}

               {blue}[SYSTEM NATOR ACTIVATE]{reset}
"#,
        bold = bold,
        yellow = yellow,
        red = red,
        blue = blue,
        cyan = cyan,
        white = white,
        reset = reset
    );

    println!("{}", logo);
}

pub fn languages_options() -> String {
    loop {
        println!("\nPlease select a language for the webserver:");
        println!("1. Rust");
        println!("2. Java (Spring Boot)");
        println!("Enter your choice:");

        stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read choice");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("You selected Rust.");
                return "Rust".to_string();
            }
            "2" => {
                println!("You selected Java (Spring Boot).");
                return "Java".to_string();
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

// Get user response that code is safe to run
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = std::io::stdout();

    loop {

        // {Print question in a specific color this one in Blue}
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        println!("WARNING: you are about to run code written entirely by AI. ");
        println!("Review your code and confirm you wish to continue. ");

        stdout.execute(ResetColor).unwrap();

        // Present Option with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Let Stop this project");

        stdout.execute(ResetColor).unwrap();

        let mut human_response = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read human response");

        // Trim the input to remove any trailing newline characters
        let human_response: String = human_response.trim().to_string();

        match human_response.as_str() {
            "1" | "ok" | "y" => {
                return true;
            },
            "2" | "no" | "n" => {
                return false;
            },
            _=> {
                println!("Invalid input, please enter 1 or 2.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall.print_agent_msg("Managing Agent", "Testing , processing");
    }
}
