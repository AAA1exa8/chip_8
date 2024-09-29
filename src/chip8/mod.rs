use std::{ops::Shr, panic, time::Duration};

use sdl2::{event::Event, keyboard::Keycode};

mod cpu_const;
pub mod disasm;
pub mod options;
pub mod screen;

pub struct Chip8 {
    pub running: bool,
    wainting: bool,
    screen: screen::Screen,
    memory: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    pc: usize,
    sp: usize,
}

impl Chip8 {
    pub fn new_with_rom(options: options::Chip8Options) -> Self {
        let mut chip = Chip8 {
            running: true,
            wainting: false,
            screen: screen::Screen::new(options.scale_factor),
            memory: [0; 4096],
            registers: [0; 16],
            i: 0,
            pc: cpu_const::PC_START,
            sp: cpu_const::STACK_POINT_START,
        };
        let font = include_bytes!("../../FONTS.chip8");
        chip.load_rom(font, 0);
        chip.load_rom(&options.rom, 0x200);
        chip.screen.start_screen();
        chip
    }

    pub fn cycle(&mut self) {
        let first_part = self.memory[self.pc];
        let second_part = self.memory[self.pc + 1];
        let disc_1: u8 = first_part.shr(4);
        let reg_1: u8 = first_part & 0x0F;
        let reg_2: u8 = second_part.shr(4);
        let disc_2: u8 = second_part & 0x0F;
        let address = ((reg_1 as u16) << 8) | (second_part as u16);
        let number = reg_2 << 4 | disc_2;
        for event in self.screen.get_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                    break;
                }
                _ => {}
            }
        }
        match disc_1 {
            0x00 => self.clear_return(address),
            0x01 => self.jump_to_address(address),
            0x02 => self.call_subroutine(address),
            0x03 => self.skip_if_reg_equal_val(number, reg_1),
            0x04 => self.skip_if_reg_not_equal_val(number, reg_1),
            0x05 => self.skip_if_reg_equal_reg(reg_1, reg_2),
            0x06 => self.move_value_to_reg(reg_1, number),
            0x07 => self.add_value_to_reg(reg_1, number),
            0x08 => self.execute_logical_instruction(reg_1, reg_2, disc_2),
            0x09 => self.skip_if_reg_not_equal_reg(reg_1, reg_2),
            0x0A => self.load_index_reg_with_value(address),
            0x0B => self.jump_to_register_plus_value(address),
            0x0C => self.generate_random_number(reg_1, number),
            0x0D => self.draw_sprite(reg_1, reg_2, disc_2),
            0x0E => self.keyboard_routines(reg_1, number),
            0x0F => self.misc_routines(reg_1, disc_2),
            _ => panic!("Unsupported instruction"),
        }
        if self.wainting {
            self.wainting = false;
        }
        self.pc += 2;
    }

    fn load_rom(&mut self, rom: &[u8], index: usize) {
        self.memory[index..index + rom.len()].copy_from_slice(rom)
    }
}

impl Chip8 {
    fn clear_return(&mut self, address: u16) {
        match address {
            0x00E0 => self.screen.clear(),
            0x00EE => {
                self.sp -= 1;
                self.pc = (self.memory[self.sp] as usize) << 8 | self.memory[self.sp - 1] as usize;
                self.sp -= 1;
            }
            _ => self.pc = address as usize,
        }
    }
    fn jump_to_address(&mut self, address: u16) {
        self.pc = address as usize;
    }
    fn call_subroutine(&mut self, address: u16) {
        self.memory[self.sp] = (self.pc.shr(8)) as u8;
        self.sp += 1;
        self.memory[self.sp] = self.pc as u8;
        self.sp += 1;
        self.pc = address as usize;
    }
    fn skip_if_reg_equal_val(&mut self, number: u8, reg: u8) {
        if self.registers[reg as usize] == number {
            self.pc += 2;
        }
    }
    fn skip_if_reg_not_equal_val(&mut self, number: u8, reg: u8) {
        if self.registers[reg as usize] != number {
            self.pc += 2;
        }
    }
    fn skip_if_reg_equal_reg(&mut self, reg_1: u8, reg_2: u8) {
        if self.registers[reg_1 as usize] == self.registers[reg_2 as usize] {
            self.pc += 2;
        }
    }
    fn move_value_to_reg(&mut self, reg: u8, number: u8) {
        self.registers[reg as usize] = number;
    }
    fn add_value_to_reg(&mut self, reg: u8, number: u8) {
        self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(number);
    }
    fn execute_logical_instruction(&mut self, reg_1: u8, reg_2: u8, disc: u8) {
        match disc {
            0x01 => self.registers[reg_1 as usize] |= self.registers[reg_2 as usize],
            0x02 => self.registers[reg_1 as usize] &= self.registers[reg_2 as usize],
            0x03 => self.registers[reg_1 as usize] ^= self.registers[reg_2 as usize],
            0x04 => {
                let result =
                    self.registers[reg_1 as usize] as u16 + self.registers[reg_2 as usize] as u16;
                self.registers[0xF] = if result > 0xFF { 1 } else { 0 };
                self.registers[reg_1 as usize] = result as u8;
            }
            0x05 => {
                self.registers[0xF] =
                    if self.registers[reg_1 as usize] > self.registers[reg_2 as usize] {
                        1
                    } else {
                        0
                    };
                self.registers[reg_1 as usize] -= self.registers[reg_2 as usize];
            }
            0x06 => {
                self.registers[0xF] = self.registers[reg_1 as usize] & 0x01;
                self.registers[reg_1 as usize] >>= 1;
            }
            0x07 => {
                self.registers[0xF] =
                    if self.registers[reg_2 as usize] > self.registers[reg_1 as usize] {
                        1
                    } else {
                        0
                    };
                self.registers[reg_1 as usize] =
                    self.registers[reg_2 as usize] - self.registers[reg_1 as usize];
            }
            0x0E => {
                self.registers[0xF] = self.registers[reg_1 as usize] & 0x80;
                self.registers[reg_1 as usize] <<= 1;
            }
            _ => panic!("Unsupported instruction"),
        }
    }
    fn skip_if_reg_not_equal_reg(&mut self, reg_1: u8, reg_2: u8) {
        if self.registers[reg_1 as usize] != self.registers[reg_2 as usize] {
            self.pc += 2;
        }
    }
    fn load_index_reg_with_value(&mut self, address: u16) {
        self.i = address;
    }
    fn jump_to_register_plus_value(&mut self, adress: u16) {
        self.pc = (self.registers[0] as u16 + adress) as usize;
    }
    fn generate_random_number(&mut self, reg: u8, mask: u8) {
        self.registers[reg as usize] = rand::random::<u8>() & mask;
    }
    fn draw_sprite(&mut self, x: u8, y: u8, len: u8) {
        let sprite = &self.memory[self.i as usize..(self.i + len as u16) as usize];
        let collision = self.screen.draw(x, y, sprite);
        self.registers[0xF] = if collision { 1 } else { 0 };
    }
    fn keyboard_routines(&mut self, reg: u8, disc: u8) {
        match disc {
            0x9E => {
                if self.screen.is_key_pressed(self.registers[reg as usize]) {
                    self.pc += 2;
                }
            }
            0xA1 => {
                if !self.screen.is_key_pressed(self.registers[reg as usize]) {
                    self.pc += 2;
                }
            }
            _ => panic!("Unsupported instruction"),
        }
    }
    fn misc_routines(&mut self, reg: u8, disc: u8) {}
}

impl Chip8 {}
