mod cpu;
mod frame_buffer;
mod key;
mod memory;
mod registers;
mod stack;

use crate::opcode::Opcode;
use crate::vm::frame_buffer::FrameBuffer;
pub use crate::vm::key::Key;
use crate::vm::memory::Memory;
use crate::vm::registers::Registers;
use crate::vm::stack::Stack;
use rand::prelude::*;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Vm {
    pub pc: u16,
    stack: Stack,
    memory: Memory,
    registers: Registers,
    st: u8,
    dt: u8,
    pub frame_buffer: FrameBuffer,
    pub key: Option<Key>,
    rng: ThreadRng,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            pc: 0x200,
            stack: Stack::new(),
            memory: Memory::new(),
            registers: Registers::new(),
            st: 0,
            dt: 0,
            frame_buffer: FrameBuffer::new(WIDTH, HEIGHT),
            key: None,
            rng: rand::thread_rng(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        rom.iter().enumerate().for_each(|(address, value)| {
            self.memory.write(address as u16 + 0x200, *value);
        });
    }

    pub fn fetch(&self) -> u16 {
        (self.memory.read(self.pc) as u16) << 8 | self.memory.read(self.pc + 1) as u16
    }

    pub fn execute(&mut self, opcode: Opcode) {
        cpu::execute(self, opcode)
    }

    pub fn rand(&mut self) -> u8 {
        self.rng.gen()
    }
}
