use clap::Parser;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::fs;

/// OCR Extract - Extract text from PDF files using Claude AI
#[derive(Parser, Debug)]
#[command(name = "ocr-extract")]
#[command(about = "Extract text from PDF files using Claude AI's OCR capabilities", long_about = None)]
struct Args {
    /// Path to the PDF file to process
    #[arg(short, long, value_name = "FILE")]
    pdf: PathBuf,

    /// Custom prompt to send along with the PDF (optional)
    #[arg(short = 'm', long, default_value = "Please perform OCR on this PDF document and extract all the text content. Return only the extracted text without any additional commentary.")]
    prompt: String,

    /// Output file path (optional, prints to stdout if not specified)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Check if Claude CLI is available
    if !is_claude_available() {
        eprintln!("Error: 'claude' CLI is not installed or not in your PATH");
        eprintln!("\nTo install Claude CLI, visit:");
        eprintln!("  https://github.com/anthropics/anthropic-cli");
        eprintln!("\nOr install via pip:");
        eprintln!("  pip install anthropic-cli");
        std::process::exit(1);
    }

    // Validate PDF file exists
    if !args.pdf.exists() {
        eprintln!("Error: PDF file '{}' does not exist", args.pdf.display());
        std::process::exit(1);
    }

    if !args.pdf.is_file() {
        eprintln!("Error: '{}' is not a file", args.pdf.display());
        std::process::exit(1);
    }

    println!("Processing PDF: {}", args.pdf.display());
    println!("Sending to Claude for OCR...\n");

    // Call Claude CLI with the PDF file
    match call_claude_with_pdf(&args.pdf, &args.prompt) {
        Ok(extracted_text) => {
            let text = extracted_text.trim();
            
            // Output the result
            if let Some(output_path) = args.output {
                match fs::write(&output_path, text) {
                    Ok(_) => {
                        println!("✓ Text extracted successfully!");
                        println!("✓ Saved to: {}", output_path.display());
                    }
                    Err(e) => {
                        eprintln!("Error writing to output file: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("--- Extracted Text ---\n");
                println!("{}", text);
                println!("\n--- End of Extracted Text ---");
            }
        }
        Err(e) => {
            eprintln!("Error processing PDF: {}", e);
            eprintln!("\nMake sure:");
            eprintln!("  1. The 'claude' CLI is installed and in your PATH");
            eprintln!("  2. You have a valid Claude API key configured");
            eprintln!("  3. The PDF file is not corrupted");
            std::process::exit(1);
        }
    }
}

fn is_claude_available() -> bool {
    Command::new("claude")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn call_claude_with_pdf(pdf_path: &PathBuf, prompt: &str) -> Result<String, String> {
    // Try calling Claude CLI with file attachment
    // The exact syntax may vary depending on your Claude CLI version
    // Common patterns: claude -f file.pdf "prompt" or claude --file file.pdf "prompt"
    
    let pdf_str = pdf_path
        .to_str()
        .ok_or_else(|| "Invalid PDF path".to_string())?;

    // Try the -f flag first (most common)
    let output = Command::new("claude")
        .arg("-f")
        .arg(pdf_str)
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
        
        // If -f flag failed, try alternative syntax
        if error.contains("unknown option") || error.contains("unrecognized option") {
            // Try --file flag
            let output_alt = Command::new("claude")
                .arg("--file")
                .arg(pdf_str)
                .arg(prompt)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("Failed to execute claude command: {}", e))?;

            if output_alt.status.success() {
                let response = String::from_utf8_lossy(&output_alt.stdout).to_string();
                return Ok(response);
            }
        }
        
        Err(format!("Claude CLI error: {}", error))
    }
}
