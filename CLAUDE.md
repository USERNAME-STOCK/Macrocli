# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Build and Run
```bash
# Build in release mode (creates optimized executable)
cd Macrocli
cargo build --release

# Run from target/release after building
./target/release/macrocli [command] [options]

# Build for development
cargo build

# Run tests
cargo test
```

### CLI Usage
```bash
# Show all supported keys, modifiers, and mouse actions
./target/release/macrocli show-keys

# Validate a configuration file
./target/release/macrocli validate -c config.ron

# Validate against connected device (reads device capabilities)
./target/release/macrocli validate -c config.ron --device-connected

# Validate against specific product ID
./target/release/macrocli validate -c config.ron -p 0x8840

# Program device with configuration
./target/release/macrocli program -c config.ron

# Read current configuration from device
./target/release/macrocli read --layer 1
# For K8850, may need: ./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1

# Set LED backlight mode
./target/release/macrocli led 1 1 red  # mode 1, layer 1, red color

# Start web server with API and frontend
./target/release/macrocli serve --port 8080
```

### Linux Setup (Required once)
```bash
# Install udev rules for non-root device access
sudo cp Macrocli/80-macrocli.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
# Unplug and replug device
```

## Architecture Overview

This is a Rust-based CLI tool for configuring USB macropad devices. The system emphasizes data validation, security, and device integrity through multiple validation layers.

### Core Components

**Device Layer** (`src/keyboard/`):
- Abstract `Keyboard` trait defining device interface
- Specific implementations for K884X, K8890, and K8850 devices
- Handles low-level USB communication via `rusb` crate
- Manages device discovery, endpoint detection, and protocol handling

**Configuration Engine** (`src/mapping.rs`, `src/config.rs`):
- RON-based configuration format for human-readable device mappings
- Multi-layer validation pipeline ensuring data integrity
- Support for device orientations (Normal, Clockwise, CounterClockwise, UpsideDown)
- Complex key sequences with delays, modifiers, and mouse actions

**Validation Pipeline** (`src/mapping.rs`):
- Product ID-specific validation (different devices have different capabilities)
- Key mapping validation against device capabilities
- Layer consistency checking
- Input sanitization with comprehensive error reporting

**CLI Interface** (`src/main.rs`, `src/options.rs`):
- Clap-based command parsing with comprehensive help
- Development options for debugging (hidden by default)
- Commands: validate, program, read, show-keys, led control, serve (web API)

**Web API Server** (`src/api.rs`):
- REST API with JSON request/response format
- CORS-enabled for cross-origin requests
- Serves frontend static files from `Webapp/dist/`
- Async tokio-based HTTP server using axum framework
- Temporary file handling for configuration processing

### Key Design Patterns

**Type Safety**: Extensive use of Rust's enum system for key codes, modifiers, and device-specific constants

**Device Abstraction**: Common trait interface allows adding new device types with minimal code changes

**Multi-Layer Validation**: Configuration validation occurs at multiple levels - syntax, semantic, and device-specific constraints

**Error Context**: Comprehensive error reporting using `anyhow` with context chains for debugging

### Device Support

- **K884X/K8842**: Full-featured devices supporting 17-key sequences, delays, LED colors
- **K8890**: Limited device supporting 5-key sequences, no delays, restricted media keys
- **K8850**: Extended device supporting 18-key sequences
  - **Programming**: ✅ Fully working - can program all 16 buttons + 9 knob actions (25 total)
  - **Reading**: ✅ Fully working - successfully reads all 25 keys (16 buttons + 9 knob actions) with perfect character parsing
  - **Protocol**: Uses vendor ID 0x514c (QingHeng Electronics), Magic Init Packet required for read mode

### Web API Server

The tool includes a built-in web server that provides both a REST API and serves a frontend web interface:

**API Endpoints:**
- `GET /api/keys` - Returns all supported keys, modifiers, media keys, and mouse actions
- `GET /api/device` - Checks if a compatible device is connected and returns device info
- `POST /api/validate` - Validates a configuration (accepts JSON config with optional product ID)
- `POST /api/program` - Programs a device with a configuration (accepts JSON config)
- `GET /api/read?layer=1` - Reads configuration from device (optional layer parameter)
- `POST /api/led` - Sets LED backlight mode (accepts index, layer, and optional color)

**Frontend:**
- Serves static files from `Webapp/dist/` directory
- Accessible at the same port as the API server
- Provides a web interface for device configuration

### Configuration Format

Configurations use RON (Rusty Object Notation) with this structure:
```ron
(
    device: (
        orientation: Normal,  // or Clockwise, CounterClockwise, UpsideDown
        rows: 3,
        cols: 4,
        knobs: 1,
    ),
    layers: [
        (  // Layer 1
            buttons: [
                [(delay: 0, mapping: "ctrl-x"), (delay: 100, mapping: "ctrl-s,ctrl-v")],
                // More rows...
            ],
            knobs: [
                (ccw: (delay: 0, mapping: "wheelup"), press: (delay: 0, mapping: "click"), cw: (delay: 0, mapping: "wheeldown")),
                // More knobs...
            ],
        ),
        // More layers (max 3)...
    ],
)
```

### Key Mapping Syntax

- **Single keys**: `"a"`, `"1"`, `"f1"`
- **Modifiers**: `"ctrl-a"`, `"shift-s"`, `"alt-f4"`
- **Sequences**: `"ctrl-c,ctrl-v"` (comma-separated)
- **Media keys**: `"play"`, `"next"`, `"mute"`, `"volumeup"`
- **Mouse actions**: `"click"`, `"rclick"`, `"mclick"`, `"wheelup"`, `"wheeldown"`
- **Custom codes**: `"<110>"` (decimal HID usage code)

### K8850 Implementation Notes

The K8850 device required significant reverse-engineering to implement:

**Protocol Differences:**
- Uses vendor ID 0x514c (QingHeng Electronics) instead of 0x1189
- Requires Magic Init Packet before reading: `[03, fa, 19, 00, 01, 06, 30, cc, ...]`
- Read commands use 0xFA instead of 0xFD
- Device automatically streams all 25 key responses after magic packet
- 25 control indices: 16 buttons + 9 knob actions (3 knobs × 3 actions)

**Current Status:**
- ✅ Programming: 100% functional - tested with real device
- ✅ Reading: 100% functional - successfully reads all 25 keys with perfect character parsing
- ✅ USB communication: Full bidirectional working
- ✅ Device detection and endpoint mapping working

**Reading Implementation Details:**
- Successfully sends Magic Init Packet and receives 65-byte response packets
- Reads all 25 control indices: 16 buttons + 9 knob actions (3 knobs × 3 actions)
- Character parsing working perfectly - converts HID codes to human-readable format
- Currently requires separate command for each layer (`--layer 1`, `--layer 2`, `--layer 3`)

**Debug K8850 Reading:**
```bash
# Use development options with explicit vendor/product IDs and debug logging
RUST_LOG=debug ./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1
```

### Development Options

Hidden development options are available for advanced debugging and testing:
- `--vendor-id <hex>`: Override default vendor ID (default: 0x1189)
- `--product-id <hex>`: Specify product ID (0x8840, 0x8842, 0x8890, 0x8850)
- `--address <bus:addr>`: Specify USB bus and device address
- `--out-endpoint-address <hex>`: Override OUT endpoint address
- `--in-endpoint-address <hex>`: Override IN endpoint address
- `--interface-number <num>`: Specify USB interface number

These options are hidden from normal help output but can be used for testing with specific hardware configurations.

### Next Steps: Multi-Layer Reading Enhancement

**Current Limitation:**
The K8850 reading functionality is fully working but requires separate commands for each layer:
```bash
./macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1
./macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 2
./macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 3
```

**Desired Enhancement:**
Implement single-read functionality to return all layers in one operation, similar to the configuration file format:
```bash
# Single command to read all layers
./macrocli --vendor-id 0x514c --product-id 0x8850 read --all-layers

# Or modify read command to default to all layers
./macrocli --vendor-id 0x514c --product-id 0x8850 read
```

**Expected Output Format:**
Should return a complete configuration with all 3 layers populated, matching the RON structure:
- Device metadata (orientation, rows, cols, knobs)
- All 3 layers with button and knob configurations
- Layer-specific key mappings and delays

**Implementation Approach:**
1. **Modify K8850 read logic** in `src/keyboard/k8850.rs` to iterate through all layers in single call
2. **Update CLI options** in `src/options.rs` to support `--all-layers` flag
3. **Enhance output formatting** to structure multi-layer data in proper RON format
4. **Optimize USB communication** to minimize device initialization overhead

**Technical Considerations:**
- Device currently responds with 65-byte packets per key per layer
- Need to efficiently sequence layer reads without reinitializing USB connection
- Consider caching device state and magic init packet to reduce overhead
- Ensure backward compatibility with existing `--layer N` commands

### Testing Reading Command Implementation

The reading functionality, especially for K8850 devices, can be tested using the following approaches:

**1. Basic Reading Tests:**
```bash
# Test with default device detection
./target/release/macrocli read --layer 1

# Test with explicit K8850 device (most common for reading issues)
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1

# Test all layers (1, 2, 3)
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 2
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 3
```

**2. Debug Mode Testing:**
```bash
# Enable debug logging to see USB communication details
RUST_LOG=debug ./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1

# Test with different device addresses if needed
./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 --address 1:2 read --layer 1
```

**3. Unit Testing:**
```bash
# Run all tests including reading functionality tests
cargo test

# Run specific reading-related tests
cargo test read
cargo test k8850
```

**4. Integration Testing Flow:**
1. **Program Test Configuration**: First program a known configuration to test against
   ```bash
   ./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 program -c test_config.ron
   ```

2. **Read Back Configuration**: Attempt to read the programmed configuration
   ```bash
   ./target/release/macrocli --vendor-id 0x514c --product-id 0x8850 read --layer 1 > read_output.txt
   ```

3. **Validate Read Data**: Compare read data with expected configuration
   ```bash
   # Manual verification of output format and content
   cat read_output.txt
   ```

**5. Expected Behavior for Testing:**
- **K884X/K8842**: Should read 17 keys + knob data successfully
- **K8890**: Should read 5 keys successfully
- **K8850**: ✅ Fully working - reads all 25 keys (16 buttons + 9 knob actions) with perfect character parsing

**6. Common Testing Issues and Solutions:**
- **Device not found**: Use `--vendor-id 0x514c --product-id 0x8850` for K8850 devices
- **Permission denied**: On Linux, ensure udev rules are installed (see Linux Setup section)
- **Timeout errors**: Try with `--address <bus:addr>` to specify exact device location
- **Garbled output**: Expected for K8850 - this indicates the core issue to resolve

**7. Test Data Verification:**
When testing reading implementation, verify:
- Packet structure (65-byte packets)
- Response packet headers and magic init sequences
- Character encoding and parsing logic
- Layer-specific data extraction
- Button vs knob action differentiation

### USB Protocol

The tool communicates with devices using USB interrupt transfers:
- 65-byte packet size standard across all devices
- Device-specific command protocols for configuration programming
- Timeout-based communication with error handling
- Support for both read and write endpoints

## Important Constants and Limits

- **Vendor ID**: 0x1189 (primary), 0x514c (QingHeng Electronics)
- **Product IDs**: 0x8840, 0x8842, 0x8890, 0x8850
- **Maximum layers**: 3
- **Maximum delay**: 6000ms (except K8890 which doesn't support delays)
- **USB timeout**: 100ms
- **Packet size**: 65 bytes

### Dependencies

**Core Dependencies:**
- `rusb`: USB device communication and protocol handling
- `clap`: Command-line argument parsing with derive macros
- `ron`: Human-readable configuration serialization (Rusty Object Notation)
- `serde`: JSON/RON serialization and deserialization
- `anyhow`: Error handling with context chains
- `tokio`: Async runtime for web server functionality
- `axum`: Web framework for REST API
- `tower-http`: HTTP middleware (CORS, static file serving)

**Utility Dependencies:**
- `strum`: Enum serialization and iteration macros
- `num`: Traits for numeric type conversions
- `itertools`: Iterator utilities for data processing
- `env_logger`: Structured logging output
- `nom`: Parser combinators for address parsing