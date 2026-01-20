# PeakOS

**A Hybrid Operating System built with Rust — One Core, Many Faces.**

Boot seamlessly into **Desktop**, **Console**, **TV**, or **Robot** mode. Your OS adapts to your hardware context. Infinite possibilities on a minimal footprint.

---

## Why PeakOS?

### The Problem
Modern operating systems force you to choose:
- **Ubuntu/Windows:** Bloated, slow, and locked to one form factor.
- **Android/ChromeOS:** Fragmented ecosystems that struggle to scale from phone to desktop to TV.
- **ROS (Robot OS):** Middleware, not an OS. Heavy and complex to secure.

### The Solution
**PeakOS gives you everything, sacrifices nothing:**

| Feature | PeakOS | Ubuntu | Windows | Android |
|---------|--------|--------|---------|---------|
| **Multi-Mode** | ✅ (Desktop/Robot/TV) | ❌ | ❌ | ❌ (Fragmented) |
| **Boot Time** | ~3 sec | ~45 sec | ~30 sec | ~20 sec |
| **RAM Idle** | ~200 MB | ~1.5 GB | ~2 GB | ~800 MB |
| **Safety** | Memory-Safe Rust | Legacy C/C++ | Proprietary C++ | Java/C++ |
| **Robotics** | Native First-Class | Heavy Overlay | Not Supported | High Latency | 
| **Privacy** | 100% | Opt-out | 0% | 0% |

---

## Key Features

### 🌍 Universal Compatibility (Hybrid OS)
More than just a desktop. PeakOS adapts to your hardware:
- **Desktop Mode:** Traditional window management for productivity.
- **Mobile Mode:** Touch-optimized UI for Linux phones (PinePhone/Librem 5).
- **TV Mode:** 10-foot interface for media consumption.
- **Console Mode:** Controller-first gaming dashboard.
- **Robot Mode:** High-performance headless/face-rendering state for robotics.
- **Kiosk/Auto/Fireplace:** Specialized modes for specific environments.

### 🎨 **Stunning UI & Glassmorphism**
- **Modern Aesthetics:** Built with `iced` 0.13, featuring real-time blurred backgrounds, glassmorphism overlays, and smooth animations.
- **Consistent Design:** Unified design language across Login, Setup Wizard, and Desktop.
- **Adaptive Themes:** Seamless Light/Dark mode switching with dynamic wallpaper adjustments.

### 🚀 **Blazing Fast & Safe**
- **Native Rust Shell:** GPU-accelerated, memory-safe, crash-proof.
- **Alpine Linux Base:** Minimal attack surface, instant boot.
- **Zero Bloat:** The OS grows with your needs, stripped down by default.

### 🤖 **AI-First Computing**
- **Peak Intelligence:** Built-in AI assistant with local (LLM) or cloud model support.
- **Context-Aware:** The "Inspector" panel provides persistent AI context across all apps.
- **Identity Layer:** AI-generated branding and logos for a unique, futuristic identity.

---

## Architecture

```
┌─────────────────────────────────────────┐
│   peak-native (Rust Shell)              │  ← Window manager, desktop, apps
│   • Terminal • Explorer • Settings      │
│   • Browser • Jukebox • Store           │
│   • Peak Intelligence (AI)              │
├─────────────────────────────────────────┤
│   Alpine Linux (Lightweight Base)       │  ← Minimal, secure foundation
│   • glibc compatibility layer           │
│   • APK package manager                 │
├─────────────────────────────────────────┤
│   Linux Kernel (Intel x86_64 | ARM64)   │  ← Cross-platform from day one
└─────────────────────────────────────────┘
```

### Project Structure
- **`peak-native/`**: Core desktop environment (Rust + iced).
- **`peak-intelligence/`**: AI assistant, MCP server, and voice protocols.
- **`peak-deploy/`**: Docker-based ISO build system for reproducible builds.
- **`peak-core/`**: Shared libraries, AppRegistry, and system utilities.

---

## Getting Started

### Development (Local)
Run the desktop environment as a window on your current OS (macOS/Linux/Windows):
```bash
cd crates/modes/desktop
cargo run
```

### Build Bootable ISO (Docker)
We use Docker to ensure a consistent, reproducible build environment for the Alpine-based ISO.
```bash
# Build for Intel (x86_64)
./crates/peak-deploy/build.sh --intel --docker

# Build for ARM (aarch64 - M-series Macs, Raspberry Pi)
./crates/peak-deploy/build.sh --arm --docker
```
*Artifacts will be output to `crates/peak-deploy/out/`.*

### Flash to USB
```bash
# macOS
sudo dd if=peakos-alpine-arm64.iso of=/dev/diskX bs=1m

# Linux
sudo dd if=peakos-alpine-x86_64.iso of=/dev/sdX bs=1M status=progress
```

---

## Roadmap

**Current Status:** Beta (UI Polish & Optimization)

### Phase 1: Core Foundation ✅
- [x] Window management & Multi-workspace support
- [x] Native apps (Terminal, Explorer, Settings)
- [x] Bootable ISO generation (Intel & ARM)

### Phase 2: UI & UX Refinement (Current) 🔄
- [x] **Glassmorphism UI:** Blurred overlays for Login & Setup.
- [x] **App Registry:** Centralized app management.
- [x] **Shell Modes:** Dynamic switching between Desktop, Mobile, etc.
- [ ] **Animations:** Smooth transitions for window opening/closing.

### Phase 3: AI & Intelligence ✅
- [x] Peak Intelligence framework & Inspector
- [x] Local LLM support (`llama-server`)
- [x] Voice input/output protocols

### Phase 4: Release 📅
- [ ] Installer Wizard (Partitioning)
- [ ] Community Package Repository
- [ ] Public Beta Release

---

## Technical Details

**Built With:**
- **Language:** Rust (2021 edition)
- **GUI Framework:** [iced](https://github.com/iced-rs/iced) 0.13+
- **Base OS:** Alpine Linux (Edge/Rolling)
- **Window Manager:** Custom Rust-native compositor
- **Package Manager:** APK + AppImage support

**System Requirements:**
- **CPU:** Intel x86_64 or ARM64 (aarch64)
- **RAM:** 512 MB minimum, 2 GB recommended (8 GB+ for AI features)
- **Storage:** 4 GB minimum
- **GPU:** OpenGL 3.0+ / Vulkan support recommended

---

## License

[MIT License](LICENSE) — Free to use, modify, and distribute.

---

**PeakOS** — *Minimal modern OS. Maximum compatibility.*
