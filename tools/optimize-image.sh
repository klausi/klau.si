#!/bin/bash
set -e

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "Error: ImageMagick is not installed. Please install it (e.g., sudo apt install imagemagick)."
    exit 1
fi

# Check arguments
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <path_to_image>"
    exit 1
fi

INPUT_IMAGE="$1"

if [ ! -f "$INPUT_IMAGE" ]; then
    echo "Error: File '$INPUT_IMAGE' not found."
    exit 1
fi

# Determine output filename
DIR=$(dirname "$INPUT_IMAGE")
FILENAME=$(basename "$INPUT_IMAGE")
NAME="${FILENAME%.*}"
OUTPUT_IMAGE="${DIR}/${NAME}.webp"

echo "Optimizing '$INPUT_IMAGE'..."

# Convert to WebP
# -resize 1200x> : Resize width to 1200px only if the image is wider (maintains aspect ratio)
# -quality 80    : Standard good quality for WebP
# -strip         : Remove metadata to save space
convert "$INPUT_IMAGE" -resize '1200x>' -quality 80 -strip "$OUTPUT_IMAGE"

echo "Created '$OUTPUT_IMAGE'"
