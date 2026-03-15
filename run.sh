#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DOCS_DIR="$SCRIPT_DIR/src"

if [ ! -d "$DOCS_DIR" ]; then
    echo "Docs directory not found at $DOCS_DIR"
    exit 1
fi

if ! command -v npm >/dev/null 2>&1; then
    echo "npm is required to build and run the docs site."
    exit 1
fi

cd "$DOCS_DIR"

echo "Installing docs dependencies..."
npm install

echo "Building docs site..."
npm run build

echo "Starting docs dev server (Ctrl+C to stop)..."
npx serve _site