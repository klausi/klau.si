# AGENTS.md

Instructions for AI coding agents working on this project.

## Project Structure

This is a Zola static site generator project.

- `content/` - Markdown content files
- `static/` - Static assets that are copied as-is to the output
- `themes/even/templates/` - Jinja2/Tera templates
- `docs/` - Generated output (do not edit directly)

## Static Assets Organization

- Icon files (SVGs for social media, RSS, etc.) should be placed in `static/icons/`

## Accessibility

- Never leave image `alt` text empty. Always provide meaningful alt text for images.

## JavaScript

- Always use braces `{}` around `if` statements, even for single-line bodies.
