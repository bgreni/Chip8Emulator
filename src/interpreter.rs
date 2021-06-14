/// Logical construct of the interperter itself.
/// Defines the interface of the interperter, and implements 
/// all top level functionality
use crate::hardware::{Registers, Memory};
use crate::program_handler::ProgramHandler;
use crate::traits::instructions::{Helpers, Instructions};
use std::slice::Chunks;

// use crate::drivers::screen::Screen;
// use crate::drivers::input::Inputs;
// use piston::input::Button;

const CLOCK_HZ: f32 = 600.0;

const STACK_SIZE_LIMIT: i8 = 31;

pub const FONT_ADDR: usize = 0;
/// Number of rows in one font sprite
pub const FONT_HEIGHT: usize = 5;
/// Size of one font sprite
const FONT_BYTES: usize = FONT_HEIGHT * 16;
/// Data of the built-in font
const FONT: [u8; FONT_BYTES] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Width of the screen in pixels
pub const SCREEN_WIDTH: usize = 64;
/// Height of the screen in pixels
pub const SCREEN_HEIGHT: usize = 32;
/// Total number of pixels of the screen
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/// Number of keys on the keypad
const NUM_KEYS: usize = 16;

#[allow(dead_code)]
pub struct Interpreter {
    pub registers: Registers,
    pub memory: Memory,
    pub key_reg: usize,
    pub program_handler: ProgramHandler,
    pub next_op: u16,
    pub timer: u8,
    pub t_tick: f32,
    pub sound_timer: u8,
    pub st_tick: f32,

    pub screen: [u8; SCREEN_PIXELS],
    pub keys: [u8; NUM_KEYS],
    pub waiting_on_key: Option<u8>,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        let mut inter = Interpreter {
            registers: Registers::default(),
            memory: Memory::default(),
            key_reg: 0,
            program_handler: ProgramHandler::new(),
            next_op: 0,
            timer: 0,
            t_tick: 0.0,
            sound_timer: 0,
            st_tick: 0.0,

            screen: [0; SCREEN_PIXELS],
            keys: [0; NUM_KEYS],
            waiting_on_key: None,
        };

        inter.load_font();
        return inter;
    }

    pub fn set_key(&mut self, idx: u8) {
        // debug!("Set key {}", idx);
        self.keys[idx as usize] = 1;
        if let Some(vx) = self.waiting_on_key {
            // debug!("No longer waiting on key");
            self.set_reg(vx as usize, idx);
            self.waiting_on_key = None;
        }
    }

    /// Marks they key with index `idx` as being unset
    pub fn unset_key(&mut self, idx: u8) {
        // debug!("Unset key {}", idx);
        self.keys[idx as usize] = 0;
    }

    fn load_font(&mut self) {
        for i in 0..FONT_BYTES {
            self.write_mem(i + FONT_ADDR, FONT[i]);
        }
    }

    pub fn push_stack(&mut self, address: u16) {
        if self.registers.sp == STACK_SIZE_LIMIT {
            panic!("Stack limit exceeded");
        }
        self.memory.stack.push(address);
        self.registers.sp += 1;
    }

    pub fn pop_stack(&mut self) -> u16 {
        match self.memory.stack.pop() {
            None => panic!("Cannot pop from empty stack"),
            Some(item) => {
                assert_ne!(self.registers.sp, -1);
                self.registers.sp -= 1;
                return item;
            }
        }
    }

    pub fn is_beeping(&self) -> bool {
        return self.timer > 0;
    }

    fn time_step(&mut self, dt: f32) {
        if self.timer > 0 {
            self.t_tick -= dt;
            if self.t_tick <= 0.0 {
                self.timer -= 1;
                self.t_tick = 1.0 / 60.0;
            }
        }
        if self.sound_timer > 0 {
            self.st_tick -= dt;
            if self.st_tick <= 0.0 {
                self.sound_timer -= 1;
                self.st_tick = 1.0 / 60.0;
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        let sub_steps = (CLOCK_HZ * dt).round() as usize;
        let ddt = dt / sub_steps as f32;

        for step in 0..sub_steps {
            self.time_step(ddt);
            if self.waiting_on_key.is_some() {
                return;
            }

            self.fetch_next_instruction();
            println!("{:?}", self.next_op);
            self.run_op();
            self.inc_pc();
        }

    }

    pub fn load_program(&mut self, filename: &str) {
        let content = self.program_handler.load_file_contents(filename);
        if (content.len() > self.memory.get_max_rom_size()) {
            // panic!("ROM size ({}) is larger than available RAM ({})!", rom_len, available_ram);
            panic!("ROM too big");
        }
        self.memory.load_program(content);
    }

    fn fetch_next_instruction(&mut self) {
        let most_sig = self.memory.memory[self.get_pc() as usize] as u16;
        let least_sig = self.memory.memory[self.get_pc() as usize + 1] as u16;
        self.next_op = (most_sig << 8) | least_sig;
    }

    fn cry(&self) {
        panic!("Unrecongized instruction {:#06x}", self.next_op)
    }

    fn run_op(&mut self) {
        let nnn = self.next_op & 0x0FFF;
        let nn = (self.next_op & 0x00FF) as u8;
        let n = (self.next_op & 0x000F) as u8;
        let x = ((self.next_op & 0x0F00) >> 8) as usize;
        let y = ((self.next_op & 0x00F0) >> 4) as usize;
        match (self.next_op & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => self.cls(),
                    0xEE => self.ret(),
                    _ => {self.cry()}
                }
            },
            0x1 => self.jp(nnn),
            0x2 => self.call(nnn),
            0xb => self.jp_add(nnn),
            0x3 => self.seb(x, nn),
            0x4 => self.sneb(x, nn),
            0x9 => self.sne(x, y),
            0x5 => self.se(x, y),
            0x6 => self.ldb(x, nn),
            0xa => self.ldi(nnn),
            0x7 => self.addb(x, nn),
            0xc => self.rnd(x, nn),
            0xd => self.drw(x, y, n),
            0xe => {
                match nn {
                    0x9e => self.skp(x),
                    0xa1 => self.sknp(x),
                    _ => {self.cry()}
                }
            }
            0x8 => {
                match n {
                    0x0 => self.ld(x, y),
                    0x4 => self.add(x, y),
                    0x5 => self.sub(x, y),
                    0x7 => self.subn(x, y),
                    0x1 => self.or(x, y),
                    0x2 => self.and(x, y),
                    0x3 => self.xor(x, y),
                    0x6 => self.shr(x),
                    0xe => self.shl(x),
                    _ => {self.cry()}
                }
            },
            0xf => {
                match nn {
                    0x29 => self.ld_sprite(x),
                    0x33 => self.ld_bcd(x),
                    0x55 => self.copy_reg_mem(x),
                    0x65 => self.read_reg_mem(x),
                    0x07 => self.ldt(x),
                    0x15 => self.sdt(x),
                    0x18 => self.sst(x),
                    0x0a => self.ldk(x),
                    0x1e => self.addi(x),
                    _ => {self.cry()}
                }
            }

            _ => {self.cry()}
        }
    }

    pub fn screen_rows<'a>(&'a self) -> Chunks<'a, u8> {
        self.screen.chunks(SCREEN_WIDTH)
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    fn get_inter() -> Interpreter {
        return Interpreter::new();
    }

    #[test]
    fn test_pop_stack() {
        let mut inter = get_inter();
        inter.push_stack(1010);
        let item = inter.pop_stack();
        assert_eq!(item, 1010);
    }

    #[test]
    #[should_panic]
    fn test_pop_empty_stack() {
        let mut inter = get_inter();
        inter.pop_stack();
    }

    #[test]
    fn test_push_stack() {
        let mut inter = get_inter();
        inter.push_stack(1010);
        assert_eq!(inter.memory.stack.pop().unwrap(), 1010);
        assert_eq!(inter.registers.sp, 0);
    }

    #[test]
    #[should_panic]
    fn test_stack_overflow() {
        let mut inter = get_inter();
        for i in 0..33 {
            inter.push_stack(i);
        }
    }
}