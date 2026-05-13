use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

fn main() {
    println!("Image Identifier - Claude CLI Image Analysis\n");

    // Get image path from command line arguments or prompt user
    let image_path = get_image_path();

    // Validate that the file exists
    if !Path::new(&image_path).exists() {
        eprintln!("Error: Image file '{}' not found!", image_path);
        std::process::exit(1);
    }

    println!("Analyzing image: {}\n", image_path);

    // Display loading message
    io::stdout()
        .execute(SetForegroundColor(Color::Blue))
        .unwrap();
    println!("Claude is analyzing the image...\n");
    io::stdout().execute(ResetColor).unwrap();

    // Call Claude CLI with the image
    match analyze_image(&image_path) {
        Ok(description) => {
            io::stdout()
                .execute(SetForegroundColor(Color::Green))
                .unwrap();
            println!("Analysis Result:");
            io::stdout().execute(ResetColor).unwrap();
            println!("{}\n", description.trim());
        }
        Err(e) => {
            io::stdout()
                .execute(SetForegroundColor(Color::Red))
                .unwrap();
            eprintln!("Error: {}", e);
            eprintln!("Make sure the 'claude' command is installed and accessible in your PATH.");
            io::stdout().execute(ResetColor).unwrap();
            std::process::exit(1);
        }
    }
}

fn get_image_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Use command-line argument
        args[1].clone()
    } else {
        // Prompt user for image path
        print!("Enter the path to the image file: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    }
}

fn analyze_image(image_path: &str) -> Result<String, String> {
    // Create the prompt for Claude
    let prompt = "Please analyze this image and describe all the elements you can identify. \
                  Provide a detailed description of what you see, including objects, people, \
                  text, colors, composition, and any other relevant details. \
                  Format your response as plain text.";

    // Call the claude CLI command with the image
    // The claude CLI accepts images by passing the image path as an argument
    let output = Command::new("claude")
        .arg("--image")
        .arg(image_path)
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
