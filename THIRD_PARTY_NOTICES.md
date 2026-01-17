# Third-Party Components & Licenses

PeakOS is a composite work that includes software from the PeakOS Project and third-party software.
This document provides license information and source code links for key components included in the PeakOS ISO distribution.

## 1. PeakOS Source Code
The core userland components (Peak Desktop, Compositor, Shell) are licensed under the **MIT License**.
- **Source**: Included in this repository.
- **License**: [LICENSE](./LICENSE)

## 2. The Linux Kernel
PeakOS includes the Linux Kernel, which is licensed under the **GNU General Public License, version 2 (GPLv2)**.
- **License**: [GPLv2](https://www.kernel.org/doc/html/latest/process/license-rules.html)
- **Source Code**: [Alpine Linux Kernel Source](https://git.alpinelinux.org/aports/tree/main/linux-lts)
*Note: PeakOS uses the standard Alpine Linux kernel build without modification.*

## 3. Alpine Linux
The base system is built using Alpine Linux packages. Alpine Linux is a collection of software with various open-source licenses (mostly MIT, BSD, GPL).
- **Project**: [https://alpinelinux.org/](https://alpinelinux.org/)
- **License**: [Alpine License Info](https://git.alpinelinux.org/aports/tree/main/LICENSE)
- **Source Code**: [https://git.alpinelinux.org/aports](https://git.alpinelinux.org/aports)

## 4. Rust Ecosystem
PeakOS leverages several open-source Rust crates:
- **Smithay**: MIT License ([Source](https://github.com/Smithay/smithay))
- **Iced**: MIT License ([Source](https://github.com/iced-rs/iced))
- **Winit**: Apache 2.0 License ([Source](https://github.com/rust-windowing/winit))
- **Weston**: MIT License ([Source](https://gitlab.freedesktop.org/wayland/weston))

## Distribution Notice
PeakOS is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY.
Please refer to the individual licenses of the components for specific rights and obligations.
