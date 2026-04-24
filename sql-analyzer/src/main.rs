use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::fs;
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sql-analyzer")]
#[command(about = "Generate SQL queries from natural language using Claude", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive mode - define schema and ask questions
    Interactive,
    /// Generate a single SQL query
    Query {
        /// Natural language query description
        #[arg(short, long)]
        question: String,
        
        /// Path to schema file (JSON format)
        #[arg(short, long)]
        schema: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interactive) => run_interactive_mode(),
        Some(Commands::Query { question, schema }) => {
            run_single_query(&question, schema);
        }
        None => {
            // Default to interactive mode
            run_interactive_mode();
        }
    }
}

fn run_interactive_mode() {
    println!("🔍 SQL Query Generator - Interactive Mode");
    println!("===========================================\n");
    
    // Get schema information
    let schema = collect_schema_info();
    
    println!("\n✓ Schema loaded successfully!");
    println!("\nYou can now ask questions in natural language.");
    println!("Examples:");
    println!("  - Show me all sales per city");
    println!("  - Get the top 10 customers by total purchase amount");
    println!("  - List products that are out of stock");
    println!("\nType 'exit' or 'quit' to end the session.\n");

    loop {
        // Display prompt
        print_colored("You", Color::Green);
        print!(": ");
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
            println!("\n👋 Goodbye!");
            break;
        }

        // Generate SQL query
        print!("\n");
        print_colored("Generating SQL", Color::Cyan);
        println!("...\n");

        match generate_sql_query(input, &schema) {
            Ok(response) => {
                print_colored("SQL Query", Color::Blue);
                println!(":");
                println!("{}", format_sql_response(&response));
            }
            Err(e) => {
                print_colored("Error", Color::Red);
                println!(": {}", e);
                eprintln!("Make sure the 'claude' command is installed and accessible in your PATH.");
            }
        }
        println!();
    }
}

fn run_single_query(question: &str, schema_file: Option<String>) {
    let schema = if let Some(path) = schema_file {
        match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading schema file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Ask for schema interactively
        collect_schema_info()
    };

    match generate_sql_query(question, &schema) {
        Ok(response) => {
            println!("{}", format_sql_response(&response));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn collect_schema_info() -> String {
    println!("Please provide your database schema information.");
    println!("You can either:");
    println!("  1. Describe tables manually");
    println!("  2. Paste a schema definition");
    println!("\nEnter your schema (press Ctrl+D or type 'END' on a new line when done):\n");

    let mut schema = String::new();
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break, // EOF (Ctrl+D)
            Ok(_) => {
                if line.trim() == "END" {
                    break;
                }
                schema.push_str(&line);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    if schema.trim().is_empty() {
        // Provide example schema
        println!("\nNo schema provided. Using example schema:");
        schema = get_example_schema();
        println!("{}", schema);
    }

    schema
}

fn get_example_schema() -> String {
    r#"
Tables:
- customers (id, name, email, city, country, created_at)
- products (id, name, description, price, stock_quantity, category)
- orders (id, customer_id, order_date, total_amount, status)
- order_items (id, order_id, product_id, quantity, unit_price)
"#.to_string()
}

fn generate_sql_query(question: &str, schema: &str) -> Result<String, String> {
    let prompt = format!(
        r#"You are a SQL expert. Given the following database schema, generate a SQL query for the user's request.

DATABASE SCHEMA:
{}

USER REQUEST: {}

Please provide ONLY the SQL query without any explanations. Make sure the query is valid and follows best practices.
Format the query nicely with proper indentation."#,
        schema, question
    );

    // Call the claude CLI command
    let output = Command::new("claude")
        .arg(&prompt)
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

fn format_sql_response(response: &str) -> String {
    // Extract SQL query from response if it contains explanations
    let response = response.trim();
    
    // If response contains code blocks, extract them
    if let Some(start) = response.find("```sql") {
        if let Some(end) = response[start..].find("```") {
            let sql_start = start + 6; // length of "```sql"
            let sql_end = start + end;
            return response[sql_start..sql_end].trim().to_string();
        }
    } else if let Some(start) = response.find("```") {
        if let Some(end) = response[start+3..].find("```") {
            let sql_start = start + 3;
            let sql_end = start + 3 + end;
            return response[sql_start..sql_end].trim().to_string();
        }
    }
    
    response.to_string()
}

fn print_colored(text: &str, color: Color) {
    io::stdout()
        .execute(SetForegroundColor(color))
        .unwrap();
    print!("{}", text);
    io::stdout().execute(ResetColor).unwrap();
    io::stdout().flush().unwrap();
}
