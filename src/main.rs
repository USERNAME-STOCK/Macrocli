mod config;
mod consts;
mod decoder;
mod keyboard;
mod mapping;
mod options;
mod parse;

use crate::consts::PRODUCT_IDS;
use crate::decoder::Decoder;
use crate::keyboard::{
    k884x, k8850, k8890, Keyboard, MediaCode, Modifier, MouseAction, MouseButton, WellKnownCode,
};
use crate::mapping::Macropad;
use crate::options::Options;
use crate::options::{Command, LedCommand, LedPerKeyCommand};

use anyhow::{anyhow, ensure, Result};
use indoc::indoc;
use itertools::Itertools;
use keyboard::LedColor;
use log::debug;
use mapping::Mapping;
use rusb::{Context, Device, DeviceDescriptor, Direction, TransferType};

use anyhow::Context as _;
use clap::Parser as _;
use rusb::UsbContext as _;
use strum::EnumMessage as _;
use strum::IntoEnumIterator as _;

fn main() -> Result<()> {
    env_logger::init();
    let options = Options::parse();
    debug!("options: {:?}", options.devel_options);

    match &options.command {
        Command::ShowKeys => {
            println!("Modifiers: ");
            for m in Modifier::iter() {
                println!(" - {}", m.get_serializations().iter().join(" / "));
            }

            println!();
            println!("Keys:");
            for c in WellKnownCode::iter() {
                println!(" - {c}");
            }

            println!();
            println!("Custom key syntax (use decimal code): <110>");

            println!();
            println!("Media keys:");
            for c in MediaCode::iter() {
                println!(" - {}", c.get_serializations().iter().join(" / "));
            }

            println!();
            println!("Mouse actions:");
            println!(" - {}", MouseAction::WheelDown);
            println!(" - {}", MouseAction::WheelUp);
            for b in MouseButton::iter() {
                println!(" - {b}");
            }
        }

        Command::Validate {
            config_file,
            product_id,
            device_connected,
        } => {
            if *device_connected {
                debug!("validating with connected device");
                if let Ok(device) = find_device(consts::VENDOR_ID, None) {
                    // read the config for buttons/knobs and validate against file
                    if device.2 != 0x8890 {
                        // 0x8890 does not support reading configuration
                        let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
                        let mut buf = vec![0; consts::READ_BUF_SIZE.into()];

                        // get the type of device
                        keyboard.send(&keyboard.device_type())?;
                        let bytes_read = keyboard.recieve(&mut buf)?;
                        if bytes_read == 0 {
                            return Err(anyhow!(
                                "Unable to read from device to validate mappings. Please use -p option instead to specify your device."
                            ));
                        }
                        let device_info = Decoder::get_device_info(&buf);
                        debug!(
                            "keys: {} encoders: {}",
                            device_info.num_keys, device_info.num_encoders
                        );

                        let macropad = Mapping::read(config_file);
                        if device_info.num_keys != macropad.device.rows * macropad.device.cols {
                            return Err(anyhow!(
                                "Number of keys specified in config does not match device"
                            ));
                        }
                        if device_info.num_encoders != macropad.device.knobs {
                            return Err(anyhow!(
                                "Number of knobs specified in config does not match device"
                            ));
                        }
                    }
                    Mapping::validate(config_file, Some(device.2))
                        .context("validating configuration file with connected device")?;
                    println!("config is valid 👌")
                } else {
                    return Err(anyhow!(
                        "Unable to find connected device with vendor id: 0x{:02x}",
                        consts::VENDOR_ID
                    ));
                }
            } else if let Some(pid) = product_id {
                debug!("validating with supplied product id 0x{pid:02x}");
                Mapping::validate(config_file, Some(*pid))
                    .context("validating configuration file against specified product id")?;
                println!("config is valid 👌")
            } else {
                // load and validate mapping
                println!("validating general ron formatting - unable to do more granular checking; use -p option to check against device");
                Mapping::validate(config_file, None)
                    .context("generic validation of configuration file")?;
                println!("config is valid 👌")
            }
        }

        Command::Program { config_file } => {
            let config = Mapping::read(config_file);
            let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
            keyboard.program(&config).context("programming macropad")?;
            println!("successfully programmed device");
        }

        Command::Led(LedCommand {
            index,
            layer,
            led_color,
        }) => {
            let mut keyboard = open_keyboard(&options).context("opening keyboard")?;

            // color is not supported on 0x8890 so don't require one to be passed
            let color = if led_color.is_some() {
                led_color.unwrap()
            } else {
                LedColor::Red
            };
            keyboard
                .set_led(*index, *layer, color)
                .context("programming LED on macropad")?;
        }

        Command::LedPerKey(LedPerKeyCommand {
            mode,
            layer,
            colors,
        }) => {
            let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
            let rgb: Vec<(u8, u8, u8)> = colors.iter().map(|c| (c.r, c.g, c.b)).collect();

            keyboard
                .set_led_per_key(*mode, *layer, &rgb)
                .context("programming per-key RGB on macropad")?;

            println!(
                "programmed {} RGB slot(s) on LED layer {} using mode {}",
                rgb.len(),
                layer,
                mode
            );
        }
        Command::Read { layer, all_layers } => {
            debug!("dev options: {:?}", options.devel_options);
            let layer_to_read = if *all_layers { 0 } else { *layer };
            let mut keyboard = open_keyboard(&options).context("opening keyboard")?;
            let macropad_config = keyboard
                .read_macropad_config(&layer_to_read)
                .context("reading macropad configuration")?;
            Mapping::print(macropad_config);
        }

        Command::RepairLayer1Factory { execute } => {
            if !*execute {
                println!("DRY RUN: nothing written.");
                println!("This command will restore Layer 1 only:");
                println!("  slot  1 -> A");
                println!("  slot  2 -> B");
                println!("  slot  3 -> C");
                println!("  slot 16 -> P  (knob CCW)");
                println!("  slot 17 -> Q  (knob press)");
                println!("  slot 18 -> R  (knob CW)");
                println!();
                println!("Run again with --execute to actually write.");
            } else {
                repair_k8850_layer1_factory(&options)?;
            }
        }

        Command::TestLayer1F13 { execute } => {
            if !*execute {
                println!("DRY RUN: nothing written.");
                println!("This command will change ONLY Layer 1 physical keys:");
                println!("  slot 1 -> F13 (HID 0x68)");
                println!("  slot 2 -> F14 (HID 0x69)");
                println!("  slot 3 -> F15 (HID 0x6A)");
                println!();
                println!("Knob slots 16/17/18 are NOT touched.");
                println!("Layer 2/3 and all other slots are NOT touched.");
                println!();
                println!("Run again with --execute to actually write.");
            } else {
                test_k8850_layer1_f13(&options)?;
            }
        }

        Command::SetLayer1MediaKnob { execute } => {
            if !*execute {
                println!("DRY RUN: nothing written.");
                println!("This command will change ONLY the Layer 1 physical knob:");
                println!("  slot 16 -> Volume Down (Consumer 0x00EA)");
                println!("  slot 17 -> Mute        (Consumer 0x00E2)");
                println!("  slot 18 -> Volume Up   (Consumer 0x00E9)");
                println!();
                println!("F13/F14/F15 slots 1/2/3 are NOT touched.");
                println!("Layer 2/3 and all other slots are NOT touched.");
                println!();
                println!("Run again with --execute to actually write.");
            } else {
                set_k8850_layer1_media_knob(&options)?;
            }
        }
    }

    Ok(())
}

pub fn find_interface_and_endpoint(
    device: &Device<Context>,
    interface_num: Option<u8>,
    endpoint_addr_out: Option<u8>,
    endpoint_addr_in: Option<u8>,
) -> Result<(u8, u8, u8)> {
    debug!("out: {endpoint_addr_out:?} in: {endpoint_addr_in:?}");
    let conf_desc = device
        .config_descriptor(0)
        .context("get config #0 descriptor")?;

    // Get the numbers of interfaces to explore
    let interface_nums = match interface_num {
        Some(iface_num) => vec![iface_num],
        None => conf_desc.interfaces().map(|iface| iface.number()).collect(),
    };

    // per usb spec, the max value for a usb endpoint is 7 bits (or 127)
    // so set the values to be invalid by default
    let mut out_if = 0xFF;
    let mut in_if = 0xFF;
    for iface_num in interface_nums {
        debug!("Probing interface {iface_num}");

        // Look for an interface with the given number
        let intf = conf_desc
            .interfaces()
            .find(|iface| iface_num == iface.number())
            .ok_or_else(|| {
                anyhow!(
                    "interface #{} not found, interface numbers:\n{:#?}",
                    iface_num,
                    conf_desc.interfaces().map(|i| i.number()).format(", ")
                )
            })?;

        // Check that it's a HID device
        let intf_desc = intf.descriptors().exactly_one().map_err(|_| {
            anyhow!(
                "only one interface descriptor is expected, got:\n{:#?}",
                intf.descriptors().format("\n")
            )
        })?;

        let descriptors = intf_desc.endpoint_descriptors();
        for endpoint in descriptors {
            // check packet size
            if endpoint.max_packet_size() != ((consts::PACKET_SIZE - 1) as u16) {
                continue;
            }

            debug!("==> {:?} direction: {:?}", endpoint, endpoint.direction());
            if endpoint.transfer_type() == TransferType::Interrupt
                && endpoint.direction() == Direction::Out
            {
                if let Some(ea) = endpoint_addr_out {
                    if endpoint.address() == ea {
                        debug!("Found OUT endpoint {endpoint:?}");
                        out_if = endpoint.address();
                    }
                } else {
                    debug!("Found OUT endpoint {endpoint:?}");
                    out_if = endpoint.address();
                }
            }
            if endpoint.transfer_type() == TransferType::Interrupt
                && endpoint.direction() == Direction::In
            {
                if let Some(ea) = endpoint_addr_in {
                    if endpoint.address() == ea {
                        debug!("Found IN endpoint {endpoint:?}");
                        in_if = endpoint.address();
                    }
                } else {
                    debug!("Found IN endpoint {endpoint:?}");
                    in_if = endpoint.address();
                }
            }
        }
        debug!("ep OUT addr: 0x{out_if:02x} ep IN addr: 0x{in_if:02x}");
        if out_if < 0xFF && in_if < 0xFF {
            return Ok((iface_num, out_if, in_if));
        } else if out_if < 0xFF {
            return Ok((iface_num, out_if, 0xFF));
        }
    }

    Err(anyhow!("No valid interface/endpoint combination found!"))
}

fn set_k8850_layer1_media_knob(options: &Options) -> Result<()> {
    ensure!(
        options.devel_options.vendor_id == 0x514c,
        "refusing write: --vendor-id must explicitly be 0x514c"
    );
    ensure!(
        options.devel_options.product_id == Some(0x8850),
        "refusing write: --product-id must explicitly be 0x8850"
    );

    let (device, desc, id_product) = find_device(
        options.devel_options.vendor_id,
        options.devel_options.product_id,
    )
    .context("find 514c:8850 USB device")?;

    ensure!(
        desc.vendor_id() == 0x514c,
        "refusing write: detected vendor id is 0x{:04x}, expected 0x514c",
        desc.vendor_id()
    );
    ensure!(
        id_product == 0x8850 && desc.product_id() == 0x8850,
        "refusing write: detected product id is 0x{:04x}, expected 0x8850",
        desc.product_id()
    );
    ensure!(
        desc.num_configurations() == 1,
        "refusing write: unexpected number of USB configurations"
    );

    let (intf_num, endpt_addr_out, endpt_addr_in) = find_interface_and_endpoint(
        &device,
        options.devel_options.interface_number,
        options.devel_options.out_endpoint_address,
        options.devel_options.in_endpoint_address,
    )?;

    ensure!(
        intf_num == 0,
        "refusing write: unexpected interface {}, expected interface 0",
        intf_num
    );
    ensure!(
        endpt_addr_out == 0x04,
        "refusing write: unexpected OUT endpoint 0x{:02x}, expected 0x04",
        endpt_addr_out
    );
    ensure!(
        endpt_addr_in == 0x84,
        "refusing write: unexpected IN endpoint 0x{:02x}, expected 0x84",
        endpt_addr_in
    );

    println!(
        "Matched 514c:8850 on interface {}, OUT=0x{:02x}, IN=0x{:02x}",
        intf_num, endpt_addr_out, endpt_addr_in
    );

    let handle = device.open().context("open USB device")?;
    let _ = handle.set_auto_detach_kernel_driver(true);
    handle
        .claim_interface(intf_num)
        .context("claim USB interface")?;

    // Native K8850 0xfd media binding:
    //
    //   03 fd <slot> <layer> 02 00 02
    //   00 00 <consumer-usage-low>
    //   00 00 <consumer-usage-high>
    //
    // Consumer usages:
    //   Mute        = 0x00E2
    //   Volume Up   = 0x00E9
    //   Volume Down = 0x00EA
    //
    // We touch ONLY the three physical knob slots on Layer 1:
    //   16 = CCW, 17 = press, 18 = CW.
    let bindings: [(u8, u16, &str); 3] = [
        (0x10, 0x00ea, "Knob CCW -> Volume Down"),
        (0x11, 0x00e2, "Knob Press -> Mute"),
        (0x12, 0x00e9, "Knob CW -> Volume Up"),
    ];

    println!("Writing exactly three Layer 1 media-knob bindings...");

    for (slot, usage, description) in bindings {
        let [low, high] = usage.to_le_bytes();

        let mut msg = vec![0u8; consts::PACKET_SIZE];
        let header = [
            0x03, // report ID
            0xfd, // native K8850 write
            slot, // firmware slot
            0x01, // Layer 1
            0x02, // media / consumer-control macro
            0x00, // modifier group count
            0x02, // two 3-byte groups carry the 16-bit usage
            0x00, 0x00, low, 0x00, 0x00, high,
        ];
        msg[..header.len()].copy_from_slice(&header);

        let written = handle
            .write_interrupt(endpt_addr_out, &msg, consts::DEFAULT_TIMEOUT)
            .with_context(|| format!("write Layer 1 slot {} ({})", slot, description))?;

        ensure!(
            written == msg.len(),
            "short USB write for slot {}: wrote {} of {} bytes",
            slot,
            written,
            msg.len()
        );

        println!(
            "  wrote slot {:2} (0x{:02x}): {} [Consumer 0x{:04X}]",
            slot, slot, description, usage
        );
    }

    let mut commit = vec![0u8; consts::PACKET_SIZE];
    commit[..4].copy_from_slice(&[0x03, 0xfd, 0xfe, 0xff]);

    let written = handle
        .write_interrupt(endpt_addr_out, &commit, consts::DEFAULT_TIMEOUT)
        .context("commit Layer 1 media-knob bindings")?;

    ensure!(
        written == commit.len(),
        "short USB write while committing: wrote {} of {} bytes",
        written,
        commit.len()
    );

    println!("Commit sent.");
    println!("Layer 1 media knob write completed.");
    Ok(())
}

fn test_k8850_layer1_f13(options: &Options) -> Result<()> {
    ensure!(
        options.devel_options.vendor_id == 0x514c,
        "refusing write: --vendor-id must explicitly be 0x514c"
    );
    ensure!(
        options.devel_options.product_id == Some(0x8850),
        "refusing write: --product-id must explicitly be 0x8850"
    );

    let (device, desc, id_product) = find_device(
        options.devel_options.vendor_id,
        options.devel_options.product_id,
    )
    .context("find 514c:8850 USB device")?;

    ensure!(
        desc.vendor_id() == 0x514c,
        "refusing write: detected vendor id is 0x{:04x}, expected 0x514c",
        desc.vendor_id()
    );
    ensure!(
        id_product == 0x8850 && desc.product_id() == 0x8850,
        "refusing write: detected product id is 0x{:04x}, expected 0x8850",
        desc.product_id()
    );
    ensure!(
        desc.num_configurations() == 1,
        "refusing write: unexpected number of USB configurations"
    );

    let (intf_num, endpt_addr_out, endpt_addr_in) = find_interface_and_endpoint(
        &device,
        options.devel_options.interface_number,
        options.devel_options.out_endpoint_address,
        options.devel_options.in_endpoint_address,
    )?;

    ensure!(
        intf_num == 0,
        "refusing write: unexpected interface {}, expected interface 0",
        intf_num
    );
    ensure!(
        endpt_addr_out == 0x04,
        "refusing write: unexpected OUT endpoint 0x{:02x}, expected 0x04",
        endpt_addr_out
    );
    ensure!(
        endpt_addr_in == 0x84,
        "refusing write: unexpected IN endpoint 0x{:02x}, expected 0x84",
        endpt_addr_in
    );

    println!(
        "Matched 514c:8850 on interface {}, OUT=0x{:02x}, IN=0x{:02x}",
        intf_num, endpt_addr_out, endpt_addr_in
    );

    let handle = device.open().context("open USB device")?;
    let _ = handle.set_auto_detach_kernel_driver(true);
    handle
        .claim_interface(intf_num)
        .context("claim USB interface")?;

    // ONLY the three physical Layer-1 key slots are changed.
    // HID keyboard usages:
    //   F13 = 0x68
    //   F14 = 0x69
    //   F15 = 0x6A
    let bindings: [(u8, u8, &str); 3] = [
        (0x01, 0x68, "Key1 -> F13"),
        (0x02, 0x69, "Key2 -> F14"),
        (0x03, 0x6a, "Key3 -> F15"),
    ];

    println!("Writing exactly three Layer 1 test bindings...");

    for (slot, hid_code, description) in bindings {
        let mut msg = vec![0u8; consts::PACKET_SIZE];
        let header = [
            0x03,     // report ID
            0xfd,     // native K8850 write
            slot,     // firmware slot
            0x01,     // Layer 1
            0x01,     // keyboard binding
            0x00,     // modifier count
            0x01,     // one 3-byte group
            0x00,     // delay high
            0x00,     // delay low
            hid_code, // HID keyboard usage
        ];
        msg[..header.len()].copy_from_slice(&header);

        let written = handle
            .write_interrupt(endpt_addr_out, &msg, consts::DEFAULT_TIMEOUT)
            .with_context(|| format!("write Layer 1 slot {} ({})", slot, description))?;

        ensure!(
            written == msg.len(),
            "short USB write for slot {}: wrote {} of {} bytes",
            slot,
            written,
            msg.len()
        );

        println!("  wrote slot {:2} (0x{:02x}): {}", slot, slot, description);
    }

    let mut commit = vec![0u8; consts::PACKET_SIZE];
    commit[..4].copy_from_slice(&[0x03, 0xfd, 0xfe, 0xff]);

    let written = handle
        .write_interrupt(endpt_addr_out, &commit, consts::DEFAULT_TIMEOUT)
        .context("commit F13/F14/F15 Layer 1 test bindings")?;

    ensure!(
        written == commit.len(),
        "short USB write while committing: wrote {} of {} bytes",
        written,
        commit.len()
    );

    println!("Commit sent.");
    println!("F13/F14/F15 Layer 1 test write completed.");
    Ok(())
}

fn repair_k8850_layer1_factory(options: &Options) -> Result<()> {
    // This helper is intentionally narrow. It only restores the six physical
    // controls observed on this specific 3-key + 1-knob 514c:8850 variant.
    ensure!(
        options.devel_options.vendor_id == 0x514c,
        "refusing write: --vendor-id must explicitly be 0x514c"
    );
    ensure!(
        options.devel_options.product_id == Some(0x8850),
        "refusing write: --product-id must explicitly be 0x8850"
    );

    let (device, desc, id_product) = find_device(
        options.devel_options.vendor_id,
        options.devel_options.product_id,
    )
    .context("find 514c:8850 USB device")?;

    ensure!(
        desc.vendor_id() == 0x514c,
        "refusing write: detected vendor id is 0x{:04x}, expected 0x514c",
        desc.vendor_id()
    );
    ensure!(
        id_product == 0x8850 && desc.product_id() == 0x8850,
        "refusing write: detected product id is 0x{:04x}, expected 0x8850",
        desc.product_id()
    );
    ensure!(
        desc.num_configurations() == 1,
        "refusing write: unexpected number of USB configurations"
    );

    let (intf_num, endpt_addr_out, endpt_addr_in) = find_interface_and_endpoint(
        &device,
        options.devel_options.interface_number,
        options.devel_options.out_endpoint_address,
        options.devel_options.in_endpoint_address,
    )?;

    // These exact endpoints were observed on the user's physical unit.
    ensure!(
        intf_num == 0,
        "refusing write: unexpected interface {}, expected interface 0",
        intf_num
    );
    ensure!(
        endpt_addr_out == 0x04,
        "refusing write: unexpected OUT endpoint 0x{:02x}, expected 0x04",
        endpt_addr_out
    );
    ensure!(
        endpt_addr_in == 0x84,
        "refusing write: unexpected IN endpoint 0x{:02x}, expected 0x84",
        endpt_addr_in
    );

    println!(
        "Matched 514c:8850 on interface {}, OUT=0x{:02x}, IN=0x{:02x}",
        intf_num, endpt_addr_out, endpt_addr_in
    );

    let handle = device.open().context("open USB device")?;
    let _ = handle.set_auto_detach_kernel_driver(true);
    handle
        .claim_interface(intf_num)
        .context("claim USB interface")?;

    // Native 0xfd keyboard-binding format:
    //   03 fd <slot> <layer> 01 00 01 00 00 <hid-code> ...
    //
    // Only Layer 1 and only the six physical slots are touched.
    //
    // Factory references copied from the intact Layer 2/3 raw mappings:
    //   slot  1 = A = HID 0x04
    //   slot  2 = B = HID 0x05
    //   slot  3 = C = HID 0x06
    //   slot 16 = P = HID 0x13  (knob CCW)
    //   slot 17 = Q = HID 0x14  (knob press)
    //   slot 18 = R = HID 0x15  (knob CW)
    let bindings: [(u8, u8, &str); 6] = [
        (0x01, 0x04, "Key1 -> A"),
        (0x02, 0x05, "Key2 -> B"),
        (0x03, 0x06, "Key3 -> C"),
        (0x10, 0x13, "Knob CCW -> P"),
        (0x11, 0x14, "Knob Press -> Q"),
        (0x12, 0x15, "Knob CW -> R"),
    ];

    println!("Writing exactly six Layer 1 recovery bindings...");

    for (slot, hid_code, description) in bindings {
        let mut msg = vec![0u8; consts::PACKET_SIZE];
        let header = [
            0x03,     // report ID
            0xfd,     // native K8850 write
            slot,     // firmware slot
            0x01,     // Layer 1
            0x01,     // keyboard binding
            0x00,     // modifier count
            0x01,     // one 3-byte group
            0x00,     // delay high
            0x00,     // delay low
            hid_code, // HID keyboard usage
        ];
        msg[..header.len()].copy_from_slice(&header);

        let written = handle
            .write_interrupt(endpt_addr_out, &msg, consts::DEFAULT_TIMEOUT)
            .with_context(|| format!("write Layer 1 slot {} ({})", slot, description))?;

        ensure!(
            written == msg.len(),
            "short USB write for slot {}: wrote {} of {} bytes",
            slot,
            written,
            msg.len()
        );

        println!("  wrote slot {:2} (0x{:02x}): {}", slot, slot, description);
    }

    // Commit once after the six bindings.
    let mut commit = vec![0u8; consts::PACKET_SIZE];
    commit[..4].copy_from_slice(&[0x03, 0xfd, 0xfe, 0xff]);

    let written = handle
        .write_interrupt(endpt_addr_out, &commit, consts::DEFAULT_TIMEOUT)
        .context("commit Layer 1 recovery bindings")?;

    ensure!(
        written == commit.len(),
        "short USB write while committing: wrote {} of {} bytes",
        written,
        commit.len()
    );

    println!("Commit sent.");
    println!("Layer 1 factory recovery write completed.");
    Ok(())
}

fn open_keyboard(options: &Options) -> Result<Box<dyn Keyboard>> {
    // Find USB device based on the product id
    let (device, desc, id_product) = find_device(
        options.devel_options.vendor_id,
        options.devel_options.product_id,
    )
    .context("find USB device")?;

    ensure!(
        desc.num_configurations() == 1,
        "only one device configuration is expected"
    );

    // Find correct endpoint
    let (intf_num, endpt_addr_out, endpt_addr_in) = find_interface_and_endpoint(
        &device,
        options.devel_options.interface_number,
        options.devel_options.out_endpoint_address,
        options.devel_options.in_endpoint_address,
    )?;

    // Open device.
    let handle = device.open().context("open USB device")?;
    let _ = handle.set_auto_detach_kernel_driver(true);
    handle
        .claim_interface(intf_num)
        .context("claim interface")?;

    match id_product {
        0x8840 | 0x8842 => {
            k884x::Keyboard884x::new(Some(handle), endpt_addr_out, endpt_addr_in, id_product)
                .map(|v| Box::new(v) as Box<dyn Keyboard>)
        }
        0x8890 => k8890::Keyboard8890::new(Some(handle), endpt_addr_out)
            .map(|v| Box::new(v) as Box<dyn Keyboard>),
        0x8850 => k8850::Keyboard8850::new(Some(handle), endpt_addr_out)
            .map(|v| Box::new(v) as Box<dyn Keyboard>),
        _ => unreachable!("This shouldn't happen!"),
    }
}

pub fn find_device(vid: u16, pid: Option<u16>) -> Result<(Device<Context>, DeviceDescriptor, u16)> {
    debug!("vid: 0x{vid:02x}");
    if let Some(prod_id) = pid {
        debug!("pid: 0x{prod_id:02x}");
    } else {
        debug!("pid: None");
    }
    let options = vec![
        #[cfg(windows)]
        rusb::UsbOption::use_usbdk(),
    ];
    let usb_context = rusb::Context::with_options(&options)?;

    let mut found = vec![];
    for device in usb_context.devices().context("get USB device list")?.iter() {
        let desc = device.device_descriptor().context("get USB device info")?;
        debug!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            desc.vendor_id(),
            desc.product_id()
        );
        let product_id = desc.product_id();

        // Check BOTH Vendor IDs
        if desc.vendor_id() == vid || desc.vendor_id() == consts::VENDOR_ID_QH {
            if let Some(prod_id) = pid {
                if PRODUCT_IDS.contains(&prod_id) {
                    found.push((device, desc, product_id));
                }
            } else {
                if PRODUCT_IDS.contains(&product_id) {
                    found.push((device, desc, product_id));
                }
            }
        }
    }

    match found.len() {
        0 => Err(anyhow!(
            "macropad device not found. Use --vendor-id and --product-id to override defaults"
        )),
        1 => Ok(found.pop().unwrap()),
        _ => {
            let mut addresses = vec![];
            for (device, _desc, _product_id) in found {
                let address = (device.bus_number(), device.address());
                addresses.push(address);
            }

            Err(anyhow!(
                indoc! {"
                Several compatible devices are found.
                Unfortunately, this model of keyboard doesn't have serial number.
                So specify USB address using --address option.

                Addresses:
                {}
            "},
                addresses
                    .iter()
                    .map(|(bus, addr)| format!("{bus}:{addr}"))
                    .join("\n")
            ))
        }
    }
}
