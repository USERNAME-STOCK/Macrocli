# `macrocli` - Macropad Programmer and Visualizer

This project provides a suite of tools to program specific macropad devices (Vendor ID `0x1189`, Product IDs `0x8840`, `0x8842`, `0x8890`). It consists of:

1.  **`macrocli`**: A Rust-based command-line tool with integrated web server for device programming
2.  **Webapp Visualizer**: A web-based interface for visually creating, editing, and programming macropad configurations
3.  **REST API**: Backend API for device operations (validate, program, read, LED control)

## Repository Structure

The main components of this repository are:

*   `Macrocli/`: Contains the core Rust project for the macropad programming tool.
    *   `src/`: Source code for the Rust application.
    *   `target/`: Compiled executables and libraries. The final binary is at `target/release/macrocli`.
    *   `macropad_configs/`: Directory for storing custom macropad configuration files (`.ron` format).
    *   `macropad_backups/`: Directory for storing backup configurations read from devices.
    *   `Webapp/`: Contains the React/TypeScript project for the visual configuration editor.
    *   `80-macrocli.rules`: A `udev` rule for granting non-root users access to the USB device on Linux.
    *   `Cargo.toml`: The Rust project manifest.
*   `README.md`: The main project documentation file (this file).

## Recommended Workflow: Integrated Web Interface (NEW!)

The easiest way to use macrocli is through the integrated web interface that combines the visualizer with direct device programming capabilities.

### Quick Start

1.  **Build the integrated server** (only needs to be done once):
    ```bash
    cd Macrocli/
    cargo build --release
    ```

2.  **Build the web interface** (only needs to be done once):
    ```bash
    cd Webapp/
    npm install
    npm run build
    cd ..
    ```

3.  **Start the integrated server**:
    ```bash
    ./target/release/macrocli serve --port 8080
    ```

4.  **Open your browser** to `http://localhost:8080`

### Using the Integrated Interface

The integrated interface provides a seamless experience:

1.  **Device Status**: The interface automatically detects when your macropad is connected
2.  **Create/Edit Layouts**: Use the visual editor to create one or more configuration layers (profiles)
3.  **Validate**: Click the **Validate** button to check if your configuration is compatible with your device
4.  **Program Device**: Click the **Program Device** button to directly flash your configuration to the device
5.  **Read from Device**: Click the **Read from Device** button to import the current configuration from your device
6.  **Export/Import**: Use **Export File** and **Import File** to save/load configurations as `.ron` files

<img width="3440" height="1440" alt="image" src="https://github.com/user-attachments/assets/a3fc3c0f-291b-46c5-9875-ebede984aadc" />
<img width="1807" height="1016" alt="image" src="https://github.com/user-attachments/assets/a0bff9cd-ad5c-4721-b07a-a164da1730e7" />
<img width="3440" height="1440" alt="image" src="https://github.com/user-attachments/assets/05fd2bef-ffb9-424a-8d8c-5082314d7aa6" />
<img width="1807" height="1016" alt="image" src="https://github.com/user-attachments/assets/c7e05e8c-5e60-42b0-a239-12fca16f24f4" />

### Features

- ✅ Real-time device connection detection
- ✅ Visual configuration editor with 3-layer support
- ✅ Direct device programming from the web interface
- ✅ Configuration validation against connected device
- ✅ Read configurations from device
- ✅ Import/Export `.ron` configuration files
- ✅ No need to manually run CLI commands

## API Endpoints

When running in server mode (`macrocli serve`), the following REST API endpoints are available:

- `GET /api/keys` - Get all supported keys and modifiers
- `GET /api/device` - Check if a device is connected
- `POST /api/validate` - Validate a configuration
- `POST /api/program` - Program the device with a configuration
- `GET /api/read?layer=N` - Read configuration from device
- `POST /api/led` - Set LED color and mode

All endpoints return JSON responses in the format:
```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

## Advanced Usage: CLI-Only

For advanced users, you can manually edit the `.ron` configuration files and use the CLI tool directly.

### Available CLI Commands

- **Start integrated server**:
  ```bash
  ./Macrocli/target/release/macrocli serve --port 8080
  ```

- **Program a device**:
  ```bash
  ./Macrocli/target/release/macrocli program -c ./Macrocli/macropad_configs/your_config.ron
  ```

- **Validate a config file**:
  ```bash
  ./Macrocli/target/release/macrocli validate -c ./Macrocli/macropad_configs/your_config.ron
  ```

- **Read configuration from device**:
  ```bash
  ./Macrocli/target/release/macrocli read
  ```

- **List Supported Keys**:
  ```bash
  ./Macrocli/target/release/macrocli show-keys
  ```

- **Set LED mode**:
  ```bash
  ./Macrocli/target/release/macrocli led <index> <layer> [color]
  ```

## Setup for Linux (`udev` rules)

To run `macrocli` without `sudo`, you need to set up a `udev` rule.

1.  Copy the `80-macrocli.rules` file to `/etc/udev/rules.d/`.
    ```bash
    sudo cp Macrocli/80-macrocli.rules /etc/udev/rules.d/
    ```
2.  Reload the `udev` rules.
    ```bash
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    ```
3.  Re-plug your macropad device.

## Acknowledgements

The Rust-based CLI tool (`macrocli`) was inspired by the work of [eccherda/ch552g_mini_keyboard](https://github.com/eccherda/ch552g_mini_keyboard), which provides firmware and a programming tool for similar CH552G-based hardware.

## License

This project is licensed under the Creative Commons Attribution-ShareAlike 3.0 Unported License. See the [LICENSE](LICENSE) file for details.
