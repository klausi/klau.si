# Website klau.si

## Test locally
```
zola serve
```

## Build
```
./build.sh
```

## Optimize image to webp
```
./tools/optimize-image.sh <filepath>
```

## Blog Comments via Mastodon

Blog posts can have a comment section that displays replies from Mastodon. When you publish a new blog post and want to enable comments, use the `mastodon-post` tool.

### Setup

1. Create a `.env` file in the project root with your Mastodon credentials:

```
MASTODON_BASE_URL=https://mastodon.social
MASTODON_ACCESS_TOKEN=your_access_token_here
MASTODON_HANDLE=@yourname@mastodon.social
```

To get an access token:
1. Log in to your Mastodon account
2. Go to Settings -> Development -> New Application
3. Give it a name (e.g., "Blog Comments")
4. Select scopes:
  * `read:statuses`
  * `write:media`
  * `write:statuses`
5. Click "Submit"
6. Click on the application name and copy the "Your access token" value

### Usage

Build the tool first:
```
cd tools/mastodon-post
cargo build --release
```

Post the latest blog entry without a comment section:
```
./post-mastodon.sh
```

Or specify a specific blog post URL:
```
./post-mastodon.sh --url "https://klau.si/blog/my-blog-post/"
```

Dry run (shows what would be done without posting):
```
./post-mastodon.sh --dry-run
```

After running the tool:
1. The blog post will have a new `{{ mastodon_comments(id="...") }}` shortcode added
2. Rebuild the site with `./build.sh`
3. Commit and push the changes

### Moderation (hiding comment replies)

Edit `themes/even/templates/shortcodes/mastodon_comments.html` and list accounts or posts to hide in `blockedAccounts` or `blockedStatuses`.
