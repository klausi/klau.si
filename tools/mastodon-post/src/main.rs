use anyhow::{Context, Result, bail};
use clap::Parser;
use megalodon::generator;
use megalodon::megalodon::PostStatusInputOptions;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Posts blog entries to Mastodon for comment functionality.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specific blog post URL to create a comment section for
    #[arg(short = 'u', long = "url")]
    url: Option<String>,

    /// Dry run - don't actually post, just show what would be done
    #[arg(short = 'n', long = "dry-run")]
    dry_run: bool,
}

/// Configuration loaded from .env file
struct Config {
    mastodon_base_url: String,
    mastodon_access_token: String,
}

/// Represents a blog post
#[derive(Debug)]
struct BlogPost {
    /// The file path to the blog post markdown file
    file_path: PathBuf,
    /// The URL of the blog post on the website
    url: String,
    /// The title of the blog post
    title: String,
    /// The date of the blog post (for sorting)
    date: String,
    /// Whether the blog post already has a Mastodon comment section
    has_mastodon_section: bool,
    /// The Mastodon post ID if already posted
    mastodon_id: Option<String>,
}

impl Config {
    fn load() -> Result<Self> {
        // Try to load .env file from the project root
        let project_root = find_project_root()?;
        let env_path = project_root.join(".env");

        if env_path.exists() {
            dotenvy::from_path(&env_path)?;
        }

        let mastodon_base_url = env::var("MASTODON_BASE_URL").map_err(|_| {
            anyhow::anyhow!(Self::missing_credentials_message())
        })?;

        let mastodon_access_token = env::var("MASTODON_ACCESS_TOKEN").map_err(|_| {
            anyhow::anyhow!(Self::missing_credentials_message())
        })?;

        Ok(Config {
            mastodon_base_url,
            mastodon_access_token,
        })
    }

    fn missing_credentials_message() -> String {
        r#"
Missing Mastodon credentials in .env file.

Please create a .env file in the project root with the following content:

MASTODON_BASE_URL=https://mastodon.social
MASTODON_ACCESS_TOKEN=your_access_token_here

To get an access token:
1. Log in to your Mastodon account
2. Go to Settings -> Development -> New Application
3. Give it a name (e.g., "Blog Comments")
4. Select scopes: read, write
5. Click "Submit"
6. Copy the "Your access token" value
"#
        .to_string()
    }
}

/// Find the project root by looking for config.toml
fn find_project_root() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()?;

    loop {
        if current_dir.join("config.toml").exists() {
            return Ok(current_dir);
        }
        if !current_dir.pop() {
            bail!("Could not find project root (no config.toml found in parent directories)");
        }
    }
}

/// Scan the content/blog directory for blog posts
fn scan_blog_posts(project_root: &Path) -> Result<Vec<BlogPost>> {
    let blog_dir = project_root.join("content/blog");
    let mut posts = Vec::new();

    // Regex to extract front matter
    let front_matter_re = Regex::new(r"(?s)^\+\+\+\n(.+?)\n\+\+\+")?;
    let title_re = Regex::new(r#"title\s*=\s*"([^"]+)""#)?;
    let date_re = Regex::new(r"date\s*=\s*(\d{4}-\d{2}-\d{2})")?;
    let mastodon_re = Regex::new(r"\{\{\s*mastodon_comments\s*\(")?;
    let mastodon_id_re = Regex::new(r#"mastodon_comments\s*\(\s*id\s*=\s*"(\d+)""#)?;

    for entry in WalkDir::new(&blog_dir)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Check if it's a markdown file
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }

        // Skip _index.md files
        if path.file_name().is_some_and(|name| name == "_index.md") {
            continue;
        }

        let content = fs::read_to_string(path)?;

        // Extract front matter
        let front_matter = match front_matter_re.captures(&content) {
            Some(caps) => caps.get(1).map(|m| m.as_str()).unwrap_or(""),
            None => continue,
        };

        // Extract title
        let title = match title_re.captures(front_matter) {
            Some(caps) => caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
            None => continue,
        };

        // Extract date
        let date = match date_re.captures(front_matter) {
            Some(caps) => caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
            None => continue,
        };

        // Check if the post already has a Mastodon comment section
        let has_mastodon_section = mastodon_re.is_match(&content);

        // Extract Mastodon ID if present
        let mastodon_id = mastodon_id_re
            .captures(&content)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

        // Derive URL from file path
        let url = derive_blog_url(path, &blog_dir)?;

        posts.push(BlogPost {
            file_path: path.to_path_buf(),
            url,
            title,
            date,
            has_mastodon_section,
            mastodon_id,
        });
    }

    // Sort by date, newest first
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

/// Derive the blog post URL from the file path
fn derive_blog_url(file_path: &Path, blog_dir: &Path) -> Result<String> {
    let relative = file_path.strip_prefix(blog_dir)?;

    // Handle both formats:
    // - 2025-04-04-testing-claude-ai-for-drupal-code.md
    // - 2025-01-30-security-advisory-nextjs-redirects/index.md

    let url_path = if relative.file_name().is_some_and(|name| name == "index.md") {
        // It's in a folder, use the folder name
        relative
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    } else {
        // It's a standalone file, strip the .md extension
        relative
            .to_str()
            .unwrap_or("")
            .trim_end_matches(".md")
            .to_string()
    };

    // Convert date prefix to URL format
    // 2025-04-04-testing-claude-ai... -> testing-claude-ai...
    let url_slug = if url_path.len() > 11 && url_path.chars().take(10).all(|c| c.is_ascii_digit() || c == '-') {
        &url_path[11..]
    } else {
        &url_path
    };

    Ok(format!("https://klau.si/blog/{}/", url_slug))
}

/// Find the latest blog post without a comment section
fn find_latest_without_comments(posts: &[BlogPost]) -> Option<&BlogPost> {
    posts.iter().find(|p| !p.has_mastodon_section)
}

/// Find a blog post by URL
fn find_by_url<'a>(posts: &'a [BlogPost], url: &str) -> Option<&'a BlogPost> {
    // Normalize URLs for comparison by removing trailing slashes
    let normalize = |s: &str| s.trim_end_matches('/').to_string();
    posts.iter().find(|p| normalize(&p.url) == normalize(url))
}

/// Post to Mastodon and return the post ID and URL
async fn post_to_mastodon(config: &Config, title: &str, url: &str, dry_run: bool) -> Result<(String, String)> {
    let status_text = format!("{}\n\n{}", title, url);

    println!("Posting to Mastodon: {}", status_text);

    if dry_run {
        println!("[DRY RUN] Would post to Mastodon");
        return Ok(("dry-run-id".to_string(), "https://example.com/@user/dry-run-id".to_string()));
    }

    let mastodon = generator(
        megalodon::SNS::Mastodon,
        config.mastodon_base_url.clone(),
        Some(config.mastodon_access_token.clone()),
        None,
    )?;

    // Verify credentials
    mastodon
        .verify_account_credentials()
        .await
        .context("Failed to verify Mastodon credentials")?;

    let options = PostStatusInputOptions {
        visibility: Some(megalodon::entities::StatusVisibility::Public),
        ..Default::default()
    };

    let response = mastodon
        .post_status(status_text, Some(&options))
        .await
        .context("Failed to post to Mastodon")?;

    let status = match response.json {
        megalodon::megalodon::PostStatusOutput::Status(status) => status,
        megalodon::megalodon::PostStatusOutput::ScheduledStatus(_) => {
            bail!("Unexpected scheduled status response");
        }
    };

    Ok((status.id, status.url.unwrap_or_default()))
}

/// Add the Mastodon comment section to the blog post
fn add_comment_section(post: &BlogPost, mastodon_id: &str) -> Result<()> {
    let content = fs::read_to_string(&post.file_path)?;

    // Add the comment shortcode at the end of the file
    let comment_section = format!(
        "\n\n{{{{ mastodon_comments(id=\"{}\") }}}}\n",
        mastodon_id
    );

    let new_content = format!("{}{}", content.trim_end(), comment_section);

    fs::write(&post.file_path, new_content)?;

    println!("Added Mastodon comment section to: {:?}", post.file_path);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Find project root and scan blog posts
    let project_root = find_project_root()?;
    let posts = scan_blog_posts(&project_root)?;

    // Determine which post to process
    let post = if let Some(url) = &args.url {
        find_by_url(&posts, url)
            .ok_or_else(|| anyhow::anyhow!("Blog post not found for URL: {}", url))?
    } else {
        find_latest_without_comments(&posts)
            .ok_or_else(|| anyhow::anyhow!("All blog posts already have comment sections"))?
    };

    // Load configuration (only required for non-dry-run or when post already has section)
    let config = if args.dry_run {
        Config::load().ok()
    } else {
        Some(Config::load()?)
    };

    // Check if already has a comment section
    if post.has_mastodon_section {
        println!("Blog post already has a comment section!");
        println!("Blog URL: {}", post.url);
        if let Some(mastodon_id) = &post.mastodon_id {
            if let Some(cfg) = &config {
                println!("Mastodon URL: {}/statuses/{}", cfg.mastodon_base_url.trim_end_matches('/'), mastodon_id);
            } else {
                println!("Mastodon ID: {}", mastodon_id);
            }
        }
        return Ok(());
    }

    println!("Processing blog post: {}", post.title);
    println!("URL: {}", post.url);

    // Post to Mastodon
    let status_text = format!("{}\n\n{}", post.title, post.url);
    let (mastodon_id, mastodon_url) = if args.dry_run {
        println!("[DRY RUN] Would post to Mastodon:\n---\n{}\n---", status_text);
        ("dry-run-id".to_string(), "https://mastodon.social/@user/dry-run-id".to_string())
    } else {
        let cfg = config.as_ref().unwrap();
        post_to_mastodon(cfg, &post.title, &post.url, false).await?
    };

    if !args.dry_run {
        println!("Posted to Mastodon: {}", mastodon_url);
    }

    // Add comment section to blog post
    if !args.dry_run {
        add_comment_section(post, &mastodon_id)?;
    } else {
        println!("[DRY RUN] Would add comment section with Mastodon ID: {}", mastodon_id);
    }

    println!("\nDone! Remember to rebuild the site with 'zola build' or './build.sh'");

    Ok(())
}
