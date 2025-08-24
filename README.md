# QuickSys - System Information Collector

QuickSys is a small, fast Rust CLI tool designed to collect system information on Windows platforms with a focus on performance and minimal resource usage. It can also detect Tally software installations.

## Features

- Collects detailed system information: OS, device, CPU, memory, disk, and network
- Specialized Tally software detection (ERP9/Prime)
- JSON output (stdout or via optional HTTP API)
- Lightweight and fast (≤300ms typical runtime)
- Small binary size (≤5MB)
- Low memory usage (≤30MB)

## Usage

```
quicksys.exe [FLAGS]
```

### Flags

- `--pretty` - Pretty-print JSON output
- `--select <fields>` - Select specific fields (e.g., os,cpu,apps.tally)
- `--no-tally` - Skip Tally detection
- `--tally-http [host:port]` - Probe Tally HTTP (default 127.0.0.1:9000)
- `--timeout-ms <n>` - Global timeout in milliseconds (default 500)
- `--http [port]` - Start local HTTP server
- `--version` - Print collector version

## Example Output

```json
{
  "collector": {"name": "QuickSys", "version": "1.0.0", "duration_ms": 172},
  "os": {"family": "Windows", "edition": "Pro", "version": "22H2", "build": "19045", "arch": "x86_64"},
  "cpu": {"name": "Intel i7", "physical_cores": 6, "logical_cores": 12},
  "memory": {"installed_mb": 16384, "available_mb": 8240},
  "apps": {"tally": {"installed": true, "variant": "TallyPrime", "version": "3.0.1"}}
}
```

## Tally Detection Strategy

QuickSys uses multiple methods to detect Tally software:

1. Registry Uninstall keys (match DisplayName like Tally*)
2. Vendor keys under HKLM\SOFTWARE\Tally Solutions
3. Default install paths (C:\Program Files\Tally*)
4. File version info of Tally.exe

Optional probes (with timeout ≤200ms):
- HTTP/XML (127.0.0.1:9000) → request product/version
- ODBC DSN (if TallyODBC exists)

## Building

```bash
# Standard build
cargo build --release

# Build without HTTP server feature
cargo build --release --no-default-features

# Build with all features
cargo build --release --features "http tally-xml tally-odbc"
```

After building, the executable will be located at:
- **Windows**: `target/release/quicksys.exe`
- **macOS/Linux**: `target/release/quicksys`

For detailed deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md).

## Running

### Windows

After building, you can run the executable directly:

```bash
# Run with default settings
target/release/quicksys.exe

# Run with pretty-printed output
target/release/quicksys.exe --pretty

# Run with specific field selection
target/release/quicksys.exe --select os,cpu,memory

# Run with HTTP server on port 8080
target/release/quicksys.exe --http 8080
```

### Cross-Platform Mode

QuickSys can be built and run on non-Windows platforms (macOS, Linux) in compatibility mode:

```bash
# Build on non-Windows platforms
cargo build --release

# Run in compatibility mode
target/release/quicksys
```

Note: In cross-platform mode, QuickSys will return mock data and indicate it's running in compatibility mode.

## Deployment

### Standalone Executable

QuickSys compiles to a single executable with no external dependencies. To deploy:

1. Build the release version: `cargo build --release`
2. Copy the executable from `target/release/quicksys.exe` (Windows) or `target/release/quicksys` (non-Windows)
3. Deploy to target systems

### Integration Options

- **Command-line integration**: Call QuickSys from scripts or other applications and parse the JSON output
- **HTTP API**: Use the `--http` flag to start a local HTTP server and query system information via REST API
- **Scheduled task**: Configure QuickSys to run periodically and output to a file for monitoring purposes

### Docker Deployment

#### Linux/macOS Development (Mock Mode)

For development and testing on non-Windows platforms with mock data:

```bash
# Build Docker image
docker build -t quicksys .

# Run in Docker
docker run --rm quicksys
```

#### Windows Container Testing

To test the actual Windows functionality in a Windows container:

1. Make sure Docker Desktop is switched to Windows containers mode
2. Build the Windows executable (requires cross-compilation or building on Windows)
3. Run the provided script:

```bash
# On Windows
.\docker-windows-test.bat

# On macOS/Linux
./docker-windows-test.sh
```

This will build and run a Windows container with the QuickSys executable, allowing you to test the full Windows functionality including Tally detection.

See [CROSS_COMPILE.md](CROSS_COMPILE.md) for detailed instructions on cross-compiling for Windows from macOS or Linux.

## Performance Requirements

- Binary Size: ≤5 MB (release, stripped)
- Run Time: ≤300 ms typical; ≤450 ms p95
- Memory Use: ≤30 MB