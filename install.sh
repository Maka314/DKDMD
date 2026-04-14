#!/bin/bash
set -euo pipefail

REPO="Maka314/DKDMD"
PACKAGE_NAME="dkdmd"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*"; exit 1; }

# Check root
if [ "$(id -u)" -ne 0 ]; then
    error "Please run with sudo: sudo bash install.sh"
fi

# Check Ubuntu/Debian
if ! command -v dpkg &> /dev/null; then
    error "This installer only supports Debian/Ubuntu systems"
fi

# Detect architecture
ARCH=$(dpkg --print-architecture)
case "$ARCH" in
    amd64) ;;
    arm64) ;;
    *) error "Unsupported architecture: $ARCH. Only amd64 and arm64 are supported." ;;
esac

# Check dependencies
for cmd in curl grep; do
    if ! command -v "$cmd" &> /dev/null; then
        error "Required command '$cmd' not found. Please install it first."
    fi
done

# Get latest release tag
info "Fetching latest release info..."
LATEST_TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    error "Failed to fetch latest release. Check your network connection."
fi

info "Latest version: ${LATEST_TAG}"

# Build download URL
VERSION="${LATEST_TAG#v}"
DEB_FILE="${PACKAGE_NAME}_${VERSION}-1_${ARCH}.deb"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${DEB_FILE}"

# Download
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

info "Downloading ${DEB_FILE}..."
if ! curl -fSL -o "${TMPDIR}/${DEB_FILE}" "$DOWNLOAD_URL"; then
    error "Failed to download: ${DOWNLOAD_URL}"
fi

# Install
info "Installing ${PACKAGE_NAME}..."
apt install -y "${TMPDIR}/${DEB_FILE}"

info "${PACKAGE_NAME} ${LATEST_TAG} installed successfully!"
info "Run 'DKDMD' to get started."
