use std::io::{self, Write};
use std::process::{Command, Stdio};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

fn main() {
    println!("Claude Chat CLI - Interactive Mode");
    println!("Type your messages and press Enter to send.");
    println!("Type 'exit' or 'quit' to end the session.\n");

    let mut conversation_history = Vec::new();

    loop {
        // Display prompt
        print!("\n");
        io::stdout()
            .execute(SetForegroundColor(Color::Green))
            .unwrap();
        print!("You: ");
        io::stdout().execute(ResetColor).unwrap();
        io::stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Check for exit commands
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("\nGoodbye!");
            break;
        }

        // Add to conversation history
        conversation_history.push(input.to_string());

        // Call Claude CLI
        io::stdout()
            .execute(SetForegroundColor(Color::Blue))
            .unwrap();
        print!("\nClaude: ");
        io::stdout().execute(ResetColor).unwrap();
        io::stdout().flush().unwrap();

        match call_claude(input) {
            Ok(response) => {
                println!("{}", response.trim());
            }
            Err(e) => {
                io::stdout()
                    .execute(SetForegroundColor(Color::Red))
                    .unwrap();
                eprintln!("\nError calling Claude CLI: {}", e);
                eprintln!("Make sure the 'claude' command is installed and accessible in your PATH.");
                io::stdout().execute(ResetColor).unwrap();
            }
        }
    }
}

fn call_claude(prompt: &str) -> Result<String, String> {
    // Call the claude CLI command
    let output = Command::new("claude")
        .arg(prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute claude command: {}", e))?;

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(response)
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Claude CLI error: {}", error))
    }
}
