<div align="center">

# Macrocli - Advanced Macropad Configuration System

## Overview

Macrocli is a command-line device configuration system for USB macropad devices. Built with Rust, it demonstrates enterprise software practices focused on data integrity, multi-layer validation, and secure device operations.

**Description:** A comprehensive tool for programming and managing USB macropad configurations through a command-line interface. Implements robust validation pipelines, device authentication, and configuration management with emphasis on security and data integrity.

### Supported Devices

| Device Model | Vendor ID | Product ID | Status | Features |
|--------------|-----------|------------|---------|----------|
| **K884X/K8842** | 0x1189 | 0x8840/0x8842 | ✅ Fully Working | 17-key sequences, delays, LED colors, read/write |
| **K8890** | 0x1189 | 0x8890 | ✅ Fully Working | 5-key sequences, no delays, limited media keys, read/write |
| **K8850** | 0x514c | 0x8850 | ✅ Fully Working | 18-key sequences, multi-layer reading, read/write |

#### Device-Specific Notes

**K8850 (QingHeng Electronics):**
- **Reading**: ✅ Multi-layer reading - reads all 3 layers in single command
- **Programming**: ✅ Full programming support for all 25 keys (16 buttons + 9 knob actions)
- **Protocol**: Uses Magic Init Packet for device communication
- **Usage**: `./macrocli --vendor-id 0x514c --product-id 0x8850 read` (reads all layers by default)

**K884X/K8842:**
- Standard USB protocol implementation
- Supports 17-key sequences with customizable delays
- LED color configuration per layer

**K8890:**
- Limited device with simplified protocol
- 5-key maximum sequences (no delays supported)
- Restricted media key set

### System Components

| Component             | Technology Stack | Purpose                                               |
| --------------------- | ---------------- | ----------------------------------------------------- |
| **CLI Tool**          | Rust             | High-performance device communication & configuration |
| **Validation Engine** | Rust             | Multi-layer data validation & integrity checking      |
| **USB Layer**         | rusb             | Direct USB device communication                       |

## Skills Demonstrated

This project showcases competencies relevant to fraud analysis, data integrity, and security operations:

### Data Validation & Integrity
- Multi-layer validation pipeline ensuring data accuracy before device programming
- Input sanitization with strict schema enforcement
- Cross-layer configuration consistency verification
- Comprehensive error detection and diagnostic reporting

### Security & Access Control
- USB device authentication via VID/PID verification
- Linux privilege management through udev rules (non-root access)
- Secure error handling without information leakage
- Pre-write data integrity verification

### Analysis & Problem-Solving
- Pattern recognition in keyboard mappings and configurations
- Binary protocol parsing and encoding
- Invalid configuration detection and prevention
- Root cause analysis with detailed diagnostics

### Technical Implementation
- Command-line tool development with Rust
- System integration: USB protocols and device communication
- Type-safe implementation leveraging Rust's compile-time guarantees
- Clean architecture with separation of concerns
- Direct hardware interaction without abstraction overhead

---

## Repository Structure

```
Macrocli/
├── Macrocli/                 # Main application directory
│   ├── src/                  # Rust source code
│   │   ├── main.rs          # CLI entry point and commands
│   │   ├── config.rs        # Configuration validation
│   │   ├── decoder.rs       # Device data decoding
│   │   ├── mapping.rs       # Key mapping definitions
│   │   ├── options.rs       # CLI argument parsing
│   │   └── keyboard/        # Device implementations (k884x, k8890, k8850)
│   ├── macropad_configs/    # Example configuration templates
│   │   ├── 12but-3layer-3knobs.ron  # 3x4 keyboard template
│   │   ├── 3but-1knob.ron           # 3-button template
│   │   └── 4x4_3knob.ron            # 4x4 keyboard template
│   ├── 80-macrocli.rules    # Linux udev rules
│   └── Cargo.toml           # Rust dependencies
├── LICENSE                   # CC BY-SA 3.0 License
└── README.md                # This file
```

---

## Key Features

- **Command-Line Interface** - Efficient CLI for device configuration and management
- **Device Auto-Detection** - Automatic USB device discovery
- **Import/Export** - Save and load configurations as `.ron` files
- **Direct Device Programming** - Flash configurations directly to device
- **Configuration Backup** - Read and backup existing device configurations
- **Multi-Layer Support** - Up to 3 independent layout layers
- **High Performance** - Rust implementation for speed and reliability
- **Secure Access Control** - Privilege management for device access

---

## Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- USB macropad device (see Supported Devices table above)

### Installation & Setup

**Step 1: Build the CLI Tool**
```bash
cd Macrocli/
cargo build --release
```

**Step 2: Configure Linux Permissions (Linux only)**
```bash
sudo cp 80-macrocli.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
# Re-plug your device
```

**Step 3: Run Commands**
```bash
# Show available keys
./target/release/macrocli show-keys

# Validate a configuration
./target/release/macrocli validate -c config.ron

# Program your device
./target/release/macrocli program -c config.ron
```

---

## Architecture

### System Design

```
┌─────────────────────────────────────────────────────────────┐
│                    Command-Line Interface                    │
│                         (main.rs)                            │
└──────────────────────────────┬──────────────────────────────┘
                               │ Direct Calls
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Core Business Logic                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Validation  │  │   Encoding   │  │   Decoding   │     │
│  │   Engine     │→ │   Engine     │→ │   Engine     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└────────────────────────────┬────────────────────────────────┘
                             │ USB Protocol (rusb)
                             ▼
                  ┌──────────────────────┐
                  │   USB Macropad       │
                  │   Hardware Device    │
                  └──────────────────────┘
```

### Design Principles

- **Type Safety** - Rust's type system prevents bugs at compile-time
- **Memory Safety** - No buffer overflows or use-after-free vulnerabilities
- **Input Validation** - Multi-layer data validation (defense in depth)
- **Error Propagation** - Comprehensive error handling with diagnostics
- **Separation of Concerns** - Clear boundaries between CLI, validation, and device layers

---

## CLI Reference

Command-line interface for direct device operations and automation:

### Commands

```bash
# Program device with configuration file
./target/release/macrocli program -c config.ron

# Validate configuration without programming
./target/release/macrocli validate -c config.ron

# Read current device configuration
./target/release/macrocli read

# Device-specific reading commands:
# K8850 (multi-layer by default):
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read

# Other devices (specify layer):
./target/release/macrocli read --layer 1

# Display all supported keys
./target/release/macrocli show-keys

# Control LED settings
./target/release/macrocli led <index> <layer> [color]
```

### Linux Security Configuration

Non-root device access setup:

```bash
# Copy udev rules
sudo cp Macrocli/80-macrocli.rules /etc/udev/rules.d/

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Re-plug device to apply changes
```

This configuration implements:
- Principle of least privilege (no root required)
- Proper Linux device permissions management
- Secure multi-user system configuration

---

## Contributing

This project follows professional development standards. All contributions must adhere to:

- **Code Quality** - Pass Rust compiler checks and linting
- **Documentation** - Document new features and changes
- **Testing** - Test validation logic thoroughly
- **Security** - Follow secure coding practices

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## Security

Security best practices implemented:

- Multi-layer input validation
- Principle of least privilege (non-root execution)
- Type-safe memory management (Rust)
- Device authentication via USB VID/PID
- Secure error handling (no information leakage)

For security concerns, see [SECURITY.md](SECURITY.md).

---

## License

Licensed under the Creative Commons Attribution-ShareAlike 3.0 Unported License.

See [LICENSE](LICENSE) for full details.

---

## Acknowledgements

- Inspired by [eccherda/ch552g_mini_keyboard](https://github.com/eccherda/ch552g_mini_keyboard)
- Built with [Rust](https://www.rust-lang.org/) and [rusb](https://github.com/a1ien/rusb)

---
