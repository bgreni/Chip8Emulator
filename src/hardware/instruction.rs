

pub const OPCODE_LEN: u16 = 2;

pub struct Instruction {
    opcode: u16
}


impl Instruction {
    pub fn new(code: u16) -> Self {
        return Instruction {
            opcode: code
        }
    }

    pub fn get_addr(&self) -> u16 {
        return self.opcode & 0x0FFF;
    }

    pub fn get_nibble(&self) -> u8 {
        return (self.opcode & 0x000F) as u8;
    }

    pub fn get_x(&self) -> u8 {
        return ((self.opcode & 0x0F00) >> 8) as u8;
    }

    pub fn get_y(&self) -> u8 {
        return ((self.opcode & 0x00F0) >> 4) as u8;
    }

    pub fn get_kk(&self) -> u8 {
        return (self.opcode & 0x00FF) as u8;
    }

    pub fn get_top_nib(&self) -> u8 {
        return ((self.opcode & 0xF000) >> 12) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_get_nibble() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_nibble(), 9 as u8);
    }

    #[test]
    fn test_get_addr() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_addr(), 0x0739 as u16);
    }

    #[test]
    fn test_get_x() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_x(), 7 as u8);
    }

    #[test]
    fn test_get_y() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_y(), 3 as u8);
    }

    #[test]
    fn test_get_kk() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_kk(), 0x39 as u8);
    }

    #[test]
    fn test_get_top_nib() {
        let instruction = Instruction::new(0x4739 as u16);
        assert_eq!(instruction.get_top_nib(), 4 as u8);
    }
}