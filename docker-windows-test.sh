#!/bin/bash

echo "Building QuickSys Windows Docker container..."

# Check if the executable exists
if [ ! -f "target/release/quicksys.exe" ]; then
    echo "Error: quicksys.exe not found in target/release/"
    echo "Please build the project for Windows first."
    echo "Note: You need to cross-compile for Windows or build on a Windows machine."
    exit 1
fi

# Build the Windows container
# Note: Docker Desktop must be switched to Windows containers mode
docker build -t quicksys-windows -f Dockerfile.windows .

# Run the container
echo ""
echo "Running QuickSys in Windows container..."
docker run --isolation=process --rm quicksys-windows

echo ""
echo "To run with custom parameters, use:"
echo "docker run --isolation=process --rm quicksys-windows [parameters]"
echo "Example: docker run --isolation=process --rm quicksys-windows --select os,cpu,memory"