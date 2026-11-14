use minifb::{Key, Window, WindowOptions};

use crate::computer::Computer;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    window: Window,
    computer: Computer,
    screen_snapshot: [bool; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new(rom: String) -> Self {
        let mut window = Window::new(
            "TheDreWen CHIP-8 Emulator",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: minifb::Scale::X16,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(60);

        let mut computer = Computer::new();
        let mut rom_path = String::from("games/");
        rom_path.push_str(&rom);
        rom_path.push_str(".ch8");
        computer.insert_rom(rom_path, &rom);
        computer.keyboard.load_rom_keymapping(rom);

        Self {
            window,
            computer,
            screen_snapshot: [false; WIDTH * HEIGHT],
        }
    }

    fn convert_pixels_to_buffer(&mut self) -> [u32; WIDTH * HEIGHT] {
        let mut buffer: [u32; WIDTH * HEIGHT] = [0x0; WIDTH * HEIGHT];
        let mut real_screen: [bool; WIDTH * HEIGHT] = [false; WIDTH * HEIGHT];

        let mut i: usize = 0;
        for row in self.computer.screen {
            for col in row {
                if col || self.screen_snapshot[i] {
                    buffer[i] = 0xFFFFFF;
                } else {
                    buffer[i] = 0x0;
                }
                real_screen[i] = col;
                i += 1;
            }
        }
        self.screen_snapshot = real_screen.clone();
        buffer
    }

    pub fn play(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.computer.keyboard.handle(&self.window);
            self.computer.play();

            let buffer = self.convert_pixels_to_buffer();

            self.window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
