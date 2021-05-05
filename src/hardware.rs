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
            pc: 0x200,
            dt: 0,
            st: 0
        }
    }
}

#[derive(Debug)]
pub struct Memory {
    pub size: u16,
    pub memory: Vec<u8>,
    pub program_lower_bound: usize,
    pub upper_bound: usize,
    pub stack: Chip8Stack,
}

impl Memory {
    pub fn load_program(&mut self, program: Vec<u8>) {
        for i in 0..program.len() {
            self.memory[self.program_lower_bound + i] = program[i].clone();
        }
    }

    pub fn get_max_rom_size(&self) -> usize {
        return self.upper_bound - self.program_lower_bound;
    }
}

impl Default for Memory {
    fn default() -> Memory {
        return Memory {
            size: 4096,
            memory: vec![0u8; 0xfff],
            program_lower_bound: 0x200,
            upper_bound: 0xfff,
            stack: Chip8Stack::new(),
        }
    }
}

#[derive(Debug)]
pub struct Chip8Stack {
    data: Vec<u16>
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

    pub fn is_empty(&mut self) -> bool {
        return self.data.is_empty();
    }
}