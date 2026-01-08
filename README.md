# PeakOS

A modern, web-powered OS layer designed for seamless multitasking and high performance on older hardware.

## Project Structure
- `peak-desktop`: React + Tauri frontend.
- `peak-intelligence`: Rust-based system tools and MCP server.
- `peak-deploy`: ISO build system and deployment tools.

## Recent Progress
- **Real-time Spotlight**: Integrated filesystem search and intelligent app resolution.
- **Global Multitasking**: Permanent workspace mounting and cross-space app switching.
- **Automated ISO Build**: Full Docker-based build pipeline for bootable x86_64 images.

## Development
```bash
# Run desktop in dev mode
cd peak-desktop
npm install
npm run tauri dev
```

## Build
```bash
# Build ISO image
cd peak-deploy
bash build.sh
```
