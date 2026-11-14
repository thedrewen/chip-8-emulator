use std::collections::HashMap;

use minifb::{Key, Window};

#[derive(Debug)]
pub struct KeyBoard {
    pub keys: [bool; 16],
    keys_mapping: HashMap<Key, usize>,
    pub is_key_push: bool,
    pub last_key_down: Option<usize>
}

impl KeyBoard {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        keys.insert(Key::LeftCtrl, 0x0);
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

        Self { keys: [false; 16], keys_mapping: keys, is_key_push: false, last_key_down: None }
    }

    pub fn load_rom_keymapping(&mut self, rom: String) {
        if rom == String::from("cave") {
            self.keys_mapping.clear();
            
            self.keys_mapping.insert(Key::Z, 0x2); // Up
            self.keys_mapping.insert(Key::Q, 0x4); // Left
            self.keys_mapping.insert(Key::S, 0x8); // Down 
            self.keys_mapping.insert(Key::D, 0x6); // Right
            self.keys_mapping.insert(Key::Enter, 0xF); // Start
        } else if rom == String::from("pong") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::A, 0x1); // P1 Up
            self.keys_mapping.insert(Key::Q, 0x4); // P1 Down
            self.keys_mapping.insert(Key::P, 0xC); // P2 Up
            self.keys_mapping.insert(Key::M, 0xD); // P2 Down
        } else if rom == String::from("tetris") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::A, 0x4); // Rotation
            self.keys_mapping.insert(Key::S, 0x7); // Down
            self.keys_mapping.insert(Key::Q, 0x5); // Left
            self.keys_mapping.insert(Key::D, 0x6); // Right
        } else if rom == String::from("wall") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::Z, 0x1); // Up
            self.keys_mapping.insert(Key::S, 0x4); // Down
        } else if rom == String::from("missile") || rom == String::from("airplane") {
            self.keys_mapping.clear();
            
            self.keys_mapping.insert(Key::Space, 0x8); // Fire
        } else if rom == String::from("tank") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::Z, 0x8); // Up 
            self.keys_mapping.insert(Key::Q, 0x4); // Left
            self.keys_mapping.insert(Key::S, 0x2); // Down
            self.keys_mapping.insert(Key::D, 0x6); // Right
            self.keys_mapping.insert(Key::Space, 0x5); // Fire
        } else if rom == String::from("tron") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::W, 0xB); // With Wall
            self.keys_mapping.insert(Key::X, 0xF); // Without Wall

            self.keys_mapping.insert(Key::Enter, 0x0); // Start

            self.keys_mapping.insert(Key::Z, 0x1); // P1 Up 
            self.keys_mapping.insert(Key::S, 0x4); // P1 Down
            self.keys_mapping.insert(Key::Q, 0x3); // P1 Left
            self.keys_mapping.insert(Key::D, 0xC); // P1 Right

            self.keys_mapping.insert(Key::O, 0x7); // P2 Up 
            self.keys_mapping.insert(Key::L, 0xA); // P2 Down
            self.keys_mapping.insert(Key::K, 0x9); // P2 Left
            self.keys_mapping.insert(Key::M, 0xE); // P2 Right
        } else if rom == String::from("chipquarium") {
            self.keys_mapping.clear();

            self.keys_mapping.insert(Key::F, 0xF); // Feed
            self.keys_mapping.insert(Key::C, 0xC); // Clean Tank
            self.keys_mapping.insert(Key::G, 0xD); // Sleep
            self.keys_mapping.insert(Key::B, 0xB); // rock-paper-scissors
            self.keys_mapping.insert(Key::E, 0xE); // Stats

            // rock-paper-scissors
            self.keys_mapping.insert(Key::Up, 0x2); // Up
            self.keys_mapping.insert(Key::Down, 0x8); // Down
        }
    }

    pub fn handle(&mut self, window: &Window) {
        self.is_key_push = false;
        for (k, h) in &self.keys_mapping {
            if window.is_key_down(*k) {
                self.keys[*h] = true;
                self.is_key_push = true;
                self.last_key_down = Some(*h);
            } else {
                self.keys[*h] = false;
            }
        }
    }
}
