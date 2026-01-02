#!/bin/bash
cd "$(dirname "$0")/tools/mastodon-post" && cargo run --release -- "$@"
