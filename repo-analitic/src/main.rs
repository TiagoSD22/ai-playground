use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::process::{Command, Stdio};

/// GitHub Repository Analytics - Analyzes GitHub repositories using Claude AI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub repository URL (e.g., https://github.com/owner/repo)
    #[arg(value_name = "URL")]
    github_url: String,

    /// GitHub API token (optional, for higher rate limits)
    #[arg(short, long, env = "GITHUB_TOKEN")]
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    description: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    topics: Vec<String>,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct Content {
    name: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Deserialize)]
struct ReadmeResponse {
    content: String,
    encoding: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🔍 Analyzing GitHub repository...\n");

    // Parse GitHub URL
    let (owner, repo) = parse_github_url(&args.github_url)
        .context("Failed to parse GitHub URL. Expected format: https://github.com/owner/repo")?;

    println!("📦 Repository: {}/{}", owner, repo);

    // Fetch repository data
    let client = reqwest::blocking::Client::builder()
        .user_agent("repo-analitic")
        .build()?;

    let repo_info = fetch_repository_info(&client, &owner, &repo, args.token.as_deref())?;
    let readme = fetch_readme(&client, &owner, &repo, args.token.as_deref())?;
    let contents = fetch_repository_contents(&client, &owner, &repo, args.token.as_deref())?;

    // Build prompt for Claude
    let prompt = build_analysis_prompt(&repo_info, &readme, &contents);

    println!("\n🤖 Sending to Claude for analysis...\n");

    // Call Claude CLI
    let analysis = call_claude(&prompt)?;

    // Display results
    println!("═══════════════════════════════════════════════════════════");
    println!("📊 REPOSITORY ANALYSIS");
    println!("═══════════════════════════════════════════════════════════\n");
    println!("{}", analysis.trim());
    println!("\n═══════════════════════════════════════════════════════════");

    Ok(())
}

fn parse_github_url(url: &str) -> Result<(String, String)> {
    let url = url.trim_end_matches('/');
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() < 2 {
        anyhow::bail!("Invalid GitHub URL format");
    }

    let owner = parts[parts.len() - 2].to_string();
    let repo = parts[parts.len() - 1].to_string();

    Ok((owner, repo))
}

fn fetch_repository_info(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo: &str,
    token: Option<&str>,
) -> Result<Repository> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    let mut request = client.get(&url);

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send().context("Failed to fetch repository info")?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API error: {}", response.status());
    }

    let repo_info: Repository = response.json().context("Failed to parse repository data")?;
    Ok(repo_info)
}

fn fetch_readme(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo: &str,
    token: Option<&str>,
) -> Result<String> {
    let url = format!("https://api.github.com/repos/{}/{}/readme", owner, repo);
    let mut request = client.get(&url);

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            let readme_data: ReadmeResponse = resp.json()?;
            if readme_data.encoding == "base64" {
                let decoded = base64_decode(&readme_data.content)?;
                Ok(decoded)
            } else {
                Ok(readme_data.content)
            }
        }
        _ => Ok(String::from("No README found")),
    }
}

fn fetch_repository_contents(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo: &str,
    token: Option<&str>,
) -> Result<Vec<Content>> {
    let url = format!("https://api.github.com/repos/{}/{}/contents", owner, repo);
    let mut request = client.get(&url);

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send().context("Failed to fetch repository contents")?;

    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let contents: Vec<Content> = response.json().context("Failed to parse contents")?;
    Ok(contents)
}

fn base64_decode(encoded: &str) -> Result<String> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    
    // Remove whitespace and newlines
    let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    
    // Decode base64
    match STANDARD.decode(&cleaned) {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
        Err(_) => Ok(cleaned),
    }
}

fn build_analysis_prompt(repo_info: &Repository, readme: &str, contents: &Vec<Content>) -> String {
    let mut prompt = String::new();

    prompt.push_str("Please analyze this GitHub repository and provide a brief description including:\n");
    prompt.push_str("1. What the project does\n");
    prompt.push_str("2. Technology stack used\n");
    prompt.push_str("3. How to use it\n\n");

    prompt.push_str("Repository Information:\n");
    prompt.push_str(&format!("- Name: {}\n", repo_info.full_name));
    if let Some(desc) = &repo_info.description {
        prompt.push_str(&format!("- Description: {}\n", desc));
    }
    if let Some(lang) = &repo_info.language {
        prompt.push_str(&format!("- Primary Language: {}\n", lang));
    }
    prompt.push_str(&format!("- Stars: {}\n", repo_info.stargazers_count));
    prompt.push_str(&format!("- Forks: {}\n", repo_info.forks_count));
    if !repo_info.topics.is_empty() {
        prompt.push_str(&format!("- Topics: {}\n", repo_info.topics.join(", ")));
    }

    prompt.push_str("\nRepository Structure:\n");
    for content in contents {
        let icon = if content.content_type == "dir" {
            "📁"
        } else {
            "📄"
        };
        prompt.push_str(&format!("{} {}\n", icon, content.name));
    }

    prompt.push_str("\nREADME Content:\n");
    prompt.push_str("---\n");
    // Limit README to first 3000 characters to avoid token limits
    let readme_preview = if readme.len() > 3000 {
        &readme[..3000]
    } else {
        readme
    };
    prompt.push_str(readme_preview);
    if readme.len() > 3000 {
        prompt.push_str("\n\n[README truncated for length...]");
    }
    prompt.push_str("\n---\n");

    prompt
}

fn call_claude(prompt: &str) -> Result<String> {
    let output = Command::new("claude")
        .arg(prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute claude command. Make sure 'claude' CLI is installed.")?;

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(response)
    } else {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        anyhow::bail!("Claude CLI error: {}", error)
    }
}
