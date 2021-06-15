use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::hardware::instruction::{Instruction, OPCODE_LEN};
use bit_vec::BitVec;
use std::thread;
use std::time::Duration;

const STACK_SIZE: usize = 16;
const MEM_SIZE: usize = 4096;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
pub const PIXEL_COUNT: usize = (WIDTH * HEIGHT) as usize;

const FONT_SIZE: usize = 80;

static FONTSET: [u8; FONT_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub fn version() -> &'static str {
    concat!(env!("CARGO_PKG_VERSION_MAJOR"),
    ".",
    env!("CARGO_PKG_VERSION_MINOR"),
    ".",
    env!("CARGO_PKG_VERSION_PATCH")
    )
}

pub struct Chip8 {
    memory: Vec<u8>,
    registers: Vec<u8>,
    I: u16,
    pc: u16,
    pub screen: [u8; PIXEL_COUNT],
    dt: u8,
    st: u8,
    stack: Vec<u16>,
    pub keys: [u8; 16],
    // sp: u8,

    pub draw: bool,
    do_sound: bool,
}


impl Default for Chip8 {
    fn default() -> Self {
        let mut emu = Chip8 {
            memory: vec![0; MEM_SIZE],
            registers: vec![0; STACK_SIZE],
            I: 0,
            pc: 0x200,
            screen: [0; PIXEL_COUNT],
            dt: 0,
            st: 0,
            stack: Vec::new(),
            keys: [0; 16],
            draw: false,
            do_sound: false,
        };

        // load font
        for i in 0..FONT_SIZE {
            emu.memory[i] = FONTSET[i];
        }

        return emu;
    }
}

impl Chip8 {
    pub fn load_program(&mut self, path: &str) {
        let path = Path::new(path);
        let display = path.display();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(E) => panic!("Failed to open file {}: {}", display, &E)
        };

        let mut data = Vec::new();

        file.read_to_end(&mut data).expect("Couldn't read file");

        for (i, byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    fn inc_pc(&mut self) {
        self.pc += OPCODE_LEN;
    }

    fn set_vf(&mut self) {
        self.registers[0xF] = 1
    }

    fn unset_vf(&mut self) {
        self.registers[0xF] = 0
    }

    fn get_vf(&self) -> u8 {
        return self.registers[0xF];
    }

    fn do_set_vf(&mut self, val: u8) {
        self.registers[0xF] = val;
    }

    fn inc_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        self.do_sound = false;
        if self.st > 0 {
            if self.st == 1 {
                self.do_sound = true;
            }
            self.st -= 1;
        }
    }

    pub fn run_cycle(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8 |
            self.memory[(self.pc + 1) as usize] as u16;

        self.execute_instruction(opcode);
        self.inc_timers();

        thread::sleep(Duration::from_millis(2));
    }

    pub fn execute_instruction(&mut self, opcode: u16) {
        let instruction = Instruction::new(opcode);

        let x = instruction.get_x() as usize;
        let y = instruction.get_y() as usize;
        let addr = instruction.get_addr();
        let nibble = instruction.get_nibble();
        let kk = instruction.get_kk();

        match instruction.get_top_nib() {
            0 => {
                match instruction.get_kk() {
                    // Clear Screen
                    0xE0 => {
                        for i in 0..PIXEL_COUNT {
                            self.screen[i] = 0;
                        }
                        self.draw = true;
                    },
                    // Return
                    0xEE => {
                        self.pc = self.stack.pop().unwrap();
                    }
                    _ => self.cry(opcode),
                }
            },
            // Jump
            1 => {
                self.pc = addr;
                self.pc -= OPCODE_LEN;
            },
            // Call
            2 => {
                self.stack.push(self.pc);
                self.pc = addr;
                self.pc -= OPCODE_LEN;
            },
            // Skip x == kk
            3 => {
                if self.registers[x] == kk {
                    self.inc_pc();
                }
            },
            // Skip x != kk
            4 => {
                if self.registers[x] != kk {
                    self.inc_pc();
                }
            },
            // Skip if x == y
            5 => {
                if self.registers[x] == self.registers[y] {
                    self.inc_pc();
                }
            },
            // Set x = kk
            6 => {
                self.registers[x] = kk;
            }
            // Add kk to x
            7 => {
                self.registers[x] = self.registers[x].wrapping_add(kk);
            },
            8 => {
                match nibble {
                    // Set x = y
                    0 => {
                        self.registers[x] = self.registers[y];
                    },
                    // Set x |= y
                    1 => {
                        self.registers[x] |= self.registers[y];
                    },
                    // Set x &= y
                    2 => {
                        self.registers[x] &= self.registers[y];
                    },
                    // Set x ^= y
                    3 => {
                        self.registers[x] ^= self.registers[y];
                    },
                    // Add y to x
                    4 => {
                        if self.registers[y] > (0xFF - self.registers[x]) {
                            self.set_vf();
                        } else {
                            self.unset_vf();
                        }
                        self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);
                    },
                    // Sub y from x
                    5 => {
                        if self.registers[y] < self.registers[x] {
                            self.set_vf();
                        } else {
                            self.unset_vf();
                        }
                        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
                    },
                    // Shift x right
                    6 => {
                        self.do_set_vf((self.registers[x] & 0x0001) as u8);
                        self.registers[x] >>= 1;
                    },
                    // x = y - x
                    7 => {
                        if self.registers[x] < self.registers[y] {
                            self.set_vf();
                        } else {
                            self.unset_vf();
                        }
                        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                    },
                    // Shift x left
                    0xE => {
                        self.do_set_vf(((self.registers[x] & 0x80) as u8) >> 7);
                        self.registers[x] <<= 1;
                    }

                    _ => self.cry(opcode),
                }
            },
            // Skip x != y
            9 => {
                if self.registers[x] != self.registers[y] {
                    self.inc_pc();
                }
            },
            // Set I
            0xA => {
                self.I = addr;
            },
            // Jump add
            0xB => {
                self.pc = addr + self.registers[0] as u16 - OPCODE_LEN;
            },
            // Set x = kk & rand
            0xC => {
                self.registers[x] = kk & rand::random::<u8>();
            },
            // Draw sprite
            0xD => {
                let x = self.registers[x] as i32;
                let y = self.registers[y]  as i32;
                let n = nibble as u16;

                self.unset_vf();
                for i in self.I..(self.I + n) {
                    let row = (i - self.I) as u8;
                    let bits = BitVec::from_bytes(&[self.memory[i as usize]]);

                    for j in 0..8 {
                        let xs = x + j;
                        let ys = y + row as i32;

                        if in_bounds(xs, ys) {
                            let address = (64 * ys) + xs;
                            if bits[j as usize] {
                                if self.screen[address as usize] == 1 {
                                    self.set_vf();
                                }
                                self.screen[address as usize] ^= 1;
                            }
                        }
                    }
                }
                self.draw = true;
            },
            0xE => {
              match kk {
                  // Skips if key stored in x is pressed
                  0x9E => {
                    if self.keys[self.registers[x] as usize] == 1 {
                        self.inc_pc();
                    }
                  },
                  // Skip if not pressed
                  0xA1 => {
                      if self.keys[self.registers[x] as usize] == 0{
                          self.inc_pc();
                      }
                  },
                  _ => self.cry(opcode),
              }
            },
            0xF => {
                match kk {
                    // Set x to dt
                    0x07 => {
                        self.registers[x] = self.dt;
                    },
                    // Wait for keypress
                    0x0A => {
                        let mut found = false;
                        for k in 0..15 {
                            if self.keys[k as usize] == 1 {
                                self.registers[x] = k as u8;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            self.pc -= OPCODE_LEN;
                        }
                    },
                    // Set dt to x
                    0x15 => {
                        self.dt = self.registers[x];
                    },
                    // Set st to x
                    0x18 => {
                        self.st = self.registers[x];
                    },
                    // Add x to I
                    0x1E => {
                        self.I += self.registers[x] as u16;
                    },
                    // Set I top loc of char
                    0x29 => {
                        self.I = 5 * self.registers[x] as u16;
                    },
                    // store bcd of x
                    0x33 => {
                        let val = self.registers[x];
                        let index = self.I as usize;
                        self.memory[index] = val / 100;
                        self.memory[index + 1] = (val / 10) % 10;
                        self.memory[index + 2] = (val % 100) % 10;
                    }
                    // Store registers
                    0x55 => {
                        for i in 0..(x + 1) as usize {
                            self.memory[self.I as usize + i] = self.registers[i];
                        }
                    },
                    // load registers
                    0x65 => {
                        for i in 0..(x + 1) as usize {
                            self.registers[i] = self.memory[self.I as usize + i];
                        }
                    },
                    _ => self.cry(opcode),
                }
            }
            _ => self.cry(opcode),
        }
        self.inc_pc();
    }

    fn cry(&self, opcode: u16) {
        panic!("Opcode {:#X} is bad", opcode);
    }
}


fn in_bounds(xs: i32, ys: i32) -> bool {
    return (0..WIDTH as i32).contains(&xs) && (0..HEIGHT as i32).contains(&ys);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_registers() {
        let mut inter = Chip8::default();
        inter.registers[9] = 4;
        for i in 0..5usize {
            inter.memory[i] = i as u8;
        }

        inter.execute_instruction(0xF965);

        for i in 0..5usize {
            assert_eq!(inter.registers[i], i as u8);
        }
    }

    #[test]
    fn test_store_registers() {
        let mut inter = Chip8::default();
        inter.registers[9] = 4;
        for i in 0..5usize {
            inter.registers[i] = i as u8;
        }

        inter.execute_instruction(0xF955);

        for i in 0..5usize {
            assert_eq!(inter.memory[i], i as u8);
        }
    }

    #[test]
    fn test_bcd() {
        let mut inter = Chip8::default();
        inter.registers[1] = 123;
        inter.execute_instruction(0xF133);
        for i in 0..3 {
            assert_eq!(inter.memory[i] as usize, i+1);
        }
    }

    #[test]
    fn test_set_I_to_char() {
        let mut inter = Chip8::default();
        inter.registers[1] = 20;
        inter.I = 3;
        inter.execute_instruction(0xF129);
        assert_eq!(inter.I, 20 * 5);
    }

    #[test]
    fn test_add_x_to_I() {
        let mut inter = Chip8::default();
        inter.registers[1] = 20;
        inter.I = 3;
        inter.execute_instruction(0xF11E);
        assert_eq!(inter.I, 23);
    }

    #[test]
    fn test_set_st_to_x() {
        let mut inter = Chip8::default();
        inter.registers[1] = 10;
        inter.execute_instruction(0xF118);
        assert_eq!(inter.st, 10);
    }

    #[test]
    fn test_set_dt_to_x() {
        let mut inter = Chip8::default();
        inter.registers[1] = 10;
        inter.execute_instruction(0xF115);
        assert_eq!(inter.dt, 10);
    }

    #[test]
    fn test_wait_keypress_not_pressed() {
        let mut inter = Chip8::default();
        let last_pc = inter.pc;
        inter.execute_instruction(0xF10A);
        assert_eq!(last_pc, inter.pc);
    }

    #[test]
    fn test_wait_keypress_pressed() {
        let mut inter = Chip8::default();
        inter.keys[1] = 1;
        let last_pc = inter.pc;
        inter.execute_instruction(0xF10A);
        assert_eq!(last_pc + 2, inter.pc);
        assert_eq!(inter.registers[1], 1);
    }

    #[test]
    fn test_set_x_to_dt() {
        let mut inter = Chip8::default();
        inter.dt = 20;
        inter.execute_instruction(0xF107);
        assert_eq!(inter.registers[1], 20);
    }

    #[test]
    fn test_skip_not_key_press_true() {
        let mut inter = Chip8::default();
        let last_pc = inter.pc;
        inter.registers[1] = 4;
        inter.keys[4] = 0;
        inter.execute_instruction(0xE1A1);
        assert_eq!(inter.pc, last_pc + 4);
    }

    #[test]
    fn test_skip_not_key_press_false() {
        let mut inter = Chip8::default();
        let last_pc = inter.pc;
        inter.registers[1] = 4;
        inter.keys[4] = 1;
        inter.execute_instruction(0xE1A1);
        assert_eq!(inter.pc, last_pc + 2);
    }

    #[test]
    fn test_skip_key_press_false() {
        let mut inter = Chip8::default();
        let last_pc = inter.pc;
        inter.registers[1] = 4;
        inter.keys[4] = 0;
        inter.execute_instruction(0xE19E);
        assert_eq!(inter.pc, last_pc + 2);
    }

    #[test]
    fn test_skip_key_press_true() {
        let mut inter = Chip8::default();
        let last_pc = inter.pc;
        inter.registers[1] = 4;
        inter.keys[4] = 1;
        inter.execute_instruction(0xE19E);
        assert_eq!(inter.pc, last_pc + 4);
    }

    #[test]
    fn test_in_bounds() {
        assert!(in_bounds(23, 12));
    }

    #[test]
    fn test_in_bounds_false1() {
        assert!(!in_bounds(23, 87));
    }

    #[test]
    fn test_in_bounds_false2() {
        assert!(!in_bounds(98, 87));
    }

    #[test]
    fn test_in_bounds_false3() {
        assert!(!in_bounds(98, 12));
    }

    #[test]
    fn test_in_bounds_false4() {
        assert!(!in_bounds(64, 12));
    }

    #[test]
    fn test_jump_add() {
        let mut inter = Chip8::default();
        inter.registers[0] = 10;
        inter.execute_instruction(0xB345);
        assert_eq!(inter.pc, 0x345 + 10);
    }

    #[test]
    fn test_set_I() {
        let mut inter = Chip8::default();
        inter.execute_instruction(0xA645);
        assert_eq!(inter.I, 0x645);
    }

    #[test]
    fn test_skip_x_noteq_y_false() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 5;
        inter.registers[6] = 5;
        inter.execute_instruction(0x9560);
        assert_eq!(inter.pc, curr_pc + 2);
    }

    #[test]
    fn test_skip_x_noteq_y_true() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 5;
        inter.registers[6] = 10;
        inter.execute_instruction(0x9560);
        assert_eq!(inter.pc, curr_pc + 4);
    }

    #[test]
    fn test_left_x_right_nobit() {
        let mut inter = Chip8::default();
        inter.registers[2] = 0x14;
        inter.execute_instruction(0x820E);
        assert_eq!(inter.registers[2], 0x14 << 1);
        assert_eq!(inter.get_vf(), 0);
    }

    #[test]
    fn test_left_x_right_bit() {
        let mut inter = Chip8::default();
        inter.registers[2] = 0x86;
        inter.execute_instruction(0x820E);
        assert_eq!(inter.registers[2], 0x86 << 1);
        assert_eq!(inter.get_vf(), 1);
    }

    #[test]
    fn test_x_eq_y_sub_x_wrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 4;
        inter.registers[3] = 0;
        inter.execute_instruction(0x8537);
        assert_eq!(inter.registers[5], 252);
        assert_eq!(inter.get_vf(), 0);
    }

    #[test]
    fn test_x_eq_y_sub_x_nowrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 4;
        inter.registers[3] = 10;
        inter.execute_instruction(0x8537);
        assert_eq!(inter.registers[5], 6);
        assert_eq!(inter.get_vf(), 1);
    }

    #[test]
    fn test_shift_x_right_even() {
        let mut inter = Chip8::default();
        inter.registers[2] = 20;
        inter.execute_instruction(0x8206);
        assert_eq!(inter.registers[2], 10);
        assert_eq!(inter.get_vf(), 0);
    }

    #[test]
    fn test_shift_x_right_odd() {
        let mut inter = Chip8::default();
        inter.registers[2] = 21;
        inter.execute_instruction(0x8206);
        assert_eq!(inter.registers[2], 21/2);
        assert_eq!(inter.get_vf(), 1);
    }

    #[test]
    fn test_x_sub_y_wrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 0;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8535);
        assert_eq!(inter.registers[5], 252);
        assert_eq!(inter.get_vf(), 0);
    }

    #[test]
    fn test_x_sub_y_nowrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 10;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8535);
        assert_eq!(inter.registers[5], 6);
        assert_eq!(inter.get_vf(), 1);
    }

    #[test]
    fn test_x_add_y_wrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 255;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8534);
        assert_eq!(inter.registers[5], 3);
        assert_eq!(inter.get_vf(), 1);
    }

    #[test]
    fn test_x_add_y_nowrap() {
        let mut inter = Chip8::default();
        inter.registers[5] = 10;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8534);
        assert_eq!(inter.registers[5], 14);
        assert_eq!(inter.get_vf(), 0);
    }

    #[test]
    fn test_x_xor_y() {
        let mut inter = Chip8::default();
        inter.registers[5] = 10;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8533);
        assert_eq!(inter.registers[5], 10 ^ 4);
    }

    #[test]
    fn test_x_and_y() {
        let mut inter = Chip8::default();
        inter.registers[5] = 10;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8532);
        assert_eq!(inter.registers[5], 10 & 4);
    }

    #[test]
    fn test_x_or_y() {
        let mut inter = Chip8::default();
        inter.registers[5] = 10;
        inter.registers[3] = 4;
        inter.execute_instruction(0x8531);
        assert_eq!(inter.registers[5], 10 | 4);
    }

    #[test]
    fn test_set_x_y() {
        let mut inter = Chip8::default();
        inter.registers[3] = 11;
        inter.execute_instruction(0x8430);
        assert_eq!(inter.registers[4], 11);
    }

    #[test]
    fn test_add_kk_x_nowrap() {
        let mut inter = Chip8::default();
        inter.execute_instruction(0x7733);
        assert_eq!(inter.registers[7], 0x33);
    }

    #[test]
    fn test_add_kk_x_wrap() {
        let mut inter = Chip8::default();
        inter.registers[7] = 255;
        inter.execute_instruction(0x770A);
        assert_eq!(inter.registers[7], 9);
    }

    #[test]
    fn test_clear_screen() {
        let mut inter = Chip8::default();
        inter.screen = [2; PIXEL_COUNT];

        inter.execute_instruction(0x00E0);
        assert_eq!(inter.screen.iter().sum::<u8>(), 0);
    }

    #[test]
    fn test_return() {
        let mut inter = Chip8::default();
        inter.pc = 700;
        inter.stack.push(512);
        inter.execute_instruction(0x00EE);
        assert_eq!(inter.pc, 514);
    }

    #[test]
    fn test_jump() {
        let mut inter = Chip8::default();
        inter.execute_instruction(0x1666);
        assert_eq!(inter.pc, 0x666);
    }

    #[test]
    fn test_call() {
        let mut inter = Chip8::default();
        inter.pc = 0x444;
        inter.execute_instruction(0x2555);
        assert_eq!(inter.pc, 0x555);
        assert_eq!(*inter.stack.last().unwrap(), 0x444);
    }

    #[test]
    fn test_skip_x_eq_kk() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 5;
        inter.execute_instruction(0x3505);
        assert_eq!(inter.pc, curr_pc + 4);
    }

    #[test]
    fn test_skip_x_eq_kk_not() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 8;
        inter.execute_instruction(0x3505);
        assert_eq!(inter.pc, curr_pc + 2);
    }

    #[test]
    fn test_skip_x_noteq_kk() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 5;
        inter.execute_instruction(0x4508);
        assert_eq!(inter.pc, curr_pc + 4);
    }

    #[test]
    fn test_skip_x_noteq_kk_not() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[5] = 5;
        inter.execute_instruction(0x4505);
        assert_eq!(inter.pc, curr_pc + 2);
    }

    #[test]
    fn test_skip_x_eq_y() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[2] = 5;
        inter.registers[3] = 5;
        inter.execute_instruction(0x5231);
        assert_eq!(inter.pc, curr_pc + 4);
    }

    #[test]
    fn test_skip_x_eq_y_not() {
        let mut inter = Chip8::default();
        let curr_pc = inter.pc;
        inter.registers[2] = 5;
        inter.registers[3] = 10;
        inter.execute_instruction(0x5231);
        assert_eq!(inter.pc, curr_pc + 2);
    }

    #[test]
    fn test_set_x_kk() {
        let mut inter = Chip8::default();
        inter.execute_instruction(0x6744);
        assert_eq!(inter.registers[7], 0x44);
    }
}
