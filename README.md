# Macrocli

**Cross-platform CLI tool and modern GUI for programming Chinese USB macro keyboards without vendor software.**

## Why This Exists

Chinese macro keyboards (K8850, K884X, K8890) offer excellent hardware value but ship with questionable closed-source Windows-only software. This project provides a clean, auditable alternative built entirely through AI-assisted development.

**Built by a non-programmer** — I'm a financial analyst who needed keyboard automation for specific workflows. With no prior Rust or systems programming experience, I reverse-engineered the USB HID protocol and built this tool collaboratively with AI (Claude) over several weeks.

Inspired by [eccherda/ch552g_mini_keyboard](https://github.com/eccherda/ch552g_mini_keyboard), which provides firmware for similar CH552G-based hardware.

## What It Does

- **Programs macro sequences** to physical keys (up to 18 keystrokes per button)
- **Per-key timing control** — millisecond-precision delays between keystrokes (K8850)
- **Multi-layer support** — 3 switchable keyboard layers (75 programmable actions)
- **Reads configurations back** from device memory
- **Cross-platform** — Linux/Windows, no vendor drivers required
- **Modern WPF GUI** — Visual configuration with dark theme, OSD overlay, and dynamic layout
- **On-Screen Display (OSD)** — Transparent overlay showing button labels in real-time

## Use Cases

Macro keyboards accelerate repetitive workflows common in:
- **Financial analysis** — caret browsing, system navigation, data entry sequences
- **Compliance operations** — case management shortcuts, investigation tool macros
- **Trading workflows** — rapid order entry, platform navigation
- **General productivity** — any multi-step keyboard automation

## Technical Approach

The vendor software was analyzed via USB packet capture (Wireshark/USBPcap). Key discoveries:
- Custom HID protocol on endpoint 0x04
- Big-endian delay encoding (initially caused 256x timing errors)
- Per-key delay support unique to K8850 hardware

No decompilation or proprietary code was used — pure protocol analysis.

## Supported Devices

| Device | VID:PID | Keys | Delays | Notes |
|--------|---------|------|--------|-------|
| K8850 | 514c:8850 | 16 + 3 knobs | Per-key (0-6000ms) | Full support (LEDs pending) |
| K884X | 1189:8840/8842 | 16 | Global only | LED control |
| K8890 | 1189:8890 | 5 | None | Basic macros |

## Quick Start

### Download

Go to the [Releases](../../releases) page and download `Macropad_v0.0.1_Win64.zip`.

### Usage

1. Extract the ZIP.
2. Run `MacropadGUI.exe`.
3. Click "Read from Device".
4. Use "👁 OSD" button to toggle the overlay.

### Building from Source

Requirements:
- Rust (Cargo)
- .NET 8 SDK

Build Script (PowerShell):
```powershell
./build_release.ps1
```

This will generate a `release/` folder with the compiled binaries.

## Project Structure

```
src/                     # Rust CLI backend
├── keyboard/           # Device-specific protocol implementations
│   ├── k8850.rs        # QingHeng K8850 (most complete)
│   ├── k884x.rs        # K8840/K8842 support
│   └── k8890.rs        # K8890 support
├── mapping.rs          # Configuration parsing/validation
└── main.rs             # CLI interface

gui/                    # C# WPF GUI frontend
├── App.xaml            # Application resources
├── MainWindow.xaml     # Main configuration window
├── OsdWindow.xaml      # On-Screen Display overlay
└── EditLabelDialog.xaml # Label editing dialog
```

## GUI Features

### 🖥️ GUI Configurator (Windows)

A modern, dark-themed application to configure your device visually.
- **Dynamic Layout**: Automatically detects 3-key, 9-key, 12-key, and 16-key macropads.
- **Visual Mapping**: Click buttons to edit them.
- **OSD (On-Screen Display)**: Semi-transparent, always-on-top overlay to remind you of your shortcuts.
  - Shows friendly names (Labels) instead of raw codes.
  - Right-click any button in OSD to rename it instantly.
- **Profiles**: Button labels are saved locally in `labels.json`.
- **Portable**: Single executable, no installation required.

### 🦀 CLI (Rust)

Fast and reliable command-line tool.
- Read/Write configuration to device.
- Save config to `.ron` files.
- Cross-platform core (Windows/Linux/macOS - though HID implementation varies).

## Roadmap

- [x] GUI configuration tool
- [ ] LED support for K8850

---

*First time using git — some commit history may be chaotic.*
