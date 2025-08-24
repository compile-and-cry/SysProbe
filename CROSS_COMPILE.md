# Cross-Compiling QuickSys for Windows

This guide explains how to cross-compile QuickSys for Windows from macOS or Linux.

## Prerequisites

1. Rust and Cargo installed via rustup
2. Windows target added to rustup

## Setup Cross-Compilation

### 1. Add Windows Target

```bash
# Add the Windows MSVC target
rustup target add x86_64-pc-windows-msvc

# Or add the Windows GNU target
rustup target add x86_64-pc-windows-gnu
```

### 2. Install Required Tools

#### On macOS:

```bash
# Install MinGW-w64 using Homebrew
brew install mingw-w64
```

#### On Linux (Debian/Ubuntu):

```bash
# Install MinGW-w64
sudo apt-get install mingw-w64
```

## Cross-Compile QuickSys

```bash
# For Windows MSVC target (requires Visual Studio installed)
cargo build --release --target x86_64-pc-windows-msvc

# For Windows GNU target
cargo build --release --target x86_64-pc-windows-gnu
```

The compiled Windows executable will be available at:
- `target/x86_64-pc-windows-msvc/release/quicksys.exe` (MSVC target)
- `target/x86_64-pc-windows-gnu/release/quicksys.exe` (GNU target)

## Testing with Docker Windows Container

After cross-compiling, you can test the Windows executable using Docker:

1. Copy the executable to the expected location:

```bash
# Create the directory if it doesn't exist
mkdir -p target/release/

# Copy the executable
cp target/x86_64-pc-windows-gnu/release/quicksys.exe target/release/
```

2. Run the Docker Windows container test script:

```bash
./docker-windows-test.sh
```

## Troubleshooting

### Common Issues

1. **Missing Windows Libraries**: If you encounter errors about missing Windows libraries, make sure you have the correct MinGW-w64 installation.

2. **Docker Windows Containers**: Ensure Docker Desktop is switched to Windows containers mode before running the test scripts.

3. **Cross-Compilation Errors**: Some Windows-specific features might not cross-compile correctly. In such cases, consider using the mock collector mode for development and testing on actual Windows machines for final verification.