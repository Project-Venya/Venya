#!/bin/bash

# Detect OS and architecture
OS_NAME="$(uname | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

# Set platform-specific names
if [ "$OS_NAME" == "darwin" ]; then
    if [ "$ARCH" == "arm64" ]; then
        OS_NAME="macos_arm"
    else
        OS_NAME="macos_x86"
    fi
elif [ "$OS_NAME" == "linux" ]; then
    OS_NAME="linux"
else
    echo "Unsupported OS: $OS_NAME"
    exit 1
fi

# Choose appropriate standalone Python URL based on platform
# Make sure to verify the URLs below from the python-build-standalone release page
if [ "$OS_NAME" == "macos_arm" ]; then
    BASE_URL="https://github.com/astral-sh/python-build-standalone/releases/download/20240107/cpython-3.11.7+20240107-aarch64-apple-darwin-install_only.tar.gz"
elif [ "$OS_NAME" == "macos_x86" ]; then
    BASE_URL="https://github.com/astral-sh/python-build-standalone/releases/download/20240107/cpython-3.11.7+20240107-x86_64-apple-darwin-install_only.tar.gz"
elif [ "$OS_NAME" == "linux" ]; then
    BASE_URL="https://github.com/astral-sh/python-build-standalone/releases/download/20240107/cpython-3.10.13+20240107-x86_64-unknown-linux-gnu-install_only.tar.gz"
else
    echo "Unsupported platform: $OS_NAME"
    exit 1
fi

# Destination directory
DEST_DIR="resources/${OS_NAME}"

# Skip if already exists
if [ -d "${DEST_DIR}" ]; then
    echo "Python already downloaded for ${OS_NAME}"
    exit 0
fi

# Create destination directory
mkdir -p "${DEST_DIR}"

# Download Python
echo "Downloading Python for ${OS_NAME}..."
curl -L "${BASE_URL}" -o "${DEST_DIR}/python.tar.gz"

# Check if the download was successful (non-zero length file)
if [ ! -s "${DEST_DIR}/python.tar.gz" ]; then
    echo "Failed to download the Python archive."
    exit 1
fi

# Verify file type
file_type=$(file --mime-type -b "${DEST_DIR}/python.tar.gz")
if [[ "$file_type" != "application/gzip" ]]; then
    echo "The downloaded file is not a valid gzip archive. Aborting."
    exit 1
fi

# Extract the tar.gz file
echo "Extracting Python archive..."
tar -xzf "${DEST_DIR}/python.tar.gz" -C "${DEST_DIR}"

# Remove the downloaded tar.gz file
rm "${DEST_DIR}/python.tar.gz"

# Install packages from requirements.txt using pip
echo "Installing packages from requirements.txt via pip..."
${DEST_DIR}/python/bin/python3 -m pip install -r ./requirements.txt

# You can also download from GitHub release manually
mkdir -p $DEST_DIR/hifigan/checkpoints/
wget https://github.com/jik876/hifi-gan/releases/download/v0.1/generator_v1 \
     -O $DEST_DIR/hifigan/checkpoints/generator_v1.pt

wget https://raw.githubusercontent.com/jik876/hifi-gan/master/config_v1.json \
     -O  $DEST_DIR/hifigan/checkpoints/config.json

echo "Python ${PYTHON_VERSION} and packages from requirements.txt setup complete in ${DEST_DIR}"
