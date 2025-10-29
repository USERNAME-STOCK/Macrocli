# `macrocli` - Macropad Programmer and Visualizer

This project provides a suite of tools to program specific macropad devices (Vendor ID `0x1189`, Product IDs `0x8840`, `0x8842`, `0x8890`). It consists of:

1.  **`macrocli`**: A Rust-based command-line tool for flashing configurations to the device, located in the `Macrocli/` directory.
2.  **Webapp Visualizer**: A web-based interface for visually creating and editing macropad layouts, located in the `Macrocli/Webapp/` directory.

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

## Recommended Workflow: Web Visualizer

The easiest way to get started is by using the web visualizer to create your configuration.

### Step 1: Run the Web Visualizer

1.  **Navigate to the web app directory:**
    ```bash
    cd Macrocli/Webapp/
    ```
2.  **Install dependencies:**
    ```bash
    npm install
    ```
3.  **Run the development server:**
    ```bash
    npm run dev
    ```
4.  Open your browser to the local URL provided (e.g., `http://localhost:3000`).

### Step 2: Create and Export Your Layout

1.  **Create Layout**: Use the web interface to create one or more configuration layers (profiles). The application starts with a set of generic default profiles.
2.  **Export Config**: Click the **Export .ron** button. This will download a `macropad_config.ron` file containing all the layers you've configured.
3.  **Move Config File**: Move the downloaded `macropad_config.ron` into the `Macrocli/macropad_configs/` directory.

### Step 3: Build and Program with `macrocli`

Now, use the command-line tool to flash the configuration you just created.

1.  **Build the CLI tool** (only needs to be done once):
    ```bash
    cd Macrocli/
    cargo build --release
    cd ..
    ```
2.  **Program the Device**: From the project root, run the program command:
    ```bash
    ./Macrocli/target/release/macrocli program -c ./Macrocli/macropad_configs/macropad_config.ron
    ```
3.  **Verify**: You can read the configuration back from the device to confirm it was programmed successfully:
    ```bash
    ./Macrocli/target/release/macrocli read
    ```

## Advanced Usage: CLI-Only

For advanced users, you can manually edit the `.ron` configuration files and use the CLI tool directly.

- **Validate a config file**:
  ```bash
  ./Macrocli/target/release/macrocli validate -c ./Macrocli/macropad_configs/your_config.ron
  ```
- **List Supported Keys**:
  ```bash
  ./Macrocli/target/release/macrocli show-keys
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
