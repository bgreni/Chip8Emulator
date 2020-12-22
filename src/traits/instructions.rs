use crate::interpreter::Interpreter;
use crate::drivers::input::Inputs;
use crate::drivers::screen::Screen;
use rand::Rng;
use std::io;
use std::io::stdin;
use std::io::Read;
use piston::input::Button;


pub trait Helpers {
    fn get_reg(&self, index: usize) -> u8;
    fn set_reg(&mut self, index: usize, val: u8);
    fn set_vf(&mut self);
    fn unset_vf(&mut self);
    fn set_i(&mut self, val: u16);
    fn get_i(&self) -> u16;
    fn handle_vf(&mut self, condition: bool);
    fn inc_pc(&mut self);
    fn get_pc(&self) -> u16;
    fn get_dt(&self) -> u8;
    fn set_dt(&mut self, val: u8);
    fn get_st(&self) -> u8;
    fn set_st(&mut self, val: u8);
    fn write_mem(&mut self, loc: usize, val: u8);
    fn read_mem(&mut self, loc: usize) -> u8;
    fn to_digits(&mut self, num: usize) -> Vec<usize>;
}

impl Helpers for Interpreter {
    fn get_reg(&self, index: usize) -> u8 {
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

    fn get_i(&self) -> u16 {
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

    fn get_pc(&self) -> u16 {
        return self.registers.pc;
    }

    fn get_dt(&self) -> u8 {
        return self.registers.dt;
    }

    fn set_dt(&mut self, val: u8) {
        self.registers.dt = val;
    }

    fn get_st(&self) -> u8 {
        return self.registers.st;
    }

    fn set_st(&mut self, val: u8) {
        self.registers.st = val;
    }

    fn write_mem(&mut self, loc: usize, val: u8) {
        self.memory.memory[loc] = val;
    }

    fn read_mem(&mut self, loc: usize) -> u8 {
        return self.memory.memory[loc];
    }

    fn to_digits(&mut self, mut num: usize) -> Vec<usize> {
        let mut vec = vec![0usize; 3];
        // let digs = num.to_string()
        //     .chars()
        //     .map(|d| d.to_digit(10).unwrap() as usize)
        //     .collect::<Vec<usize>>();
        // for i in digs.len() - 1..=0 {
        //     vec[i] = digs[i].clone();
        // }
        while num < 0 {
            let piece = num % 10;
            vec.push(piece);
            num /= 10;
        }
        vec.reverse();
        return vec;
    }
}


pub trait Instructions {
    fn cls(&mut self, screen: &mut Screen);                 // 00E0
    fn ret(&mut self);                                      // 00EE
    fn jp(&mut self, address: u16);                         // 1nnn
    fn jp_add(&mut self, address: u16);                     // Bnnn
    fn call(&mut self, address: u16);                       // 2nnn
    fn seb(&mut self, regx: usize, byte: u8);               // 3xkk
    fn sneb(&mut self, regx: usize, byte: u8);              // 4xkk
    fn sne(&mut self, regx: usize, regy: usize);            // 9xy0
    fn se(&mut self, regx: usize, regy: usize);             // 5xy0
    fn ldb(&mut self, regx: usize, byte: u8);               // 6xkk
    fn ld(&mut self, regx: usize, regy: usize);             // 8xy0
    fn ld_sprite(&mut self, regx: usize);                   // Fx29
    fn ld_bcd(&mut self, regx: usize);                      // Fx33
    fn copy_reg_mem(&mut self, limit_reg: usize);           // Fx55
    fn read_reg_mem(&mut self, limit_reg: usize);           // Fx65
    fn ldi(&mut self, val: u16);                            // Annn
    fn ldt(&mut self, regx: usize);                         // Fx07
    fn sdt(&mut self, regx: usize);                         // Fx15
    fn sst(&mut self, regx: usize);                         // Fx18
    fn ldk(&mut self, regx: usize);                         // Fx0A
    fn addb(&mut self, regx: usize, byte: u8);              // 7xkk
    fn add(&mut self, regx: usize, regy: usize);            // 8xy4
    fn addi(&mut self, regx: usize);                        // Fx1E
    fn sub(&mut self, regx: usize, regy: usize);            // 8xy5
    fn subn(&mut self, regx: usize, regy: usize);           // 8xy7
    fn or(&mut self, regx: usize, regy: usize);             // 8xy1
    fn and(&mut self, regx: usize, regy: usize);            // 8xy2
    fn xor(&mut self, regx: usize, regy: usize);            // 8xy3
    fn shr(&mut self, regx: usize);                         // 8xy6
    fn shl(&mut self, regx: usize);                         // 8xyE
    fn rnd(&mut self, regx: usize, byte: u8);               // Cxkk
    fn drw(&mut self, regx: usize, regy: usize, sprite_height: u8, screen: &mut Screen);// Dxyn
    fn skp(&mut self, regx: usize, input_driver: &Inputs, key: Option<Button>);  // Ex9E
    fn sknp(&mut self, regx: usize, input_driver: &Inputs, key: Option<Button>); // ExA1
}


impl Instructions for Interpreter {
    /// Add byte to register
    fn addb(&mut self, regx: usize, byte: u8) {
        let regx_val = self.get_reg(regx);
        self.set_reg(regx, regx_val.wrapping_add(byte));
        self.inc_pc();
    }

    /// Vx = Vx + Vy
    /// set vf if result is greater than 255 (max 8 bit int)
    fn add(&mut self, regx: usize, regy: usize) {
        let result: u16 = self.get_reg(regx) as u16 + self.get_reg(regy) as u16;
        self.handle_vf(result > u8::MAX as u16);
        self.set_reg(regx, result as u8);
        self.inc_pc();
    }

    /// I = Vx + I
    fn addi(&mut self, regx: usize) {
        let i_val = self.get_i();
        let regx_val = self.get_reg(regx) as u16;
        self.set_i(i_val.wrapping_add(regx_val));
        self.inc_pc();
    }

    /// Vx = Vx - Vy
    /// set Vf if Vx > Vy
    fn sub(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.handle_vf(regx_val > regy_val);
        self.set_reg(regx, regx_val.wrapping_sub(regy_val));
        self.inc_pc();
    }

    /// Vx = Vy - Vx
    /// set Vf if Vy > Vx
    fn subn(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.handle_vf(regy_val > regx_val);
        self.set_reg(regx, regy_val.wrapping_sub(regx_val));
        self.inc_pc();
    }

    /// Vx = Vx or Vy (bitwise)
    fn or(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val | regy_val);
        self.inc_pc();
    }

    /// Vx = Vx and Vy (bitwise)
    fn and(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val & regy_val);
        self.inc_pc();
    }

    /// Vx = Vx xor Vy (bitwise)
    fn xor(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regx_val ^ regy_val);
        self.inc_pc();
    }

    /// Vx = Vx / 2
    /// sets Vf if least significant bit of Vx is 1
    fn shr(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.handle_vf(regx_val & 0x1 == 1);
        self.set_reg(regx, regx_val >> 1);
        self.inc_pc();
    }

    /// Vx = Vx * 2
    /// sets Vf if most significant bit of Vx is 1
    fn shl(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.handle_vf(regx_val & 0x80 == 0x80);
        self.set_reg(regx, regx_val << 1);
        self.inc_pc();
    }

    /// inc pc if Vx == byte
    fn seb(&mut self, regx: usize, byte: u8) {
        if self.get_reg(regx) == byte {
            self.inc_pc();
        }
        self.inc_pc();
    }

    /// inc pc if Vx != byte
    fn sneb(&mut self, regx: usize, byte: u8) {
        if self.get_reg(regx) != byte {
            self.inc_pc();
        }
        self.inc_pc();
    }

    /// inc pc if Vx != Vy
    fn sne(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        if regx_val != regy_val {
            self.inc_pc();
        }
        self.inc_pc();
    }

    /// inc pc if Vx == Vy
    fn se(&mut self, regx: usize, regy: usize) {
        let regx_val = self.get_reg(regx);
        let regy_val = self.get_reg(regy);
        if regx_val == regy_val {
            self.inc_pc();
        }
        self.inc_pc();
    }

    /// set Vx = byte
    fn ldb(&mut self, regx: usize, byte: u8) {
        self.set_reg(regx, byte);
        self.inc_pc();
    }

    /// set Vx = Vy
    fn ld(&mut self, regx: usize, regy: usize) {
        let regy_val = self.get_reg(regy);
        self.set_reg(regx, regy_val);
        self.inc_pc();
    }

    /// I = val
    fn ldi(&mut self, val: u16) {
        self.set_i(val);
        self.inc_pc();
    }

    /// Vx = DT
    fn ldt(&mut self, regx: usize) {
        let dt_val = self.get_dt();
        self.set_reg(regx, dt_val);
        self.inc_pc();
    }

    /// DT = Vx
    fn sdt(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.set_dt(regx_val);
        self.inc_pc();
    }

    /// ST = Vx
    fn sst(&mut self, regx: usize) {
        let regx_val = self.get_reg(regx);
        self.set_st(regx_val);
        self.inc_pc();
    }

    /// gen rand number [0...255]
    /// Vx = rand & byte
    fn rnd(&mut self, regx: usize, byte: u8) {
        self.set_reg(regx, rand::thread_rng().gen::<u8>() & byte);
        self.inc_pc();
    }

    /// Wait for keypress and store value of key in Vx
    fn ldk(&mut self, regx: usize) {
        println!("waiting for key");
        self.wait_for_key = true;
        self.key_reg = regx;
        self.inc_pc();
    }

    /// Inc pc if key value held in Vx is pressed
    fn skp(&mut self, regx: usize, input_driver: &Inputs, key: Option<Button>) {
        let key = input_driver.check_key_presses(key);
        let target_key = self.get_reg(regx);
        if key != None {
            if target_key == key.unwrap() {
                self.inc_pc();
            }
        }
        self.inc_pc();
    }

    /// Inc pc if key value held in Vx is not pressed
    fn sknp(&mut self, regx: usize, input_driver: &Inputs, key: Option<Button>) {
        let key = input_driver.check_key_presses(key);
        let target_key = self.get_reg(regx);
        if key != None {
            if target_key == key.unwrap() {
                self.inc_pc();
            }
        }
        self.inc_pc();
    }

    /// Set PC = address
    fn jp(&mut self, address: u16) {
        self.registers.pc = address;
    }

    /// Set PC = address + V0
    fn jp_add(&mut self, address: u16) {
        self.registers.pc = address + self.get_reg(0) as u16;
    }

    /// Set PC to address on top of stack
    /// decrement stack pointer
    fn ret(&mut self) {
        let ret_add = self.pop_stack();
        self.registers.pc = ret_add;
    }

    /// Put current PC on top of stack
    /// Set PC to new address
    fn call(&mut self, address: u16) {
        self.push_stack(self.registers.pc);
        self.registers.pc = address;
    }

    /// clear the screen
    fn cls(&mut self, screen: &mut Screen) {
        screen.clear_screen();
        self.inc_pc();
    }

    /// copy values of V0..V[limit_reg] into memory 
    /// locations starting at the value in I
    fn copy_reg_mem(&mut self, limit_reg: usize){
        let mem_start = self.get_i() as usize;
        for i in 0..limit_reg + 1 {
            let reg = self.get_reg(i);
            self.write_mem((mem_start + i) as usize, reg);
        }
        self.inc_pc();
    }

    /// read values into V0..V[limit_reg] from
    /// mem locations starting at value of I
    fn read_reg_mem(&mut self, limit_reg: usize) {
        let mem_start = self.get_i() as usize;
        for i in 0..limit_reg + 1 {
            let val = self.read_mem((mem_start + i) as usize);
            self.set_reg(i, val);
        }
        self.inc_pc();
    }

    /// Load the location of a sprite into I
    fn ld_sprite(&mut self, regx: usize) {
        let loc = self.get_reg(regx) as u16;
        self.set_i(loc);
        self.inc_pc();
    }

    /// Load BCD repr of Vx into memory locations
    /// I, I+1, I+2
    fn ld_bcd(&mut self, regx: usize) {
        let val = self.get_reg(regx) as u8;
        let digits = self.to_digits(val as usize);
        println!("{:?}", digits);
        let mem_loc = self.get_i();

        for i in 0..=2 {
            self.write_mem((mem_loc + i) as usize, digits[i as usize] as u8)
        }
        self.inc_pc();
    }

    fn drw(&mut self, regx: usize, regy: usize, sprite_height: u8, screen: &mut Screen) {
        self.unset_vf();
        let start = self.get_i() as usize;
        let mut sprite = Vec::new();

        for i in 0..sprite_height {
            sprite.push(self.read_mem(start + 1) as u8);
        }

        let x = self.get_reg(regx) as usize;
        let y = self.get_reg(regy) as usize;

        let coll = screen.draw_sprite(x, y, sprite);
        if coll == 1 {
            self.set_vf();
        }
        self.inc_pc();
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    fn get_inter() -> Interpreter {
        return Interpreter::new();
    }

    // tests for ld_bcd
    #[test]
    fn test_ld_bcd() {
        let mut inter = get_inter();
        inter.set_i(0);
        inter.set_reg(1, 123);
        inter.ld_bcd(1);
        for i in 0..3 {
            assert_eq!(inter.read_mem(i as usize), i + 1);
        }
    }

    // tests for ld_sprite
    #[test]
    fn test_ld_sprite() {
        let mut inter = get_inter();
        inter.set_reg(1, 8);
        inter.ld_sprite(1);
        assert_eq!(inter.get_i(), 8);
    }

    // tests for read_reg_mem
    #[test]
    fn test_read_reg_mem() {
        let mut inter = get_inter();
        for i in 10..20 {
            inter.write_mem(i, i as u8);
        }

        inter.set_i(10);
        inter.read_reg_mem(9);

        for i in 0..10 {
            assert_eq!(inter.get_reg(i), (i + 10) as u8);
        }
    }

    // tests for copy_reg_mem
    #[test]
    fn test_copy_reg_mem() {
        let mut inter = get_inter();
        inter.set_i(10);
        for i in 0..10 {
            inter.set_reg(i, (i+1) as u8);
        }
        inter.copy_reg_mem(9);

        for i in 0..10 {
            assert_eq!(inter.read_mem(i + 10), (i + 1) as u8);
        }
    }

    // tests for call
    #[test]
    fn test_call() {
        let mut inter = get_inter();
        inter.registers.pc = 10;
        inter.call(107);
        assert_eq!(inter.pop_stack(), 10);
        assert_eq!(inter.registers.pc, 107);
    }

    // tests for ret
    #[test]
    fn test_ret() {
        let mut inter = get_inter();
        inter.push_stack(1010);
        inter.ret();
        assert_eq!(inter.registers.pc, 1010);
        assert_eq!(inter.memory.stack.is_empty(), true);
    }

    // tests for jp_add
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
