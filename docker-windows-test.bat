@echo off
echo Building QuickSys Windows Docker container...

REM Check if the executable exists
if not exist target\release\quicksys.exe (
    echo Error: quicksys.exe not found in target\release\
    echo Please build the project first with: cargo build --release
    exit /b 1
)

REM Build the Windows container
docker build -t quicksys-windows -f Dockerfile.windows .

REM Run the container
echo.
echo Running QuickSys in Windows container...
docker run --isolation=process --rm quicksys-windows

echo.
echo To run with custom parameters, use:
echo docker run --isolation=process --rm quicksys-windows [parameters]
echo Example: docker run --isolation=process --rm quicksys-windows --select os,cpu,memory