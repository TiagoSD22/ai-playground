# Claude Chat CLI

A simple interactive CLI wrapper for the Claude command-line tool, written in Rust.

## Features

- 🚀 Interactive chat interface
- 🎨 Colored output for better readability
- 💬 Simple conversation flow
- ⌨️ Easy to use

## Prerequisites

- Rust (1.70 or later)
- Claude CLI installed and accessible in your PATH

## Installation

1. Clone or navigate to this directory:
```bash
cd ai-playground
```

2. Build the project:
```bash
cargo build --release
```

3. Run the CLI:
```bash
cargo run
```

Or run the compiled binary:
```bash
./target/release/claude-chat
```

## Usage

Once started, the CLI will enter an interactive mode:

```
Claude Chat CLI - Interactive Mode
Type your messages and press Enter to send.
Type 'exit' or 'quit' to end the session.

You: What is Rust?
Claude: [Response from Claude...]

You: Tell me more
Claude: [Response from Claude...]

You: exit
Goodbye!
```

## Commands

- Type any message and press Enter to send it to Claude
- Type `exit` or `quit` to end the session

## How It Works

The CLI:
1. Prompts you for input
2. Calls the `claude` CLI command with your input
3. Displays the response
4. Repeats until you exit

## Troubleshooting

If you see an error about the Claude CLI not being found:
- Ensure the `claude` command is installed
- Check that it's in your system PATH
- Try running `claude --version` in your terminal to verify

## Customization

You can modify `src/main.rs` to:
- Add conversation context/history
- Change colors or formatting
- Add additional CLI flags
- Implement streaming responses
- Save conversation history to file

## Building for Production

```bash
cargo build --release
```

The binary will be available at `target/release/claude-chat` (or `claude-chat.exe` on Windows).

## License

MIT
