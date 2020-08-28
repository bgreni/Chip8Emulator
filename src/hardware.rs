use std::ops::Index;

#[derive(Debug)]
pub struct Registers {
    pub gen_regs: [u8; 16],
    pub i: u16, 
    pub sp: i8,
    pub pc: u16,
    pub dt: u8,
    pub st: u8
}

impl Default for Registers {
    fn default() -> Registers {
        return Registers {
            gen_regs: [0u8; 16],
            i: 0, 
            sp: -1,
            pc: 0,
            dt: 0,
            st: 0
        }
    }
}

pub struct Memory {
    pub size: u16,
    pub memory: Vec<u8>,
    pub program_lower_bound: u16,
    pub upper_bound: u16,
    pub stack: Chip8Stack,
}

impl Default for Memory {
    fn default() -> Memory {
        return Memory {
            size: 4096,
            memory: vec![0u8; 4096],
            program_lower_bound: 0x200,
            upper_bound: 0xfff,
            stack: Chip8Stack::new(),
        }
    }
}

pub struct Chip8Stack {
    pub data: Vec<u16>
}

impl Chip8Stack {

    pub fn new() -> Self {
        return Chip8Stack {
            data: Vec::new()
        }
    }

    pub fn push(&mut self, address: u16) {
        self.data.push(address);
    }

    pub fn pop(&mut self) -> Option<u16> {
        return self.data.pop();
    }
}