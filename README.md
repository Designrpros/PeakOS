# PeakOS

A modern, web-powered operating system shell designed for old hardware and seamless multitasking. It features a distributed architecture with a Rust-based intelligence backend and a web-based desktop environment.

## Key Features
- **Multitasking & Spaces**: Implemented a multi-space workspace manager that keeps all applications mounted for uninterrupted playback and lightning-fast switching.
- **Deep App Switching**: Global Alt + Tab and intelligent app launching that teleports users between spaces to avoid duplicate windows.
- **Spotlight Search**: A powerful, debounced system search with real-time filesystem indexing and intelligent app resolution.
- **Intelligent Sidebar & Dock**: Unified app launching and state management across the entire shell.
- **Production Build System**: Fully automated ISO generation via Docker, optimized for bootability on target hardware (Intel Macs).

## Project Structure
- `peak-desktop`: React + Tauri frontend.
- `peak-intelligence`: Rust-based system tools and MCP server.
- `peak-deploy`: ISO build system and deployment tools.

## Recent Milestones
- **Spotlight Wiring**: Successfully connected Spotlight to the backend `search_files` tool.
- **Launch Mapping**: Fixed system app launching (Terminal, Browser) to ensure full functionality from all entry points.
- **ISO Generation**: Built the first 2.7GB bootable ISO image with custom branding and hardware support.
- **Workspace Stability**: Resolved regressions in window snapping and persistent views.

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
