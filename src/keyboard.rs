use std::collections::HashMap;

use minifb::{Key, Window};

#[derive(Debug)]
pub struct KeyBoard {
    pub keys: [bool; 16],
    keys_mapping: HashMap<Key, usize>
}

impl KeyBoard {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        keys.insert(Key::A, 0x1);
        keys.insert(Key::Z, 0x2);
        keys.insert(Key::E, 0x3);
        keys.insert(Key::Q, 0x4);
        keys.insert(Key::S, 0x5);
        keys.insert(Key::D, 0x6);
        keys.insert(Key::W, 0x7);
        keys.insert(Key::X, 0x8);
        keys.insert(Key::C, 0x9);

        keys.insert(Key::R, 0xA);
        keys.insert(Key::F, 0xB);
        keys.insert(Key::T, 0xC);
        keys.insert(Key::G, 0xD);
        keys.insert(Key::B, 0xE);
        keys.insert(Key::Enter, 0xF);

        Self { keys: [false; 16], keys_mapping: keys }
    }

    pub fn load_rom_keymapping(&mut self, rom: String) {
        if rom == String::from("cave") {
            self.keys_mapping.remove(&Key::X);
            self.keys_mapping.insert(Key::S, 0x8);
        } else if rom == String::from("pong") {
            self.keys_mapping.remove(&Key::T);
            self.keys_mapping.remove(&Key::G);
            self.keys_mapping.insert(Key::P, 0xC);
            self.keys_mapping.insert(Key::M, 0xD);
        }
    }

    pub fn handle(&mut self, window: &Window) {
        
        for (k, h) in &self.keys_mapping {
            if window.is_key_down(*k) {
                self.keys[*h] = true;
            } else {
                self.keys[*h] = false;
            }
        }
    }
}
