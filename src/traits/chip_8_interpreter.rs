use crate::interpreter::Interpreter;
use crate::program_handler::ProgramHandler;
use crate::hardware::Memory;
use crate::traits::instructions::{Helpers, Instructions};
use crate::drivers::screen::Screen;
use crate::drivers::input::Inputs;
use piston::input::Button;

pub trait Chip8Interpreter {
    fn load_program(&mut self, filename: &str);
    fn fetch_next_instruction(&mut self);
    fn run_op(&mut self, screen: &mut Screen, input: &mut Inputs, key: Option<Button>);
    fn cry(&self);
}

impl Chip8Interpreter for Interpreter {
    fn load_program(&mut self, filename: &str) {
        let content = self.program_handler.load_file_contents(filename);
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

    fn run_op(&mut self, screen: &mut Screen, input: &mut Inputs, key: Option<Button>) {
        let nnn = self.next_op & 0x0FFF;
        let nn = (self.next_op & 0x0FF) as u8;
        let n = (self.next_op & 0x00F) as u8;
        let x = ((self.next_op & 0x0F00) >> 8) as usize;
        let y = ((self.next_op & 0x00F0) >> 4) as usize;
        match (self.next_op & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => self.cls(screen),
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
            0xd => self.drw(x, y, n, screen),
            0xe => {
                match nn {
                    0x9e => self.skp(x, input, key),
                    0xa1 => self.sknp(x, input, key),
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
}

#[cfg(test)]
mod Chip8_tests {
    use super::*;

    #[test]
    fn test_fetch_instruction() {
        let mut inter = Interpreter::new();
        inter.memory.memory[0x200] = 0x23;
        inter.memory.memory[0x201] = 0x45;

        assert_eq!(inter.fetch_next_instruction(), 0x2345);
    }

}