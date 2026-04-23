# GitHub Repository Analytics

A Rust CLI tool that analyzes GitHub repositories using Claude AI. Given a GitHub repository URL, it fetches repository information and generates a comprehensive analysis including what the project does, the technology stack used, and how to use it.

## Features

- 📊 Fetches repository metadata (stars, forks, language, topics)
- 📄 Retrieves and parses README content
- 📁 Analyzes repository structure
- 🤖 Uses Claude AI for intelligent analysis
- 🔒 Supports GitHub API tokens for higher rate limits

## Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **Claude CLI** - This tool requires the `claude` command to be installed and accessible in your PATH
3. **GitHub API Token** (optional but recommended) - For higher API rate limits

## Installation

### Build from source

```bash
cd repo-analitic
cargo build --release
```

The binary will be available at `target/release/repo-analitic` (or `repo-analitic.exe` on Windows).

### Add to PATH (optional)

**Linux/macOS:**
```bash
cargo install --path .
```

**Windows:**
```powershell
cargo install --path .
```

Or manually add the `target/release` directory to your PATH.

## Usage

### Basic usage

```bash
repo-analitic https://github.com/owner/repository
```

### With GitHub token (recommended)

```bash
repo-analitic https://github.com/owner/repository --token YOUR_GITHUB_TOKEN
```

Or set the token as an environment variable:

```bash
# Linux/macOS
export GITHUB_TOKEN=your_token_here

# Windows (PowerShell)
$env:GITHUB_TOKEN="your_token_here"

# Windows (CMD)
set GITHUB_TOKEN=your_token_here

# Then run without --token flag
repo-analitic https://github.com/owner/repository
```

### Examples

Analyze the Rust programming language repository:
```bash
repo-analitic https://github.com/rust-lang/rust
```

Analyze a Node.js project:
```bash
repo-analitic https://github.com/expressjs/express --token ghp_yourtoken
```

## Getting a GitHub Token

1. Go to [GitHub Settings > Developer Settings > Personal Access Tokens](https://github.com/settings/tokens)
2. Click "Generate new token" → "Generate new token (classic)"
3. Give it a name and select the `public_repo` scope
4. Click "Generate token" and copy the token
5. Use it with the `--token` flag or set it as an environment variable

## How It Works

1. **Parse GitHub URL** - Extracts owner and repository name
2. **Fetch Repository Data** - Uses GitHub API to get:
   - Repository metadata (description, language, stats)
   - README content (base64 decoded)
   - Repository structure (root-level files and folders)
3. **Build Analysis Prompt** - Combines all gathered information into a structured prompt
4. **Call Claude** - Sends the prompt to Claude CLI for analysis
5. **Display Results** - Shows Claude's analysis with information about:
   - What the project does
   - Technology stack used
   - How to use it

## Troubleshooting

### "claude command not found"
- Ensure the Claude CLI is installed and available in your system PATH
- Try running `claude --help` to verify installation

### GitHub API rate limit errors
- Use a GitHub token with the `--token` flag or `GITHUB_TOKEN` environment variable
- Anonymous requests are limited to 60 requests/hour
- Authenticated requests allow 5,000 requests/hour

### "Failed to parse GitHub URL"
- Ensure the URL is in the format: `https://github.com/owner/repo`
- Repository must be public (or you need appropriate permissions)

## Dependencies

- `reqwest` - HTTP client for GitHub API calls
- `serde` & `serde_json` - JSON serialization/deserialization
- `clap` - Command-line argument parsing
- `anyhow` - Error handling
- `base64` - Decoding README content

## License

MIT

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
