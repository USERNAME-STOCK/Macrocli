<div align="center">

# Macrocli - Advanced Macropad Configuration System

[![License: CC BY-SA 3.0](https://img.shields.io/badge/License-CC%20BY--SA%203.0-blue.svg)](https://creativecommons.org/licenses/by-sa/3.0/)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![React](https://img.shields.io/badge/react-%2320232a.svg?style=flat&logo=react&logoColor=%2361DAFB)](https://reactjs.org/)
[![Security](https://img.shields.io/badge/security-validated-brightgreen.svg)](SECURITY.md)

**Production-grade USB device configuration tool with emphasis on data validation, security, and integrity**

[Features](#key-features) • [Quick Start](#quick-start) • [Architecture](#architecture) • [Skills](#skills-demonstrated) • [API](#api-reference)

</div>

---

## Overview

Macrocli is a full-stack device configuration system for USB macropad devices (VID: `0x1189`, PIDs: `0x8840`, `0x8842`, `0x8890`). Built with Rust and React/TypeScript, it demonstrates enterprise software practices focused on data integrity, multi-layer validation, and secure device operations.

**Description:** A comprehensive tool for programming and managing USB macropad configurations through CLI and web interfaces. Implements robust validation pipelines, device authentication, and real-time configuration management with emphasis on security and data integrity.

### System Components

| Component | Technology Stack | Purpose |
|-----------|-----------------|---------|
| **Backend CLI** | Rust + Axum | High-performance device communication & REST API server |
| **Frontend UI** | React + TypeScript | Interactive visual configuration editor |
| **Validation Engine** | Rust | Multi-layer data validation & integrity checking |
| **API Layer** | RESTful JSON | Secure device operations interface |

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
- API security with structured error handling
- Pre-write data integrity verification

### Analysis & Problem-Solving
- Pattern recognition in keyboard mappings and configurations
- Binary protocol parsing and encoding
- Invalid configuration detection and prevention
- Root cause analysis with detailed diagnostics

### Technical Implementation
- Full-stack development: Rust backend, React/TypeScript frontend
- RESTful API architecture with comprehensive documentation
- System integration: USB protocols, web server, client interface
- Type-safe implementation leveraging Rust's compile-time guarantees
- Clean architecture with separation of concerns

---

## Repository Structure

```
Macrocli/
├── src/                      # Rust source code
│   ├── main.rs              # Application entry point
│   ├── api.rs               # REST API endpoints
│   ├── config.rs            # Configuration validation logic
│   ├── decoder.rs           # Device data decoding
│   ├── mapping.rs           # Key mapping definitions
│   └── keyboard/            # Device-specific implementations
├── Webapp/                   # React TypeScript frontend
│   ├── src/                 # Frontend source code
│   └── public/              # Static assets
├── macropad_configs/        # Configuration file storage (.ron)
├── macropad_backups/        # Device backup storage
├── 80-macrocli.rules        # Linux udev rules (security)
└── Cargo.toml               # Rust dependencies
```

---

## Key Features

- **Visual Configuration Editor** - Web-based interface for device layout creation
- **Real-time Validation** - Instant configuration validity feedback
- **Device Auto-Detection** - Automatic USB device discovery
- **Import/Export** - Save and load configurations as `.ron` files
- **Live Device Programming** - Direct device flashing via web interface
- **Configuration Backup** - Read and backup existing device configurations
- **Multi-Layer Support** - Up to 3 independent layout layers
- **High Performance** - Rust implementation for speed and reliability
- **Secure Access Control** - Privilege management for device access

---

## Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- Node.js (18+) and npm
- USB macropad device (VID: `0x1189`, PID: `0x8840`/`0x8842`/`0x8890`)

### Installation & Setup

**Step 1: Build the Backend**
```bash
cd Macrocli/
cargo build --release
```

**Step 2: Build the Frontend**
```bash
cd Webapp/
npm install
npm run build
cd ..
```

**Step 3: Configure Linux Permissions (Linux only)**
```bash
sudo cp 80-macrocli.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
# Re-plug your device
```

**Step 4: Start the Server**
```bash
./target/release/macrocli serve --port 8080
```

**Step 5: Open in Browser**

Navigate to `http://localhost:8080` and start configuring your device!

---

## Web Interface Usage

The integrated web interface provides complete device management:

### Workflow

1. **Device Detection** - Automatic detection of connected macropad
2. **Visual Editing** - Create and modify configuration layouts
3. **Validation** - Verify configuration compatibility with device
4. **Program Device** - Flash configuration to device
5. **Read from Device** - Import existing device configuration
6. **File Operations** - Export/import configurations as `.ron` files

### Data Validation Pipeline

Multi-stage validation ensures data integrity:

```
User Input → Schema Validation → Device Compatibility → Binary Encoding → Device Write
     ↓              ↓                      ↓                    ↓               ↓
  Type Check   Format Check         Hardware Check      Protocol Check    Success/Fail
```

All configuration data is validated through multiple checkpoints before device modification.

### Screenshots

<details>
<summary><b>Click to view application screenshots</b></summary>

<img width="3440" height="1440" alt="Main Interface - Device Connected" src="https://github.com/user-attachments/assets/a3fc3c0f-291b-46c5-9875-ebede984aadc" />
<img width="1807" height="1016" alt="Configuration Editor" src="https://github.com/user-attachments/assets/a0bff9cd-ad5c-4721-b07a-a164da1730e7" />
<img width="3440" height="1440" alt="Multi-Layer Configuration" src="https://github.com/user-attachments/assets/05fd2bef-ffb9-424a-8d8c-5082314d7aa6" />
<img width="1807" height="1016" alt="Validation and Programming" src="https://github.com/user-attachments/assets/c7e05e8c-5e60-42b0-a239-12fca16f24f4" />

</details>

---

## API Reference

The application runs as a REST API server when started with `macrocli serve`. All endpoints follow a consistent JSON response format with proper error handling.

### Response Format

```json
{
  "success": boolean,
  "data": object | null,
  "error": string | null
}
```

### Available Endpoints

| Method | Endpoint | Description | Authentication |
|--------|----------|-------------|----------------|
| `GET` | `/api/keys` | Retrieve all supported keys and modifiers | None |
| `GET` | `/api/device` | Check device connection status | None |
| `POST` | `/api/validate` | Validate configuration data | None |
| `POST` | `/api/program` | Program device with configuration | Device Required |
| `GET` | `/api/read?layer=N` | Read configuration from device layer | Device Required |
| `POST` | `/api/led` | Set LED color and mode | Device Required |

### Example API Calls

**Check Device Connection**
```bash
curl http://localhost:8080/api/device
```

**Validate Configuration**
```bash
curl -X POST http://localhost:8080/api/validate \
  -H "Content-Type: application/json" \
  -d @config.json
```

**Read Device Configuration**
```bash
curl http://localhost:8080/api/read?layer=0
```

### Error Handling

The API implements comprehensive error handling with detailed error messages:

- `400 Bad Request` - Invalid input data or malformed requests
- `404 Not Found` - Device not connected or resource unavailable
- `500 Internal Server Error` - Unexpected server errors

All errors include descriptive messages in the response body.

---

## Architecture

### System Design

```
┌─────────────────────────────────────────────────────────────┐
│                      Web Browser (Client)                    │
│                   React + TypeScript Frontend                │
└──────────────────────────────┬──────────────────────────────┘
                               │ HTTP/REST
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Backend Server                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   API Layer  │  │  Validation  │  │   Encoding   │     │
│  │   (Axum)     │→ │   Engine     │→ │   Engine     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└────────────────────────────┬────────────────────────────────┘
                             │ USB Protocol
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
- **Separation of Concerns** - Clear boundaries between API, validation, and device layers

---

## CLI Reference

Command-line interface for direct device operations and automation:

### Commands

```bash
# Start the integrated server
./target/release/macrocli serve --port 8080

# Program device with configuration file
./target/release/macrocli program -c config.ron

# Validate configuration without programming
./target/release/macrocli validate -c config.ron

# Read current device configuration
./target/release/macrocli read

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
- Built with [Rust](https://www.rust-lang.org/), [Axum](https://github.com/tokio-rs/axum), [React](https://reactjs.org/), and [TypeScript](https://www.typescriptlang.org/)

---

<div align="center">

[Report Bug](https://github.com/USERNAME-STOCK/Macrocli/issues) | [Request Feature](https://github.com/USERNAME-STOCK/Macrocli/issues)

</div>
