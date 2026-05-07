# OCR Extract

A Rust command-line tool that extracts text from PDF files using Claude AI's OCR capabilities.

## Features

- 📄 Extract text from PDF documents
- 🤖 Powered by Claude AI's vision and OCR capabilities
- 💾 Save output to file or display in terminal
- 🎯 Custom prompts for specific extraction needs
- ⚡ Simple and fast command-line interface

## Prerequisites

- Rust (1.70 or later)
- Claude CLI installed and accessible in your PATH
- Valid Claude API key configured

## Installation

1. Navigate to this directory:
```bash
cd ocr-extract
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage

Extract text from a PDF and display it in the terminal:
```bash
cargo run -- --pdf document.pdf
```

Or using the compiled binary:
```bash
./target/release/ocr-extract --pdf document.pdf
```

### Save to File

Extract text and save it to a file:
```bash
cargo run -- --pdf document.pdf --output extracted_text.txt
```

### Custom Prompt

Use a custom prompt for specific extraction needs:
```bash
cargo run -- --pdf document.pdf --prompt "Extract only the table data from this PDF"
```

### Short Flags

You can use short flags for convenience:
```bash
cargo run -- -p document.pdf -o output.txt
```

## Command-Line Options

- `-p, --pdf <FILE>`: Path to the PDF file to process (required)
- `-o, --output <FILE>`: Output file path (optional, prints to stdout if not specified)
- `-m, --prompt <TEXT>`: Custom prompt to send along with the PDF (optional)

## Examples

```bash
# Extract text from a receipt
cargo run -- -p receipt.pdf -m "Extract all text from this receipt" -o receipt.txt

# Extract text from a scanned document
cargo run -- -p scanned_doc.pdf

# Extract specific information
cargo run -- -p invoice.pdf -m "Extract the invoice number, date, and total amount"
```

## How It Works

1. The application takes a PDF file path as input
2. It sends the PDF to Claude CLI with an OCR prompt
3. Claude processes the PDF using its vision capabilities to extract text
4. The extracted text is either displayed or saved to a file

## Troubleshooting

If you encounter errors:

1. **Claude CLI not found**: Make sure the `claude` command is installed and in your PATH
2. **API key issues**: Ensure your Claude API key is properly configured
3. **PDF not supported**: Some PDFs might be encrypted or corrupted - try with a different file

## Notes

- The Claude CLI command syntax may vary depending on your version. The application tries multiple common patterns.
- Large PDF files may take longer to process.
- The quality of OCR depends on the clarity of the PDF document.
