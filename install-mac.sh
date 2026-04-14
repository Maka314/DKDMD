#!/bin/bash
set -euo pipefail

REPO="Maka314/DKDMD"
BINARY_NAME="DKDMD"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# Check macOS
if [[ "$(uname)" != "Darwin" ]]; then
    error "This installer is for macOS only."
fi

# Check dependencies
for cmd in curl; do
    if ! command -v "$cmd" &> /dev/null; then
        error "Required command '$cmd' not found."
    fi
done

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64) ASSET_ARCH="x86_64" ;;
    arm64)  ASSET_ARCH="arm64" ;;
    *)      error "Unsupported architecture: $ARCH" ;;
esac

# Get latest release tag
info "Fetching latest release info..."
LATEST_TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":[[:space:]]*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    error "Failed to fetch latest release. Check your network connection."
fi

info "Latest version: ${LATEST_TAG}"

# Build download URL
ASSET_FILE="DKDMD-macos-${ASSET_ARCH}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${ASSET_FILE}"

# Download
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

info "Downloading ${ASSET_FILE}..."
if ! curl -fSL -o "${TMPDIR}/${BINARY_NAME}" "$DOWNLOAD_URL"; then
    error "Failed to download: ${DOWNLOAD_URL}"
fi

chmod +x "${TMPDIR}/${BINARY_NAME}"

# Install
info "Installing to ${INSTALL_DIR}/${BINARY_NAME}..."
if [ -w "$INSTALL_DIR" ]; then
    mv "${TMPDIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
else
    sudo mv "${TMPDIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
fi

info "${BINARY_NAME} ${LATEST_TAG} installed successfully!"
info "Run 'DKDMD' to get started."
