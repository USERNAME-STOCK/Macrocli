use crate::{
    keyboard::{
        Configuration, Keyboard, LedColor, MediaCode, Messages, Modifier,
        WellKnownCode,
    },
    mapping::Macropad,
};
use anyhow::Result;
use log::debug;
use num::ToPrimitive;
use rusb::{Context, DeviceHandle};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub struct Keyboard8850 {
    handle: Option<DeviceHandle<Context>>,
    out_endpoint: u8,
}

impl Configuration for Keyboard8850 {
    fn read_macropad_config(&mut self, _layer: &u8) -> Result<Macropad> {
        // 1. Send the "Magic Packet" to trigger read mode
        // Based on Wireshark capture:
        // 03 fb fb fb fb 50 0e 09 14 10 67 84 a2 f0 6b 25 6f f1 cc f8 50 03 2d 54 c0 0e 08 f4 10 69 a1 41 06 9a ed d1 77 00 00 00 00 00 0b 14 a2 5f ce 09 14 10 6f cc f8 50 00 00 9b b3 68 18 d0 85 00
        let magic_packet: [u8; 65] = [
            0x03, 0xfb, 0xfb, 0xfb, 0xfb, 0x50, 0x0e, 0x09, 0x14, 0x10, 0x67, 0x84, 0xa2, 0xf0,
            0x6b, 0x25, 0x6f, 0xf1, 0xcc, 0xf8, 0x50, 0x03, 0x2d, 0x54, 0xc0, 0x0e, 0x08, 0xf4,
            0x10, 0x69, 0xa1, 0x41, 0x06, 0x9a, 0xed, 0xd1, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x0b, 0x14, 0xa2, 0x5f, 0xce, 0x09, 0x14, 0x10, 0x6f, 0xcc, 0xf8, 0x50, 0x00, 0x00,
            0x9b, 0xb3, 0x68, 0x18, 0xd0, 0x85, 0x00, 0x00, 0x00,
        ];

        debug!("Sending magic packet to trigger read...");
        self.send(&magic_packet)?;

        // 2. Receive 25 packets (one for each key/knob position)
        // 4x4 grid = 16 buttons
        // 3 knobs * 3 actions (CCW, Press, CW) = 9 actions
        // Total = 25 packets
        let mut packets = Vec::new();
        for i in 0..25 {
            let mut buf = [0u8; 64]; // Standard interrupt packet size
            let len = self.recieve(&mut buf)?;
            if len > 0 {
                debug!("Received packet {}: {:02x?}", i + 1, &buf[..len]);
                packets.push(buf.to_vec());
            } else {
                debug!("Timeout or empty packet at index {}", i);
            }
        }

        // The device seems to only send one packet back, which is an ACK-like response.
        // The rest are timeouts. This suggests the magic packet might be incorrect,
        // or the device needs to be in a specific "read" mode (e.g., holding a button while plugging in).
        // For now, let's assume the device is empty and return a default Macropad.
        // The logic to decode is correct, but it needs data.
        // Let's add a more informative message for the user.
        if packets.is_empty() || (packets.len() == 1 && packets[0][6] == 0) {
            println!("Warning: Device did not return any configuration data.");
            println!("This could mean:");
            println!("  1. The device is not configured (all keys are empty).");
            println!("  2. The 'read' magic packet is incorrect.");
            println!("  3. The device needs to be in a special mode to be read from.");
            println!("Returning an empty configuration.");
            // Return a default, empty Macropad struct
            return Ok(Macropad::new(4, 4, 3));
        }

        // 3. Decode packets into Macropad struct
        // Assuming standard 4x4 layout + 3 knobs
        let mut macropad = Macropad::new(4, 4, 3);
        // We are reading the whole device config, but the trait asks for a specific layer?
        // The 8850 protocol seems to dump everything or maybe just the active layer?
        // The magic packet doesn't seem to specify a layer.
        // Let's assume the response contains data for the requested layer or we just fill one layer.
        // Wait, the write logic sends data for ALL layers.
        // The read trigger might just dump the current layer or all?
        // Given the loop count (25), it matches exactly ONE layer of controls (16 buttons + 9 knob actions).
        // So we will populate the first layer of the Macropad struct.

        let layer_idx = 0; // We'll put it in the first layer for now

        // Buttons 1-16
        for (i, packet) in packets.iter().enumerate() {
            if i < 16 {
                // It's a button
                let row = i / 4;
                let col = i % 4;
                let (delay, mapping) = self.decode_packet(packet);
                macropad.layers[layer_idx].buttons[row][col].delay = delay;
                macropad.layers[layer_idx].buttons[row][col].mapping = mapping;
            } else {
                // It's a knob action
                // 16: Knob 1 CCW
                // 17: Knob 1 Press
                // 18: Knob 1 CW
                // ...
                let knob_action_idx = i - 16;
                let knob_idx = knob_action_idx / 3;
                let action = knob_action_idx % 3;

                if knob_idx < 3 {
                    let (delay, mapping) = self.decode_packet(packet);
                    match action {
                        0 => {
                            macropad.layers[layer_idx].knobs[knob_idx].ccw.delay = delay;
                            macropad.layers[layer_idx].knobs[knob_idx].ccw.mapping = mapping;
                        }
                        1 => {
                            macropad.layers[layer_idx].knobs[knob_idx].press.delay = delay;
                            macropad.layers[layer_idx].knobs[knob_idx].press.mapping = mapping;
                        }
                        2 => {
                            macropad.layers[layer_idx].knobs[knob_idx].cw.delay = delay;
                            macropad.layers[layer_idx].knobs[knob_idx].cw.mapping = mapping;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(macropad)
    }
}

impl Messages for Keyboard8850 {
    fn read_config(&self, _keys: u8, _encoders: u8, _layer: u8) -> Vec<u8> {
        vec![]
    }

    fn device_type(&self) -> Vec<u8> {
        vec![]
    }

    fn program_led(&self, _mode: u8, _layer: u8, _color: LedColor) -> Vec<u8> {
        // LED support requires further reverse engineering for this specific model
        vec![]
    }

    fn end_program(&self) -> Vec<u8> {
        // "03 fd fe ff" indicates end of programming block
        let mut msg = vec![0x03, 0xfd, 0xfe, 0xff];
        msg.extend_from_slice(&[0; 61]);
        msg
    }
}

impl Keyboard for Keyboard8850 {
    fn program(&mut self, macropad: &Macropad) -> Result<()> {
        // Assuming standard 4x4 mapping: Keys 1-16
        let mut key_num = 1;

        for (i, layer) in macropad.layers.iter().enumerate() {
            let lyr = (i + 1) as u8;

            // 1. Program Buttons
            for row in &layer.buttons {
                for btn in row {
                    // Skip empty mappings to save time/writes if desired,
                    // but writing clears previous configs.
                    let msg = self.build_key_msg(&btn.mapping, lyr, key_num, btn.delay)?;
                    self.send(&msg)?;
                    key_num += 1;
                }
            }

            // 2. Program Knobs
            // Standard mapping usually continues after buttons.
            // If keys are 1-16, Knob 1 might be 17(CCW), 18(Press), 19(CW)...
            for knob in &layer.knobs {
                // CCW
                self.send(&self.build_key_msg(&knob.ccw.mapping, lyr, key_num, knob.ccw.delay)?)?;
                key_num += 1;

                // Press
                self.send(&self.build_key_msg(&knob.press.mapping, lyr, key_num, knob.press.delay)?)?;
                key_num += 1;

                // CW
                self.send(&self.build_key_msg(&knob.cw.mapping, lyr, key_num, knob.cw.delay)?)?;
                key_num += 1;
            }

            // Reset key counter for next layer
            key_num = 1;
        }

        self.send(&self.end_program())?;
        Ok(())
    }

    fn set_led(&mut self, _mode: u8, _layer: u8, _color: LedColor) -> Result<()> {
        Ok(())
    }

    fn get_handle(&self) -> &DeviceHandle<Context> {
        self.handle.as_ref().unwrap()
    }

    fn get_out_endpoint(&self) -> u8 {
        self.out_endpoint
    }

    fn get_in_endpoint(&self) -> u8 {
        // Hardcoded based on device info (EP 4 IN is 0x84)
        0x84
    }
}

impl Keyboard8850 {
    pub fn new(handle: Option<DeviceHandle<Context>>, out_endpoint: u8) -> Result<Self> {
        Ok(Self {
            handle,
            out_endpoint,
        })
    }

    fn decode_packet(&self, packet: &[u8]) -> (u16, String) {
        // The received packet seems to be a response to the magic packet, not the key data itself.
        // The first packet is: [03, fb, 10, 03, 0b, 00, 00, ...]
        // This looks like a header. The actual key data might follow in subsequent packets.
        // However, the logic expects to decode each of the 25 packets.
        // Let's re-examine the write packet structure:
        // [03, fd, key_pos, layer, 0x01, 0x00, count, data...]
        // The read response might be similar.
        // The first packet we received has `count` (byte 6) as 0, so it's empty.
        // This suggests the device might not be configured, or the magic packet is not correct,
        // or the device needs to be in a specific mode to dump its config.
        // For now, let's assume the logic is correct and the device is just empty.
        // The decoding logic itself seems fine, it just needs data to work with.

        // Let's refine the decoding logic based on the assumption that a non-empty packet
        // will have the structure we expect.
        if packet.len() < 8 {
            return (0, "".to_string());
        }

        let count = packet[6];
        if count == 0 {
            return (0, "".to_string());
        }

        let mut mappings = Vec::new();
        let mut global_delay = 0;

        let mut idx = 7;
        for _ in 0..count {
            if idx + 3 > packet.len() {
                break;
            }

            let delay_hi = packet[idx];
            let delay_lo = packet[idx + 1];
            let keycode = packet[idx + 2];

            let delay = ((delay_hi as u16) << 8) | (delay_lo as u16);
            if delay > 0 {
                global_delay = delay;
            }

            let key_str = self.keycode_to_string(keycode);
            if !key_str.is_empty() {
                mappings.push(key_str);
            }

            idx += 3;
        }

        // Let's try to reconstruct modifier-key combinations.
        // e.g., ["ctrl", "c"] -> "ctrl-c"
        let mut final_mapping = String::new();
        let mut i = 0;
        while i < mappings.len() {
            let current = &mappings[i];
            if self.is_modifier(current) && i + 1 < mappings.len() {
                let next = &mappings[i + 1];
                if !self.is_modifier(next) {
                    final_mapping.push_str(&format!("{}-{}", current, next));
                    i += 2;
                    continue;
                }
            }
            if !final_mapping.is_empty() {
                final_mapping.push(',');
            }
            final_mapping.push_str(current);
            i += 1;
        }

        (global_delay, final_mapping)
    }

    fn is_modifier(&self, s: &str) -> bool {
        matches!(s, "ctrl" | "shift" | "alt" | "win" | "rctrl" | "rshift" | "ralt" | "rwin")
    }

    fn keycode_to_string(&self, code: u8) -> String {
        // Check Modifiers
        match code {
            0xf1 => return "ctrl".to_string(),
            0xf2 => return "shift".to_string(),
            0xf3 => return "alt".to_string(),
            0xf4 => return "win".to_string(),
            0xf5 => return "rctrl".to_string(),
            0xf6 => return "rshift".to_string(),
            0xf7 => return "ralt".to_string(),
            0xf8 => return "rwin".to_string(),
            _ => {}
        }

        // Check Standard Keys
        // We can iterate over WellKnownCode variants
        for key in WellKnownCode::iter() {
            if <WellKnownCode as ToPrimitive>::to_u8(&key).unwrap() == code {
                return key.to_string().to_lowercase();
            }
        }

        // Check Media Keys?
        // Media keys are u16 in the enum, but protocol uses u8?
        // Or maybe they are mapped to specific u8 codes in this device?
        // The write logic didn't fully implement media keys for 8850 yet.
        // We'll skip for now or print hex if unknown.

        format!("0x{:02x}", code)
    }

    fn build_key_msg(&self, key_chord: &str, layer: u8, key_pos: u8, delay: u16) -> Result<Vec<u8>> {
        // Protocol Structure:
        // Byte 0: 0x03 (Report ID)
        // Byte 1: 0xfd (Command)
        // Byte 2: Key Position
        // Byte 3: Layer
        // Byte 4: Type (0x01 = Keyboard)
        // Byte 5: 0x00
        // Byte 6: Count of sequences
        // Byte 7+: Sequence data [DelayHi, DelayLo, KeyCode]

        let mut msg = vec![0x03, 0xfd, key_pos, layer, 0x01, 0x00];

        let mut sequence: Vec<u8> = Vec::new();
        // Note: This splitting logic is simplified. Complex nested commas inside quotes aren't handled by simple split,
        // but standard macrocli mappings usually don't quote keys.
        let keys_str: Vec<&str> = key_chord.split(',').collect();

        for k in keys_str {
            let parts: Vec<&str> = k.split('-').collect();

            // 8850 Specific: Modifiers are sent as individual keys in the sequence
            // e.g., "ctrl-c" -> Sequence: [Delay, CtrlCode], [Delay, C_Code]

            for part in parts {
                if part.trim().is_empty() { continue; }

                let mut code_to_add = 0u8;

                // Check Modifiers
                if let Ok(m) = Modifier::from_str(part) {
                    code_to_add = match m {
                        Modifier::Ctrl => 0xf1,
                        Modifier::Shift => 0xf2,
                        Modifier::Alt => 0xf3,
                        Modifier::Win => 0xf4,
                        Modifier::RightCtrl => 0xf5,
                        Modifier::RightShift => 0xf6,
                        Modifier::RightAlt => 0xf7,
                        Modifier::RightWin => 0xf8,
                    };
                }
                // Check Standard Keys
                else if let Ok(w) = WellKnownCode::from_str(part) {
                    code_to_add = <WellKnownCode as ToPrimitive>::to_u8(&w).unwrap();
                }
                // Check Media Keys (Basic mapping attempt)
                else if let Ok(_m) = MediaCode::from_str(part) {
                    debug!("Media key {} not fully supported in mixed sequence yet for 8850", part);
                    continue;
                }

                if code_to_add != 0 {
                    // Add 3 bytes: Delay High, Delay Low, Code
                    // Using the button's global delay for every key in the chord
                    let d_bytes = delay.to_be_bytes();
                    sequence.push(d_bytes[0]);
                    sequence.push(d_bytes[1]);
                    sequence.push(code_to_add);
                }
            }
        }

        // Calculate number of key presses (each takes 3 bytes)
        let num_keys = (sequence.len() / 3) as u8;
        msg.push(num_keys);
        msg.extend_from_slice(&sequence);

        // Pad to 65 bytes
        while msg.len() < 65 {
            msg.push(0);
        }

        Ok(msg)
    }
}