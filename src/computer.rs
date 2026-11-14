use std::{
    fs
};

use crate::{keyboard::KeyBoard, sound::Sound};

pub const FONTS_MIN_ADDR: usize = 80;
pub const FONTS_MAX_ADDR: usize = 160;
pub const INSTRUC_BY_IMAGE: usize = 11;

#[derive(Debug)]
pub struct Computer {
    pc: usize,
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: usize,
    call_stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    
    pub screen: [[bool; 64]; 32],
    pub keyboard: KeyBoard,
    pub sound: Sound
}

impl Computer {
    pub fn new() -> Self {
        let memory = Computer::initialize_memory();

        Self {
            pc: 0x200,
            memory,
            registers: [0; 16],
            index_register: 0,
            call_stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen: [[false; 64]; 32],
            keyboard: KeyBoard::new(),
            sound: Sound::new()
        }
    }

    fn initialize_memory() -> [u8; 4096] {
        let mut memory = [0; 4096];
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        let fonts_slice = &mut memory[FONTS_MIN_ADDR..FONTS_MAX_ADDR];
        fonts_slice.copy_from_slice(&fonts);

        memory
    }

    pub fn insert_rom(&mut self, file: String) {
        let r = fs::read(&file).expect("Unable to read the file.");

        if r.len() > 3584 {
            panic!("The file is too large.");
        }

        let memory_slice = &mut self.memory[0x200..(0x200 + r.len())];
        memory_slice.copy_from_slice(&r);
    }

    fn fetch(&mut self) -> u16 {
        let opcode: u16 = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        self.pc += 2;
        if self.pc == 4095 {
            panic!("Memory Error !");
        }

        opcode
    }

    pub fn play(&mut self) {
        for _ in 0..INSTRUC_BY_IMAGE {
            let opcode = self.fetch();
            let op_type = ((opcode & 0xF000) >> 12) as u8;
            let op_x = ((opcode & 0x0F00) >> 8) as usize;
            let op_y = ((opcode & 0x00F0) >> 4) as usize;
            let op_n = (opcode & 0x000F) as u8;

            let op_nn = (opcode & 0x00FF) as u8;
            let op_nnn = opcode & 0x0FFF;

            match op_type {
                0x1 => {
                    self.pc = op_nnn as usize;
                }
                0x6 => {
                    self.registers[op_x] = op_nn;
                }
                0xA => {
                    self.index_register = op_nnn as usize;
                }
                0xC => {
                    self.registers[op_x] = rand::random_range(0..=255) & op_nn;
                }
                0xD => {
                    self.registers[0xF] = 0;

                    let x = self.registers[op_x] % 64;
                    let y = self.registers[op_y] % 32;
                    
                    let frame =
                        &self.memory[self.index_register..(self.index_register + op_n as usize)];

                    for (row, octet) in frame.iter().enumerate() {
                        let mut cols: [usize; 8] = [0; 8];
                        for i in 0..8 {
                            cols[i] = ((octet >> (7 - i)) & 0x1) as usize;
                        }

                        for (col, bit) in cols.iter().enumerate() {
                            if row + y as usize <= 31 && col + x as usize <= 63 {
                                let default_v = self.screen[row + y as usize][col + x as usize];
                                self.screen[row + y as usize][col + x as usize] =
                                    ((self.screen[row + y as usize][col + x as usize] as usize)
                                        ^ bit)
                                        == 1;

                                if default_v != self.screen[row + y as usize][col + x as usize] && default_v {
                                    self.registers[0xF] = 1;
                                }
                            }
                        }
                    }

                    // self.display_screen();
                }
                0x3 => {
                    if self.registers[op_x] == op_nn {
                        self.pc += 2;
                    }
                }
                0x4 => {
                    if self.registers[op_x] != op_nn {
                        self.pc += 2;
                    }
                }
                0x5 => {
                    if self.registers[op_x] == self.registers[op_y] {
                        self.pc += 2;
                    }
                }
                0x7 => {
                    self.registers[op_x] = self.registers[op_x].wrapping_add(op_nn);
                }
                0x9 => {
                    if self.registers[op_x] != self.registers[op_y] {
                        self.pc += 2;
                    }
                }
                0x2 => {
                    self.call_stack[self.stack_pointer as usize] = self.pc as u16;
                    self.stack_pointer += 1;

                    self.pc = op_nnn as usize;
                }
                0x8 => match op_n {
                    0 => {
                        self.registers[op_x] = self.registers[op_y];
                    }
                    1 => {
                        self.registers[op_x] |= self.registers[op_y];
                    }
                    2 => {
                        self.registers[op_x] &= self.registers[op_y];
                    }
                    3 => {
                        self.registers[op_x] ^= self.registers[op_y];
                    }
                    4 => {
                        let default_value = self.registers[op_x];
                        self.registers[op_x] =
                            self.registers[op_x].wrapping_add(self.registers[op_y]);
                        if default_value > self.registers[op_x] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                    }
                    5 => {
                        let default_value = self.registers[op_x];
                        self.registers[op_x] =
                            self.registers[op_x].wrapping_sub(self.registers[op_y]);
                        if default_value >= self.registers[op_y] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                    }
                    6 => {
                        let default_bit = self.registers[op_y] & 0x1;
                        self.registers[op_x] = self.registers[op_y] >> 0x1;
                        self.registers[0xF] = default_bit;
                    }
                    7 => {
                        let default_value = self.registers[op_x];
                        self.registers[op_x] =
                            self.registers[op_y].wrapping_sub(self.registers[op_x]);
                        if default_value <= self.registers[op_y] {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                    }
                    0xE => {
                        let default_bit = self.registers[op_y] & 0x1;
                        self.registers[op_x] = self.registers[op_y] << 0x1;
                        self.registers[0xF] = default_bit;
                    }
                    _ => {
                        println!("OpCode : {opcode:04X}");
                        println!("OpType : {op_type:04X}");
                        println!("OpX : {op_x:04X}");
                        println!("OpY : {op_y:04X}");
                        println!("OpN : {op_n:04X}");
                        println!("OpNN : {op_nn:04X}");
                        println!("OpNNN : {op_nnn:04X}");
                        panic!("Instruction not found.");
                    }
                },
                0x0 => match op_nn {
                    0xEE => {
                        self.stack_pointer -= 1;
                        self.pc = self.call_stack[self.stack_pointer as usize] as usize;
                    }
                    0xE0 => {
                        self.screen = [[false; 64]; 32];
                    }
                    _ => {
                        println!("OpCode : {opcode:04X}");
                        println!("OpType : {op_type:04X}");
                        println!("OpX : {op_x:04X}");
                        println!("OpY : {op_y:04X}");
                        println!("OpN : {op_n:04X}");
                        println!("OpNN : {op_nn:04X}");
                        println!("OpNNN : {op_nnn:04X}");
                        panic!("Instruction not found.");
                    }
                },
                0xE => match op_nn {
                    0x9E => {
                        //println!("Ps : {:04X}", self.registers[op_x]);
                        if self.keyboard.keys[self.registers[op_x] as usize] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        //println!("Rl : {:04X}", self.registers[op_x]);
                        if !self.keyboard.keys[self.registers[op_x] as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        println!("OpCode : {opcode:04X}");
                        println!("OpType : {op_type:04X}");
                        println!("OpX : {op_x:04X}");
                        println!("OpY : {op_y:04X}");
                        println!("OpN : {op_n:04X}");
                        println!("OpNN : {op_nn:04X}");
                        println!("OpNNN : {op_nnn:04X}");
                        panic!("Instruction not found.");
                    }
                },
                0xF => match op_nn {
                    0x7 => {
                        self.registers[op_x] = self.delay_timer;
                    }
                    0x15 => {
                        self.delay_timer = self.registers[op_x];
                    }
                    0x18 => {
                        self.sound_timer = self.registers[op_x];
                    }
                    0x29 => {
                        self.index_register = FONTS_MIN_ADDR + (self.registers[op_x] as usize & 0xF) * 5;
                    }
                    0x55 => {
                        let v = &self.registers[0x0..=op_x];
                        let m =
                            &mut self.memory[self.index_register..(self.index_register + v.len())];
                        m.copy_from_slice(v);
                    }
                    0x65 => {
                        let v = &mut self.registers[0x0..=op_x];
                        let m = &self.memory[self.index_register..(self.index_register + v.len())];
                        v.copy_from_slice(m);
                    }
                    0x33 => {
                        let v = self.registers[op_x];
                        let c = (v / 100) % 100;
                        let d = (v / 10) % 10;
                        let u = v % 10;

                        self.memory[self.index_register] = c;
                        self.memory[self.index_register + 1] = d;
                        self.memory[self.index_register + 2] = u;
                    }
                    _ => {
                        println!("OpCode : {opcode:04X}");
                        println!("OpType : {op_type:04X}");
                        println!("OpX : {op_x:04X}");
                        println!("OpY : {op_y:04X}");
                        println!("OpN : {op_n:04X}");
                        println!("OpNN : {op_nn:04X}");
                        println!("OpNNN : {op_nnn:04X}");
                        panic!("Instruction not found.");
                    }
                },
                _ => {
                    println!("OpCode : {opcode:04X}");
                    println!("OpType : {op_type:04X}");
                    println!("OpX : {op_x:04X}");
                    println!("OpY : {op_y:04X}");
                    println!("OpN : {op_n:04X}");
                    println!("OpNN : {op_nn:04X}");
                    println!("OpNNN : {op_nnn:04X}");
                    panic!("Instruction not found.");
                }
            }
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            println!("SOUND");
            self.sound.play_beep();
            self.sound_timer -= 1;
        }
    }
}
