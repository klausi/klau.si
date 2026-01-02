# AGENTS.md

Instructions for AI coding agents working on this project.

## Project Structure

This is a Zola static site generator project.

- `content/` - Markdown content files
- `static/` - Static assets that are copied as-is to the output
- `themes/even/templates/` - Jinja2/Tera templates
- `docs/` - Generated output (do not edit directly)
- `tools/` - CLI tools for managing the blog
  - `tools/mastodon-post/` - Rust CLI for posting blog entries to Mastodon for comments

## Static Assets Organization

- Icon files (SVGs for social media, RSS, etc.) should be placed in `static/icons/`

## Accessibility

- Never leave image `alt` text empty. Always provide meaningful alt text for images.

## JavaScript and CSS

- Always use braces `{}` around `if` statements, even for single-line bodies.
- Use 2 space indentation for all JS and CSS code

## Zola

- When using or generating URLs in `img` or other HTML tags always use the `get_url()` function.

## Rust

- Run `cargo clippy` to validate code
