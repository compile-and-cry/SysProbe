# QuickSys Deployment Guide

## Building the Application

### Prerequisites

- Rust toolchain (rustc, cargo) installed
- For Windows builds: Windows OS or cross-compilation setup
- For Docker: Docker installed and running

### Standard Build Process

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/yourusername/SysProbe.git
cd SysProbe

# Build the app with default features
cargo build --release

# Build without HTTP server feature
cargo build --release --no-default-features

# Build with all features
cargo build --release --features "http tally-xml tally-odbc"
```

### Where to Find the Executable

After a successful build, the executable will be located at:

- **Windows**: `target/release/quicksys.exe`
- **macOS/Linux**: `target/release/quicksys`

## Running the Application

### Windows (Native)

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

### Cross-Platform Mode (macOS/Linux)

```bash
# Run in compatibility mode (returns mock data)
target/release/quicksys

# Run with HTTP server
target/release/quicksys --http 8080
```

## Docker Deployment

### Linux/macOS Development (Mock Mode)

```bash
# Build Docker image
docker build -t quicksys .

# Run in Docker
docker run --rm quicksys

# Run with HTTP server and expose port
docker run --rm -p 8080:8080 quicksys --http 8080
```

### Windows Container Testing

To test the Windows executable in a Docker container:

```bash
# On Windows (make sure Docker Desktop is in Windows containers mode)
.\docker-windows-test.bat

# On macOS/Linux (requires cross-compiled Windows executable)
./docker-windows-test.sh
```

For cross-compilation instructions, see [CROSS_COMPILE.md](CROSS_COMPILE.md).

## Deployment Options

### Standalone Executable

The simplest deployment method is to copy the compiled executable to the target machine and run it directly.

1. Build the application as described above
2. Copy `target/release/quicksys.exe` (Windows) or `target/release/quicksys` (macOS/Linux) to the target machine
3. Run the executable with desired parameters

### Integration Options

#### Command-line Integration

QuickSys can be integrated into scripts or other applications by capturing its JSON output:

```bash
# Windows
quicksys.exe > system_info.json

# Parse with PowerShell
Get-Content system_info.json | ConvertFrom-Json
```

#### HTTP API Integration

Start QuickSys with the HTTP server option to integrate via REST API:

```bash
# Start HTTP server
quicksys.exe --http 8080

# In another application, fetch data
curl http://localhost:8080/api/info

# Get specific information
curl http://localhost:8080/api/info/os
curl http://localhost:8080/api/info/cpu
```

#### Scheduled Task (Windows)

Set up QuickSys to run periodically and save system information:

```powershell
# Create a scheduled task (PowerShell)
$action = New-ScheduledTaskAction -Execute "C:\path\to\quicksys.exe" -Argument "--select os,cpu,memory > C:\logs\system_info.json"
$trigger = New-ScheduledTaskTrigger -Daily -At 8am
Register-ScheduledTask -Action $action -Trigger $trigger -TaskName "QuickSys Daily Report" -Description "Collect system information daily"
```

## Performance Considerations

- QuickSys is designed to be lightweight with minimal resource usage
- Typical runtime: ≤300ms
- Memory usage: ≤30MB
- Binary size: ≤5MB

## Troubleshooting

### Common Issues

1. **Missing Windows Dependencies**: Ensure you have the necessary Windows components installed if running on Windows
2. **Permission Issues**: Run with administrator privileges if accessing certain system information
3. **Port Conflicts**: If using the HTTP server, ensure the specified port is not in use by another application

### Logs

QuickSys outputs errors to stderr. Redirect stderr to a file for troubleshooting:

```bash
quicksys.exe 2> error.log
```