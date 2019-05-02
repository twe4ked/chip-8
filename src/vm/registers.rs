use crate::opcode::DataRegister;

pub struct Registers {
    registers: [u8; 16],
    pub i: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            i: 0,
        }
    }

    pub fn read(&self, register: DataRegister) -> u8 {
        self.registers[register as usize]
    }

    pub fn write(&mut self, register: DataRegister, value: u8) {
        self.registers[register as usize] = value
    }
}
