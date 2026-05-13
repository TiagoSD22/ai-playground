# Image Identifier

A Rust CLI application that uses Claude CLI to analyze and describe images.

## Description

This application receives an image path from the user and sends it to Claude CLI with a prompt asking to identify and describe all elements in the image. The response is returned as plain text with detailed information about the image contents.

## Prerequisites

- Rust and Cargo installed
- Claude CLI installed and accessible in your PATH
- Valid Claude API credentials configured

## Installation

Build the project:

```bash
cd image-identifier
cargo build --release
```

## Usage

### Option 1: Pass image path as command-line argument

```bash
cargo run -- /path/to/your/image.jpg
```

Or if you've built the release version:

```bash
./target/release/image-identifier /path/to/your/image.jpg
```

### Option 2: Interactive mode

Run without arguments and the application will prompt you for the image path:

```bash
cargo run
```

Then enter the path when prompted:

```
Enter the path to the image file: /path/to/your/image.jpg
```

## Features

- Accepts image path via command-line argument or interactive prompt
- Validates that the image file exists before processing
- Uses Claude CLI to analyze image contents
- Provides detailed descriptions of:
  - Objects and elements in the image
  - People (if present)
  - Text content
  - Colors and composition
  - Other relevant visual details
- Colored terminal output for better readability
- Clear error messages

## Supported Image Formats

The application supports any image format that Claude CLI accepts, typically including:
- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)

## Example Output

```
Image Identifier - Claude CLI Image Analysis

Analyzing image: photo.jpg

Claude is analyzing the image...

Analysis Result:
This image shows a sunset over a beach. The sky displays vibrant orange and 
pink hues blending into deeper purple tones. In the foreground, there's a sandy 
beach with gentle waves lapping at the shore. A few silhouettes of people can 
be seen walking along the waterline. The composition creates a peaceful, serene 
atmosphere with the sun positioned in the upper third of the frame...
```

## Dependencies

- `crossterm` - For colored terminal output

## Error Handling

The application handles various error scenarios:
- Missing or invalid image file paths
- Claude CLI not found in PATH
- Claude CLI execution errors
- Invalid image formats

## Notes

- Make sure your Claude CLI is properly configured with API credentials
- Large images may take longer to analyze
- The quality of the description depends on the image content and Claude's vision capabilities
