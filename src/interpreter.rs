/// Logical construct of the interperter itself.
/// Defines the interface of the interperter, and implements 
/// all top level functionality
use crate::hardware::{Registers, Memory};
use crate::program_handler::ProgramHandler;

const STACK_SIZE_LIMIT: i8 = 31;

#[allow(dead_code)]
pub struct Interpreter {
    pub registers: Registers,
    pub memory: Memory,
    pub wait_for_key: bool,
    pub key_reg: usize,
    pub program_handler: ProgramHandler,
    pub next_op: u16,
}


impl Interpreter {
    pub fn new() -> Interpreter {
        return Interpreter {
            registers: Registers::default(),
            memory: Memory::default(),
            wait_for_key: false,
            key_reg: 0,
            program_handler: ProgramHandler::new(),
            next_op: 0,
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