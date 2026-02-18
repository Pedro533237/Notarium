@echo off
setlocal

echo [Notarium] Building for Windows x64 (MSVC)...
rustup target add x86_64-pc-windows-msvc
if errorlevel 1 (
  echo Failed to install target.
  exit /b 1
)

cargo build --release --target x86_64-pc-windows-msvc
if errorlevel 1 (
  echo Build failed.
  exit /b 1
)

echo Build finished:
echo target\x86_64-pc-windows-msvc\release\notarium.exe
endlocal
