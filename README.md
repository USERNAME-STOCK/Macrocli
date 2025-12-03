<div align="center">

# Macrocli - Advanced Macropad Configuration System

**A high-performance CLI tool for configuring, validating, and managing USB macropad devices.**

</div>

## Supported Devices

| Device Model    | Vendor ID | Product ID    | Key Features                                            |
| --------------- | --------- | ------------- | ------------------------------------------------------- |
| **K884X/K8842** | 0x1189    | 0x8840/0x8842 | 17-key macros, LED control, Global Delays               |
| **K8890**       | 0x1189    | 0x8890        | 5-key macros, Media keys, No delays                     |
| **K8850**       | 0x514c    | 0x8850        | **18-key macros**, **Per-Key Delays (0-6s)**, 3 Layers  |

### Device Capabilities

- **K8850 (QingHeng)**:
  - Full support for **18 keys per macro** (buttons & knobs).
  - **Precise Delay Control**: 0ms to 6000ms range.
  - **Per-Key Delays**: Define specific delays for each step in a macro sequence.
  - **Multi-Layer**: Reads/Writes all 3 layers simultaneously.
  - 16 Buttons + 3 Knobs (CW/CCW/Press) = 25 programmable inputs per layer.

- **K884X (0x8840/0x8842)**:
  - Standard 17-key sequence support.
  - **Global Delay**: Supports a single delay setting per macro button.
  - **Read Support**: Can read back configuration from device.
  - Per-layer LED color configuration.

- **K8890**:
  - Basic 5-key sequences.
  - Multimedia key support.

---

## Installation

### Prerequisites
- Rust toolchain (1.70+)

### Build
```bash
cd Macrocli
cargo build --release
```
The binary will be available at `target/release/macrocli`.

### Linux Permissions (Required for non-root access)
```bash
sudo cp 80-macrocli.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
# Re-plug your device
```

---

## Usage

### Common Commands

**Show connected device and keys:**
```bash
./macrocli show-keys
```

**Read current configuration:**
```bash
# Auto-detects device and reads config
./macrocli read
```

**Program device:**
```bash
./macrocli program -c my_config.ron
```

**Validate configuration file:**
```bash
./macrocli validate -c my_config.ron
```

**Control LEDs (K884X only):**
```bash
# Set Layer 1 LED to Red (Index 1)
./macrocli led 1 1
```

### Configuration Format (.ron)

Configurations use the **RON** (Rust Object Notation) format.

**Example (K8850/K884X):**
```rust
(
    device: (
        orientation: Normal,
        rows: 4, cols: 4, knobs: 3,
    ),
    layers: [
        (
            buttons: [
                // Row 1
                [
                    // Simple delay for the whole macro
                    (delay: 0, mapping: "Ctrl+c"),

                    // Advanced: Per-key delays (K8850 only)
                    // Format: [delay_before_key1, delay_before_key2, ...]
                    (
                        delay: 0, // Fallback/Initial delay
                        per_key_delays: [10, 500, 10],
                        mapping: "a+b+c"
                    ),

                    (delay: 50, mapping: "Ctrl+v"),
                ],
                // ... other rows
            ],
            knobs: [
                (
                    ccw: (delay: 10, mapping: "VolumeDown"),
                    press: (delay: 0, mapping: "Mute"),
                    cw: (delay: 10, mapping: "VolumeUp")
                ),
                // ... other knobs
            ]
        ),
        // ... Layer 2, Layer 3
    ]
)
```

---

## Roadmap

- [ ] Reverse engineer LED control support for K8850 and K8890 devices.
- [ ] Add GUI for configuration.

## Troubleshooting

- **"Device not found"**: Ensure you have permissions (check Linux Setup) or run as sudo/admin.
- **"Validation failed"**: Check your `.ron` file syntax. Ensure you aren't exceeding the max keys per macro (18 for K8850, 17 for K884X, 5 for K8890).
- **K8850 Programming**: Ensure you provide full 3-layer configuration as the device expects all layers to be written.

## License

Creative Commons Attribution-ShareAlike 3.0 Unported License.
