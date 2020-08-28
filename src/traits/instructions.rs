use crate::interpreter::Interpreter;
use crate::drivers::input::{Inputs, val_in_keys};
use rand::Rng;
use std::io;
use std::io::stdin;
use std::io::Read;


pub trait Helpers {
    fn get_reg(&mut self, index: usize) -> u8;
    fn set_reg(&mut self, index: usize, val: u8);
    fn set_vf(&mut self);
    fn unset_vf(&mut self);
    fn set_i(&mut self, val: u16);
    fn get_i(&mut self) -> u16;
    fn handle_vf(&mut self, condition: bool);
    fn inc_pc(&mut self);
    fn get_pc(&mut self) -> u16;
    fn get_dt(&mut self) -> u8;
    fn set_dt(&mut self, val: u8);
    fn get_st(&mut self) -> u8;
    fn set_st(&mut self, val: u8);
}

impl Helpers for Interpreter {
    fn get_reg(&mut self, index: usize) -> u8 {
        return self.registers.gen_regs[index].clone();
    }

    fn set_reg(&mut self, index: usize, val: u8) {
        self.registers.gen_regs[index] = val;
    }

    fn set_vf(&mut self) {
        self.registers.gen_regs[15] = 1;
    }

    fn unset_vf(&mut self) {
        self.registers.gen_regs[15] = 0;
    }

    fn set_i(&mut self, val: u16) {
        self.registers.i = val;
    }

    fn get_i(&mut self) -> u16 {
        return self.registers.i;
    }

    fn handle_vf(&mut self, condition: bool) {
        if condition {
            self.set_vf();
        } else {
            self.unset_vf();
        }
    }

    fn inc_pc(&mut self) {
        self.registers.pc += 2;
    }

    fn get_pc(&mut self) -> u16 {
        return self.registers.pc;
    }

    fn get_dt(&mut self) -> u8 {
        return self.registers.dt;
    }

    fn set_dt(&mut self, val: u8) {
        self.registers.dt = val;
    }

    fn get_st(&mut self) -> u8 {
        return self.registers.st;
    }

    fn set_st(&mut self, val: u8) {
        self.registers.st = val;
    }
}


pub trait Instructions {
    // fn cls();
    fn ret();
    fn jp(&mut self, address: u16);
    fn jp_add(&mut self, address: u16);
    // fn call(address: u16);
    fn seb(&mut self, regx: usize, byte: u8);
    fn sneb(&mut self, regx: usize, byte: u8);
    fn sne(&mut self, regx: usize, regy: usize);
    fn se(&mut self, regx: usize, regy: usize);
    fn ldb(&mut self, regx: usize, byte: u8);
    fn ld(&mut self, regx: usize, regy: usize);
    // fn ld_sprite(register: u8);
    // fn ld_bcd(register: u8);
    // fn copy_reg_mem(limit_reg: u8);
    // fn read_reg_mem(limit_reg: u8);
    fn ldi(&mut self, val: u16);
    fn ldt(&mut self, regx: usize);
    fn sdt(&mut self, regx: usize);
    fn sst(&mut self, regx: usize);
    fn ldk(&mut self, regx: usize);
    fn addb(&mut self, regx: usize, byte: u8);
    fn add(&mut self, regx: usize, regy: usize);
    fn addi(&mut self, regx: usize);
    fn sub(&mut self, regx: usize, regy: usize);
    fn subn(&mut self, regx: usize, regy: usize);
    fn or(&mut self, regx: usize, regy: usize);
    fn and(&mut self, regx: usize, regy: usize);
    fn xor(&mut self, regx: usize, regy: usize);
    fn shr(&mut self, regx: usize);
    fn shl(&mut self, regx: usize);
    fn rnd(&mut self, regx: usize, byte: u8);
    // fn drw(register1: u8, register2: u8, nibble: u8);
    fn skp(&mut self, regx: usize, input_driver: &Inputs);
    fn sknp(&mut self, regx: usize, input_driver: &Inputs);
}


impl Instructions for Interpreter {
    /// Add byte to register
    fn addb(&mut self, regx: usize, byte: u8) {
        let regx_val = self.get_reg(regx);
        self.set_reg(regx, regx_val.wrapping_add(byte));
    }

    /// Vx = Vx + Vy
    /// set vf if result is greater than 255 (max 8 bit int)
    fn add(&mut self, regx: usize, regy: usize) {
        let result: u16 = self.get_reg(regx) as u16 + self.get_reg(regy) as u16;
        self.handle_vf(result > u8::MAX as u16);
        self.set_reg(regx, result as u8);
    }

    /// I = Vx + I
    fn addi(&mut self, regx: usize) {
        let i_val = self.get_i();
        let regx_val = self.get_reg(regx) as u16;
        self.set_i(i_val.wrapping_add(regx_val));
    }

    /// Vx = Vx - Vy
    /// set Vf if Vx > Vy
    fn sub(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.handle_vf(regx_val > regy_val);
        self.set_reg(regx, regx_val.wrapping_sub(regy_val));
    }

    /// Vx = Vy - Vx
    /// set Vf if Vy > Vx
    fn subn(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.handle_vf(regy_val > regx_val);
        self.set_reg(regx, regy_val.wrapping_sub(regx_val));

    }

    /// Vx = Vx or Vy (bitwise)
    fn or(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val | regy_val);
    }

    /// Vx = Vx and Vy (bitwise)
    fn and(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val & regy_val);
    }

    /// Vx = Vx xor Vy (bitwise)
    fn xor(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val ^ regy_val);
    }

    /// Vx = Vx / 2
    /// sets Vf if least significant bit of Vx is 1
    fn shr(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.handle_vf(regx_val & 0x1 == 1);
        self.set_reg(regx, regx_val >> 1);
    }

    /// Vx = Vx * 2
    /// sets Vf if most significant bit of Vx is 1
    fn shl(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.handle_vf(regx_val & 0x80 == 0x80);
        self.set_reg(regx, regx_val << 1);
    }

    /// inc pc if Vx == byte
    fn seb(&mut self, regx: usize, byte: u8) {
        if self.get_reg(regx) == byte {
            self.inc_pc();
        }
    }

    /// inc pc if Vx != byte
    fn sneb(&mut self, regx: usize, byte: u8) {
        if self.get_reg(regx) != byte {
            self.inc_pc();
        }
    }

    /// inc pc if Vx != Vy
    fn sne(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        if regx_val != regy_val {
            self.inc_pc();
        }
    }

    /// inc pc if Vx == Vy
    fn se(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        if regx_val == regy_val {
            self.inc_pc();
        }   
    }

    /// set Vx = byte
    fn ldb(&mut self, regx: usize, byte: u8) {
        self.set_reg(regx, byte);
    }

    /// set Vx = Vy
    fn ld(&mut self, regx: usize, regy: usize) {
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regy_val);
    }

    /// I = val
    fn ldi(&mut self, val: u16) {
        self.set_i(val);
    }

    /// Vx = DT
    fn ldt(&mut self, regx: usize) {
        let dt_val = self.get_dt();
        self.set_reg(regx, dt_val);
    }

    /// DT = Vx
    fn sdt(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.set_dt(regx_val);
    }

    /// ST = Vx
    fn sst(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.set_st(regx_val);
    }

    /// gen rand number [0...255]
    /// Vx = rand & byte
    fn rnd(&mut self, regx: usize, byte: u8) {
        self.set_reg(regx, rand::thread_rng().gen::<u8>() & byte);
    }

    /// Wait for keypress and store value of key in Vx
    fn ldk(&mut self, regx: usize) {
        self.wait_for_key = true;
        self.key_reg = regx;
    }

    /// Inc pc if key value held in Vx is pressed
    fn skp(&mut self, regx: usize, input_driver: &Inputs) {
        let keys = input_driver.check_key_presses();
        let target_key: usize = self.get_reg(regx) as usize;
        if val_in_keys(&keys, target_key) {
            self.inc_pc();
        }
    }

    /// Inc pc if key value held in Vx is not pressed
    fn sknp(&mut self, regx: usize, input_driver: &Inputs) {
        let keys = input_driver.check_key_presses();
        let target_key: usize = self.get_reg(regx) as usize;
        if !val_in_keys(&keys, target_key) {
            self.inc_pc();
        }
    }

    /// Set PC = address
    fn jp(&mut self, address: u16) {
        self.registers.pc = address;
    }

    /// Set PC = address + V0
    fn jp_add(&mut self, address: u16) {
        self.registers.pc = address + self.get_reg(0) as u16;
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    fn get_inter() -> Interpreter {
        return Interpreter::new();
    }

    #[test]
    fn test_jp_add() {
        let mut inter = get_inter();
        inter.set_reg(0, 10);
        inter.jp_add(100);
        assert_eq!(inter.get_pc(), 110);
    }

    // tests for jp
    #[test]
    fn test_jp() {
        let mut inter = get_inter();
        inter.jp(100);
        assert_eq!(inter.get_pc(), 100);
    }

    // tests for sst
    #[test]
    fn test_sst() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.sst(1);
        assert_eq!(inter.get_st(), 10);
    }

    // tests for sdt
    #[test]
    fn test_sdt() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.sdt(1);
        assert_eq!(inter.get_dt(), 10);
    }

    // tests for ldt
    #[test]
    fn test_ldt() {
        let mut inter = get_inter();
        inter.set_dt(10);
        inter.ldt(1);
        assert_eq!(inter.get_reg(1), 10);
    }

    // tests for ldi
    #[test]
    fn test_ldi() {
        let mut inter = get_inter();
        inter.ldi(30);
        assert_eq!(inter.get_i(), 30);
    }

    // tests for ld
    #[test]
    fn test_ld() {
        let mut inter = get_inter();
        inter.set_reg(2, 15);
        inter.ld(1, 2);
        assert_eq!(inter.get_reg(1), 15);
    }

    // tests for ldb
    #[test]
    fn test_ldb() {
        let mut inter = get_inter();
        inter.ldb(1, 10);
        assert_eq!(inter.get_reg(1), 10);
    }

    // test se
    #[test]
    fn test_se_true() {
        let mut inter = get_inter();
        let curr_pc = inter.get_pc();
        inter.set_reg(1, 10);
        inter.set_reg(2, 10);
        inter.se(1, 2);
        assert_eq!(inter.get_pc(), curr_pc + 2);
    }

    #[test]
    fn test_se_false() {
        let mut inter = get_inter();
        let curr_pc = inter.get_pc();
        inter.set_reg(1, 10);
        inter.set_reg(2, 4);
        inter.se(1, 2);
        assert_eq!(inter.get_pc(), curr_pc);
    }

    // test sne
    #[test]
    fn test_sne_true() {
        let mut inter = get_inter();
        let curr_pc = inter.get_pc();
        inter.set_reg(1, 10);
        inter.set_reg(2, 4);
        inter.sne(1, 2);
        assert_eq!(inter.get_pc(), curr_pc + 2);
    }

    #[test]
    fn test_sne_false() {
        let mut inter = get_inter();
        let curr_pc = inter.get_pc();
        inter.set_reg(1, 10);
        inter.set_reg(2, 10);
        inter.sne(1, 2);
        assert_eq!(inter.get_pc(), curr_pc);
    }

    // test sneb
    #[test]
    fn test_sneb_true() {
        let mut inter = get_inter();
        let byte = 10;
        inter.set_reg(1, 5);
        let curr_pc = inter.get_pc();
        inter.sneb(1, byte);
        assert_eq!(inter.get_pc(), curr_pc + 2);
    }

    #[test]
    fn test_sneb_false() {
        let mut inter = get_inter();
        let byte = 10;
        inter.set_reg(1, 10);
        let curr_pc = inter.get_pc();
        inter.sneb(1, byte);
        assert_eq!(inter.get_pc(), curr_pc);
    }

    // test seb
    #[test]
    fn test_seb_true() {
        let mut inter = get_inter();
        let byte = 10;
        inter.set_reg(1, 10);
        let curr_pc = inter.get_pc();
        inter.seb(1, byte);
        assert_eq!(inter.get_pc(), curr_pc + 2);
    }

    #[test]
    fn test_seb_false() {
        let mut inter = get_inter();
        let byte = 10;
        inter.set_reg(1, 2);
        let curr_pc = inter.get_pc();
        inter.seb(1, byte);
        assert_eq!(inter.get_pc(), curr_pc);
    }

    // test shr
    #[test]
    fn test_shr_0() {
        let mut inter = get_inter();
        let val: u8 = 0x10;
        inter.set_reg(1, val);
        inter.shr(1);
        assert_eq!(inter.get_reg(1), val >> 1);
        assert_eq!(inter.get_reg(15), 0);
    }

    #[test]
    fn test_shr_1() {
        let mut inter = get_inter();
        let val: u8 = 0x11;
        inter.set_reg(1, val);
        inter.shr(1);
        assert_eq!(inter.get_reg(1), val >> 1);
        assert_eq!(inter.get_reg(15), 1);
    }

    // test shl
    #[test]
    fn test_shl_0() {
        let mut inter = get_inter();
        let val: u8 = 0x1;
        inter.set_reg(1, val);
        inter.shl(1);
        assert_eq!(inter.get_reg(1), val << 1);
        assert_eq!(inter.get_reg(15), 0);
    }

    #[test]
    fn test_shl_1() {
        let mut inter = get_inter();
        let val: u8 = 0x90;
        inter.set_reg(1, val);
        inter.shl(1);
        assert_eq!(inter.get_reg(1), val << 1);
        assert_eq!(inter.get_reg(15), 1);
    }

    // tests for xor
    #[test]
    fn test_xor() {
        let mut inter = get_inter();
        inter.set_reg(1, 0x10);
        inter.set_reg(2, 0x11);
        inter.xor(1, 2);
        assert_eq!(inter.get_reg(1), 0x1);
    }

    // tests for and
    #[test]
    fn test_and() {
        let mut inter = get_inter();
        inter.set_reg(1, 0x10);
        inter.set_reg(2, 0x11);
        inter.and(1, 2);
        assert_eq!(inter.get_reg(1), 0x10);
    }

    // tests for or
    #[test]
    fn test_or() {
        let mut inter = get_inter();
        inter.set_reg(1, 0x1);
        inter.or(2, 1);
        assert_eq!(inter.get_reg(2), 1);
    }

    // tests for subn
    #[test]
    fn test_subn() {
        let mut inter = get_inter();
        inter.set_reg(1, 3);
        inter.set_reg(2, 10);
        inter.subn(1, 2);
        assert_eq!(inter.get_reg(1), 7);
        assert_eq!(inter.get_reg(15), 1);
    }

    #[test]
    fn test_subn_underflow() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.set_reg(2, 3);
        inter.subn(1, 2);
        assert_eq!(inter.get_reg(1), 249);
        assert_eq!(inter.get_reg(15), 0);
    }

    // tests for sub
    #[test]
    fn test_sub() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.set_reg(2, 3);
        inter.sub(1, 2);
        assert_eq!(inter.get_reg(1), 7);
        assert_eq!(inter.get_reg(15), 1);
    }

    #[test]
    fn test_sub_underflow() {
        let mut inter = get_inter();
        inter.set_reg(1, 3);
        inter.set_reg(2, 10);
        inter.sub(1, 2);
        assert_eq!(inter.get_reg(1), 249);
        assert_eq!(inter.get_reg(15), 0);
    }

    // tests for addi
    #[test]
    fn test_addi() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.set_i(10);
        inter.addi(1);
        assert_eq!(inter.get_i(), 20);
    }

    #[test]
    fn test_addi_overflow() {
        let mut inter = get_inter();
        inter.set_reg(1, 10);
        inter.set_i(u16::MAX);
        inter.addi(1);
        assert_eq!(inter.get_i(), 9);
    }

    // tests for addb()
    #[test]
    fn test_addb() {
       let mut inter = get_inter();
       inter.addb(1, 8);
       assert_eq!(inter.get_reg(1), 8);
    }

    #[test]
    fn test_addb_overflow() {
        let mut inter = get_inter();
        inter.addb(2, 255);
        inter.addb(2, 1);
        assert_eq!(inter.get_reg(2), 0);
    }

    // tests for add()
    #[test]
    fn test_add() {
        let mut inter = get_inter();
        inter.set_reg(2, 10);
        inter.add(1, 2);
        assert_eq!(inter.get_reg(1), 10);
        assert_eq!(inter.get_reg(15), 0);
    }

    #[test]
    fn test_add_overflow() {
        let mut inter = get_inter();
        inter.set_reg(1, 255);
        inter.set_reg(2, 2);
        inter.add(1, 2);
        assert_eq!(inter.get_reg(1), 1);
        assert_eq!(inter.get_reg(15), 1);
    }
}
