use crate::{
    consts,
    keyboard::{
        Configuration, Keyboard, LedColor, MediaCode, Messages, Modifier,
        WellKnownCode,
    },
    mapping::Macropad,
};
use anyhow::Result;
use log::{debug, info};
use num::ToPrimitive;
use rusb::{Context, DeviceHandle};
use std::{str::FromStr, thread, time::Duration};
use strum::IntoEnumIterator;

pub struct Keyboard8850 {
    handle: Option<DeviceHandle<Context>>,
    out_endpoint: u8,
}

impl Configuration for Keyboard8850 {
    fn read_macropad_config(&mut self, layer: &u8) -> Result<Macropad> {
        let mut macropad = Macropad::new(4, 4, 3);
        // Track which layers we have successfully read data for
        let mut received_layers = [false; 4]; // Index 1-3 used for layers 1-3

        // --- STEP 1: Send Magic Init Packet (Frame 37871) ---
        // This puts the device into a state where it accepts read requests.
        // Extracted from Wireshark: 03 fa 19 00 01 06 30 cc ...
        let magic_packet: [u8; 65] = [
            0x03, 0xfa, 0x19, 0x00, 0x01, 0x06, 0x30, 0xcc,
            0x85, 0x00, 0xf0, 0xcc, 0x85, 0x00, 0x7c, 0xf2,
            0x02, 0x69, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x00,
            0x3c, 0x06, 0xf0, 0xcc, 0x85, 0x00, 0xbd, 0x00,
            0x00, 0x00, 0x97, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xe0, 0xcc, 0x85, 0x00, 0x70, 0xcd,
            0x85, 0x00, 0x30, 0xeb, 0x34, 0x06, 0x08, 0x23,
            0x3c, 0x06, 0x10, 0xcd, 0x85, 0x00, 0xc7, 0xb6,
            0x54
        ];

        // Determine which layers we want to read
        let layers_to_read: Vec<u8> = if *layer > 0 {
            vec![*layer]
        } else {
            info!("Reading all layers (1-3)...");
            vec![1, 2, 3]
        };

        for target_layer in layers_to_read {
            // Check if we already have this layer (e.g. from a previous multi-layer response)
            if received_layers[target_layer as usize] {
                debug!("Layer {} already received, skipping request.", target_layer);
                continue;
            }

            // Construct Magic Packet for this specific layer
            // We suspect byte 4 is the layer index based on K884x protocol and behavior
            let mut current_magic = magic_packet;
            current_magic[4] = target_layer;

            info!("Requesting Layer {}...", target_layer);
            self.send(&current_magic)?;

            // Wait for device to switch modes (increased to 200ms for stability)
            thread::sleep(Duration::from_millis(200));

            // --- STEP 2: Continuous Read Loop ---
            info!("Reading configuration stream...");

            let mut empty_reads = 0;
            // Increase timeout tolerance: 100ms timeout + 20ms sleep = 120ms per cycle.
            // 50 attempts * 120ms = 6 seconds.
            // This ensures we don't stop if there's a pause between layers.
            let max_empty_reads = 50;
            let mut total_packets = 0;

            loop {
                let mut buf = [0u8; consts::PACKET_SIZE];
                let len = self.recieve(&mut buf)?;

                if len == 0 {
                    empty_reads += 1;
                    if empty_reads >= max_empty_reads {
                        debug!("No data received for {} attempts, assuming stream end.", max_empty_reads);
                        break;
                    }
                    // Small delay between empty reads
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }

                // Reset empty reads counter on successful read
                empty_reads = 0;
                total_packets += 1;

            // Check if this looks like a valid response
            if buf[0] == 0x03 && buf[1] == 0xfa && buf[2] >= 1 && buf[2] <= 25 {
                let key_index = buf[2];
                let response_layer = buf[3];

                // The device streams layers 1, 2, 3 sequentially.
                if response_layer >= 1 && response_layer <= 3 {
                    // Mark this layer as received
                    received_layers[response_layer as usize] = true;

                    // Only update if we want all layers (0) or this specific layer
                    if *layer == 0 || *layer == response_layer {
                        let (delay, per_key_delays, mapping) = self.decode_packet(&buf);

                        if !mapping.is_empty() {
                            debug!("Layer {} Key {}: {} (Delay: {})", response_layer, key_index, mapping, delay);
                            self.update_macropad_struct(
                                &mut macropad,
                                response_layer,
                                key_index,
                                delay,
                                per_key_delays,
                                mapping,
                            );
                        }
                    }
                }
            }
            }

            info!("Read finished. Processed {} packets.", total_packets);

            // --- STEP 3: Send Termination Packet ---
            self.send(&self.end_program())?;

            // Small delay before next layer read
            thread::sleep(Duration::from_millis(200));
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

    fn program_led(&self, mode: u8, _layer: u8, color: LedColor) -> Vec<u8> {
        // K8850 uses a 3-packet sequence for LED control.
        // Structure: 03 fe b0 [seq] [mode] 00 [20 bytes data] ...
        // Total color data is 60 bytes (split 20/20/20 across 3 packets)
        // Sequence numbers are 00, 01, 02.

        let color_byte = <LedColor as ToPrimitive>::to_u8(&color).unwrap();
        // Fill 60 bytes with the color value.
        // Note: Real RGB control might require mapping LedColor to 3-byte RGB values,
        // but for now we repeat the color code which works for the device's internal palette.
        let color_data = vec![color_byte; 60];

        let mut all_packets = Vec::new();

        for seq in 0..3 {
            let mut packet = vec![0x03, 0xfe, 0xb0, seq, mode, 0x00];

            // Append 20 bytes of data for this sequence
            let start_idx = (seq as usize) * 20;
            if start_idx + 20 <= color_data.len() {
                packet.extend_from_slice(&color_data[start_idx..start_idx + 20]);
            }

            // Pad to 65 bytes
            while packet.len() < consts::PACKET_SIZE {
                packet.push(0);
            }

            all_packets.extend(packet);
        }

        all_packets
    }

    fn end_program(&self) -> Vec<u8> {
        // "03 fd fe ff" indicates end of session/programming block
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
                    let msg = self.build_key_msg(
                        &btn.mapping,
                        lyr,
                        key_num,
                        btn.delay,
                        &btn.per_key_delays,
                    )?;
                    self.send(&msg)?;
                    key_num += 1;
                }
            }

            // 2. Program Knobs
            for knob in &layer.knobs {
                // CCW
                self.send(&self.build_key_msg(
                    &knob.ccw.mapping,
                    lyr,
                    key_num,
                    knob.ccw.delay,
                    &knob.ccw.per_key_delays,
                )?)?;
                key_num += 1;

                // Press
                self.send(&self.build_key_msg(
                    &knob.press.mapping,
                    lyr,
                    key_num,
                    knob.press.delay,
                    &knob.press.per_key_delays,
                )?)?;
                key_num += 1;

                // CW
                self.send(&self.build_key_msg(
                    &knob.cw.mapping,
                    lyr,
                    key_num,
                    knob.cw.delay,
                    &knob.cw.per_key_delays,
                )?)?;
                key_num += 1;
            }

            // Reset key counter for next layer (Buttons start at 1)
            key_num = 1;
        }

        self.send(&self.end_program())?;
        Ok(())
    }

    fn set_led(&mut self, mode: u8, layer: u8, color: LedColor) -> Result<()> {
        let packets = self.program_led(mode, layer, color);

        // Send each 65-byte packet
        for chunk in packets.chunks(consts::PACKET_SIZE) {
            self.send(chunk)?;
            // Small delay to ensure device processes the sequence
            thread::sleep(Duration::from_millis(20));
        }

        // We might need to send end_program() after, similar to k884x,
        // but the protocol analysis didn't explicitly demand it for LED only.
        // However, it's safer to ensure state is saved/committed.
        // self.send(&self.end_program())?;
        // Based on K884x behavior, we'll omit end_program for now unless testing shows it's needed,
        // as the 3-packet sequence might be self-contained.

        Ok(())
    }

    fn get_handle(&self) -> &DeviceHandle<Context> {
        self.handle.as_ref().unwrap()
    }

    fn get_out_endpoint(&self) -> u8 {
        self.out_endpoint
    }

    fn get_in_endpoint(&self) -> u8 {
        // K8850 standard IN endpoint (auto-detected as 0x84)
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


    fn update_macropad_struct(
        &self,
        macropad: &mut Macropad,
        layer: u8,
        key_index: u8,
        delay: u16,
        per_key_delays: Vec<u16>,
        mapping: String,
    ) {
        let layer_idx = (layer - 1) as usize;

        if key_index <= 16 {
            // Button Mappings (1-16)
            let idx = (key_index - 1) as usize;
            let row = idx / 4;
            let col = idx % 4;
            if row < 4 && col < 4 {
                macropad.layers[layer_idx].buttons[row][col].delay = delay;
                macropad.layers[layer_idx].buttons[row][col].per_key_delays = per_key_delays;
                macropad.layers[layer_idx].buttons[row][col].mapping = mapping;
            }
        } else if key_index <= 25 {
            // Knob Mappings (17-25)
            // Grouped by 3: [CCW, Press, CW]
            let idx = (key_index - 17) as usize;
            let knob_idx = idx / 3;
            let action_idx = idx % 3;

            if knob_idx < 3 {
                let knob = &mut macropad.layers[layer_idx].knobs[knob_idx];
                match action_idx {
                    0 => {
                        knob.ccw.delay = delay;
                        knob.ccw.per_key_delays = per_key_delays;
                        knob.ccw.mapping = mapping;
                    }
                    1 => {
                        knob.press.delay = delay;
                        knob.press.per_key_delays = per_key_delays;
                        knob.press.mapping = mapping;
                    }
                    2 => {
                        knob.cw.delay = delay;
                        knob.cw.per_key_delays = per_key_delays;
                        knob.cw.mapping = mapping;
                    }
                    _ => {}
                }
            }
        }
    }

    fn decode_packet(&self, packet: &[u8]) -> (u16, Vec<u16>, String) {
        // Packet Header is bytes 0-6. Data starts at byte 7.
        // Byte 6 is the Count of sequences.
        if packet.len() < 8 {
            return (0, Vec::new(), "".to_string());
        }

        // Byte 6 is the Count of sequences.
        let count = packet[6];
        if count == 0 {
            return (0, Vec::new(), "".to_string());
        }

        let mut mappings = Vec::new();
        let mut per_key_delays = Vec::new();
        let mut global_delay = 0;

        let mut idx = 7;
        for _ in 0..count {
            if idx + 3 > packet.len() {
                break;
            }

            let delay_lo = packet[idx];
            let delay_hi = packet[idx + 1];
            let keycode = packet[idx + 2];

            let delay = ((delay_lo as u16) << 8) | (delay_hi as u16);
            per_key_delays.push(delay);

            // Store the delay (assuming uniform delay for chord)
            if delay > 0 {
                global_delay = delay;
            }

            let key_str = self.keycode_to_string(keycode);
            if !key_str.is_empty() {
                mappings.push(key_str);
            }

            idx += 3;
        }

        // Reconstruct modifiers (e.g., "ctrl", "c" -> "ctrl-c")
        let mut final_mapping = String::new();
        let mut i = 0;
        while i < mappings.len() {
            let current = &mappings[i];
            if self.is_modifier(current) && i + 1 < mappings.len() {
                let next = &mappings[i + 1];
                if !self.is_modifier(next) {
                    if !final_mapping.is_empty() {
                        final_mapping.push(',');
                    }
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

        (global_delay, per_key_delays, final_mapping)
    }

    fn is_modifier(&self, s: &str) -> bool {
        matches!(s, "ctrl" | "shift" | "alt" | "win" | "rctrl" | "rshift" | "ralt" | "rwin")
    }

    fn keycode_to_string(&self, code: u8) -> String {
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

        for key in WellKnownCode::iter() {
            if <WellKnownCode as ToPrimitive>::to_u8(&key).unwrap() == code {
                return key.to_string().to_lowercase();
            }
        }

        // Basic Media/Mouse check could go here if needed, or return hex
        // format!("0x{:02x}", code)
        // Empty string skips unknown codes
        "".to_string()
    }

    fn build_key_msg(
        &self,
        key_chord: &str,
        layer: u8,
        key_pos: u8,
        delay: u16,
        per_key_delays: &[u16],
    ) -> Result<Vec<u8>> {
        let mut msg = vec![0x03, 0xfd, key_pos, layer, 0x01, 0x00];

        let mut sequence: Vec<u8> = Vec::new();
        let keys_str: Vec<&str> = key_chord.split(',').collect();
        let mut key_seq_index = 0;

        for k in keys_str {
            let parts: Vec<&str> = k.split('-').collect();

            for part in parts {
                if part.trim().is_empty() {
                    continue;
                }

                let mut code_to_add = 0u8;

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
                } else if let Ok(w) = WellKnownCode::from_str(part) {
                    code_to_add = <WellKnownCode as ToPrimitive>::to_u8(&w).unwrap();
                } else if let Ok(_m) = MediaCode::from_str(part) {
                    debug!(
                        "Media key {} not fully supported in mixed sequence yet for 8850",
                        part
                    );
                    continue;
                }

                if code_to_add != 0 {
                    // Use per-key delay if available, otherwise default delay
                    let current_delay = if key_seq_index < per_key_delays.len() {
                        per_key_delays[key_seq_index]
                    } else {
                        delay
                    };
                    key_seq_index += 1;

                    // Delay is Big Endian based on user analysis (10ms -> 2560ms issue)
                    let d_bytes = current_delay.to_be_bytes();
                    sequence.push(d_bytes[0]);
                    sequence.push(d_bytes[1]);
                    sequence.push(code_to_add);
                }
            }
        }

        let num_keys = (sequence.len() / 3) as u8;
        msg.push(num_keys);
        msg.extend_from_slice(&sequence);

        while msg.len() < 65 {
            msg.push(0);
        }

        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k8850_keycode_to_string() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();

        // Test modifiers
        assert_eq!(keyboard.keycode_to_string(0xf1), "ctrl");
        assert_eq!(keyboard.keycode_to_string(0xf2), "shift");
        assert_eq!(keyboard.keycode_to_string(0xf3), "alt");
        assert_eq!(keyboard.keycode_to_string(0xf4), "win");

        // Test unknown codes
        assert_eq!(keyboard.keycode_to_string(0x99), "");
        assert_eq!(keyboard.keycode_to_string(0x00), "");
    }

    #[test]
    fn test_k8850_decode_empty_packet() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();

        // Test empty response (count = 0)
        let empty_packet = [0x03, 0xfa, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00];
        let (delay, per_key_delays, mapping) = keyboard.decode_packet(&empty_packet);
        assert_eq!(delay, 0);
        assert!(per_key_delays.is_empty());
        assert_eq!(mapping, "");
    }

    #[test]
    fn test_k8850_decode_packet_with_data() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();

        // Test packet with single key (count = 1)
        // [03, fa, key, layer, 01, 00, 01, delay_hi, delay_lo, keycode]
        let packet_with_key = [
            0x03, 0xfa, 0x01, 0x01, 0x01, 0x00, 0x01, 0x00, 0x00, 0x04,
        ]; // 0x04 = 'a'
        let (delay, per_key_delays, mapping) = keyboard.decode_packet(&packet_with_key);
        assert_eq!(delay, 0);
        assert_eq!(per_key_delays.len(), 1);
        assert_eq!(per_key_delays[0], 0);
        assert_eq!(mapping, "a");
    }

    #[test]
    fn test_k8850_build_key_msg() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();

        // Test simple key mapping
        let msg = keyboard.build_key_msg("a", 1, 1, 0, &[]).unwrap();
        assert_eq!(msg[0], 0x03);
        assert_eq!(msg[1], 0xfd); // Write command
        assert_eq!(msg[2], 0x01); // Key position
        assert_eq!(msg[3], 0x01); // Layer
        assert_eq!(msg[4], 0x01); // Type (keyboard)
        assert_eq!(msg[6], 0x01); // Count of sequences
        assert_eq!(msg.len(), 65); // Should be padded to 65 bytes
    }

    #[test]
    fn test_k8850_build_modifier_key_msg() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();

        // Test modifier + key mapping
        let msg = keyboard.build_key_msg("ctrl-a", 1, 1, 100, &[]).unwrap();
        assert_eq!(msg[0], 0x03);
        assert_eq!(msg[1], 0xfd);
        assert_eq!(msg[6], 0x02); // Count of sequences (ctrl + a)

        // Verify delay bytes are correct (100 = 0x0064). Little Endian: 0x64, 0x00 (wait, big endian now)
        // In prev turn I fixed endianness to Big Endian. So 100 (0x64) -> 0x00, 0x64
        // The previous test logic in `search` block:
        // assert_eq!(msg[7], 0x64); // Delay low byte
        // assert_eq!(msg[8], 0x00); // Delay high byte
        // That was checking for Little Endian. My fix in previous turn must have changed this.
        // Wait, did I fix the test in the previous turn? The condense summary said I fixed `build_key_msg` and `decode_packet`.
        // The test code shown in `read_file` results (above) shows:
        // // Verify delay bytes are correct (100 = 0x0064). Little Endian: 0x64, 0x00
        // assert_eq!(msg[7], 0x64); // Delay low byte
        // assert_eq!(msg[8], 0x00); // Delay high byte
        //
        // This suggests the test was NOT updated in the file I read.
        // BUT the `build_key_msg` in the file I read has:
        // let d_bytes = delay.to_be_bytes();
        // sequence.push(d_bytes[0]); // MSB
        // sequence.push(d_bytes[1]); // LSB
        //
        // `to_be_bytes` for 100 (0x0064) gives [0x00, 0x64].
        // So msg[7] should be 0x00, msg[8] should be 0x64.
        // The existing test code in the file expects Little Endian (0x64, 0x00).
        // This means the test is currently FAILING or I misread.
        // Ah, `to_be_bytes` returns [0x00, 0x64].
        // `sequence.push(d_bytes[0])` -> pushes 0x00.
        // `sequence.push(d_bytes[1])` -> pushes 0x64.
        // `sequence` is appended to `msg` after header (len 6 or 7).
        // Header: `vec![0x03, 0xfd, key_pos, layer, 0x01, 0x00]` -> 6 bytes.
        // `msg.push(num_keys)` -> 7th byte (index 6).
        // Data starts at index 7.
        // Index 7: 0x00
        // Index 8: 0x64
        // The test says `assert_eq!(msg[7], 0x64);`. This expects 0x64 at index 7.
        // So the test expects Little Endian.
        // But the code does Big Endian.
        // So the test IS failing if run.
        // I will fix the test expectations here to Big Endian [0x00, 0x64].

        assert_eq!(msg[7], 0x00); // Delay high byte
        assert_eq!(msg[8], 0x64); // Delay low byte
        assert_eq!(msg[9], 0xf1); // Ctrl modifier code
    }

    #[test]
    fn test_update_macropad_struct() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();
        let mut macropad = Macropad::new(4, 4, 3);

        // Test button mapping
        keyboard.update_macropad_struct(
            &mut macropad,
            1,
            1,
            50,
            vec![],
            "test_key".to_string(),
        );
        assert_eq!(macropad.layers[0].buttons[0][0].delay, 50);
        assert_eq!(macropad.layers[0].buttons[0][0].mapping, "test_key");

        // Test knob mapping
        keyboard.update_macropad_struct(
            &mut macropad,
            1,
            17,
            100,
            vec![],
            "knob_ccw".to_string(),
        );
        assert_eq!(macropad.layers[0].knobs[0].ccw.delay, 100);
        assert_eq!(macropad.layers[0].knobs[0].ccw.mapping, "knob_ccw");

        keyboard.update_macropad_struct(
            &mut macropad,
            1,
            18,
            150,
            vec![],
            "knob_press".to_string(),
        );
        assert_eq!(macropad.layers[0].knobs[0].press.delay, 150);
        assert_eq!(macropad.layers[0].knobs[0].press.mapping, "knob_press");

        keyboard.update_macropad_struct(
            &mut macropad,
            1,
            19,
            200,
            vec![],
            "knob_cw".to_string(),
        );
        assert_eq!(macropad.layers[0].knobs[0].cw.delay, 200);
        assert_eq!(macropad.layers[0].knobs[0].cw.mapping, "knob_cw");
    }

    #[test]
    fn test_end_program_message() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();
        let end_msg = keyboard.end_program();

        assert_eq!(end_msg[0], 0x03);
        assert_eq!(end_msg[1], 0xfd);
        assert_eq!(end_msg[2], 0xfe);
        assert_eq!(end_msg[3], 0xff);
        assert_eq!(end_msg.len(), 65); // Should be padded to 65 bytes
    }

    #[test]
    fn test_program_led_packets() {
        let keyboard = Keyboard8850::new(None, 0).unwrap();
        let mode = 0x03; // Middle Glow
        let color = LedColor::Red; // 0x10

        let packets = keyboard.program_led(mode, 0, color);

        // Should generate 3 packets of 65 bytes each
        assert_eq!(packets.len(), 65 * 3);

        // Verify Packet 0
        let p0 = &packets[0..65];
        assert_eq!(p0[0], 0x03);
        assert_eq!(p0[1], 0xfe);
        assert_eq!(p0[2], 0xb0);
        assert_eq!(p0[3], 0x00); // Seq 0
        assert_eq!(p0[4], mode);
        assert_eq!(p0[5], 0x00);
        // Data starts at index 6. Should be 20 bytes of color (0x10)
        for i in 0..20 {
            assert_eq!(p0[6 + i], 0x10);
        }

        // Verify Packet 1
        let p1 = &packets[65..130];
        assert_eq!(p1[0], 0x03);
        assert_eq!(p1[1], 0xfe);
        assert_eq!(p1[2], 0xb0);
        assert_eq!(p1[3], 0x01); // Seq 1
        assert_eq!(p1[4], mode);
        assert_eq!(p1[5], 0x00);
        // Data starts at index 6
        for i in 0..20 {
            assert_eq!(p1[6 + i], 0x10);
        }

        // Verify Packet 2
        let p2 = &packets[130..195];
        assert_eq!(p2[0], 0x03);
        assert_eq!(p2[1], 0xfe);
        assert_eq!(p2[2], 0xb0);
        assert_eq!(p2[3], 0x02); // Seq 2
        assert_eq!(p2[4], mode);
        assert_eq!(p2[5], 0x00);
        // Data starts at index 6
        for i in 0..20 {
            assert_eq!(p2[6 + i], 0x10);
        }
    }
}
