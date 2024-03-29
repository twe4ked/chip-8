use crate::opcode::{DataRegister, Kk, Nnn, Opcode, N};
use crate::vm::memory::MEMORY_LENGTH;
use crate::vm::{Vm, HEIGHT, WIDTH};

pub fn execute(vm: &mut Vm, opcode: Opcode) {
    let mut new_pc = vm.pc + 2;

    vm.update_timers();

    match opcode {
        Opcode::DisplayClear => vm.frame_buffer.clear(),

        Opcode::JP(Nnn(address)) => new_pc = address,

        Opcode::JPB(Nnn(nnn)) => new_pc = vm.registers.read(DataRegister::V0) as u16 + nnn,

        Opcode::CALL(Nnn(address)) => {
            vm.stack.push(wrap_pc(vm.pc + 2)).expect("stack overflow");
            new_pc = address;
        }

        Opcode::RET => new_pc = vm.stack.pop().expect("stack underflow"),

        Opcode::SYS(_address) => { /* NO OP */ }

        Opcode::SKP(x) => {
            if let Some(key) = vm.try_key() {
                if key as u8 == vm.registers.read(x) {
                    new_pc += 2;
                }
            }
        }

        Opcode::SKNP(x) => {
            if let Some(key) = vm.try_key() {
                if key as u8 != vm.registers.read(x) {
                    new_pc += 2;
                }
            }
        }

        Opcode::LD6(register, Kk(value)) => vm.registers.write(register, value),

        Opcode::LdB(x) => {
            let parts = Parts::from(vm.registers.read(x));
            vm.memory.write(vm.registers.i, parts.hundreds);
            vm.memory.write(vm.registers.i + 1, parts.tens);
            vm.memory.write(vm.registers.i + 2, parts.ones);
        }

        Opcode::LDI(Nnn(value)) => vm.registers.i = value,

        Opcode::LdAllI(x) => {
            for r in 0..=x as u8 {
                let value = vm.registers.read(DataRegister::from(r));
                vm.memory.write(vm.registers.i + r as u16, value);
            }
        }

        Opcode::LdAll(x) => {
            for r in 0..=x as u8 {
                let value = vm.memory.read(vm.registers.i + r as u16);
                vm.registers.write(DataRegister::from(r), value);
            }
        }

        Opcode::LdF(x) => vm.registers.i = vm.registers.read(x) as u16 * 5,

        Opcode::LdKey(x) => loop {
            if let Some(key) = vm.try_key() {
                vm.registers.write(x, key as u8);
                break;
            }
        },

        Opcode::LdSt(x) => vm.st = vm.registers.read(x),

        Opcode::LdDt(x) => vm.dt = vm.registers.read(x),

        Opcode::LdDtToReg(x) => vm.registers.write(x, vm.dt),

        Opcode::LD8(x, y) => {
            let y = vm.registers.read(y);
            vm.registers.write(x, y);
        }

        Opcode::SE3(register, Kk(value)) => {
            if vm.registers.read(register) == value {
                new_pc += 2
            }
        }

        Opcode::SNE4(register, Kk(value)) => {
            if vm.registers.read(register) != value {
                new_pc += 2
            }
        }

        Opcode::SE5(x, y) => {
            if vm.registers.read(x) == vm.registers.read(y) {
                new_pc += 2
            }
        }

        Opcode::SNE(x, y) => {
            if vm.registers.read(x) != vm.registers.read(y) {
                new_pc += 2
            }
        }

        Opcode::DRW(x_register, y_register, N(height)) => {
            let x_offset = vm.registers.read(x_register);
            let y_offset = vm.registers.read(y_register);

            let mut vf = 0;

            for y in 0..height {
                let line = vm.memory.read(vm.registers.i + y as u16);
                for x in 0..8 {
                    if (line & (0x80 >> x)) != 0 {
                        let mut x = x_offset as usize + x as usize;
                        if x >= WIDTH {
                            x = 0;
                        }
                        let mut y = y_offset as usize + y as usize;
                        if y >= HEIGHT {
                            y = 0;
                        }

                        let collision = vm.frame_buffer.toggle_pixel(x, y);
                        if collision {
                            vf = 1;
                        }
                    }
                }
            }

            vm.registers.write(DataRegister::VF, vf);
        }

        Opcode::ADD(x, Kk(kk)) => {
            let result = vm.registers.read(x).wrapping_add(kk);
            vm.registers.write(x, result);
        }

        Opcode::ADD8(x, y) => {
            let (result, overflow) = vm.registers.read(x).overflowing_add(vm.registers.read(y));
            vm.registers.write(x, result);
            vm.registers
                .write(DataRegister::VF, if overflow { 1 } else { 0 });
        }

        Opcode::AddI(x) => {
            vm.registers.i = vm.registers.i.wrapping_add(vm.registers.read(x) as u16);
        }

        Opcode::SUB8(x, y) => {
            let (result, overflow) = vm.registers.read(x).overflowing_sub(vm.registers.read(y));
            vm.registers.write(x, result);
            vm.registers
                .write(DataRegister::VF, if overflow { 0 } else { 1 });
        }

        Opcode::SUBN8(x, y) => {
            let (result, overflow) = vm.registers.read(y).overflowing_sub(vm.registers.read(x));
            vm.registers.write(x, result);
            vm.registers
                .write(DataRegister::VF, if overflow { 0 } else { 1 });
        }

        Opcode::AND8(x, y) => {
            let result = vm.registers.read(x) & vm.registers.read(y);
            vm.registers.write(x, result);
        }

        Opcode::XOR8(x, y) => {
            let result = vm.registers.read(x) ^ vm.registers.read(y);
            vm.registers.write(x, result);
        }

        Opcode::OR8(x, y) => {
            let result = vm.registers.read(x) | vm.registers.read(y);
            vm.registers.write(x, result);
        }

        Opcode::SHR8(x, _y) => {
            let value = vm.registers.read(x);
            vm.registers.write(DataRegister::VF, value & 1);
            vm.registers.write(x, value.wrapping_div(2));
        }

        Opcode::SHL8(x, _y) => {
            let value = vm.registers.read(x);
            vm.registers.write(DataRegister::VF, value >> 7 & 1);
            vm.registers.write(x, value.wrapping_mul(2));
        }

        Opcode::RND(x, Kk(kk)) => {
            let rand = vm.rand();
            vm.registers.write(x, rand & kk);
        }
    }

    vm.pc = wrap_pc(new_pc);
}

fn wrap_pc(pc: u16) -> u16 {
    pc & (MEMORY_LENGTH as u16 - 1)
}

struct Parts {
    ones: u8,
    tens: u8,
    hundreds: u8,
}

impl Parts {
    fn from(x: u8) -> Self {
        Self {
            ones: (x % 10),
            tens: (x / 10 % 10),
            hundreds: (x / 100 % 10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::memory::Memory;

    #[test]
    fn test_wrap_pc() {
        let memory_length = MEMORY_LENGTH as u16;

        assert_eq!(wrap_pc(0), 0);
        assert_eq!(wrap_pc(42), 42);
        assert_eq!(wrap_pc(memory_length), memory_length - 1);
        assert_eq!(wrap_pc(memory_length + 1), 0);
        assert_eq!(wrap_pc(memory_length + 2), 0);

        let memory = Memory::new();

        // No panics
        memory.read(wrap_pc(0));
        memory.read(wrap_pc(42));
        memory.read(wrap_pc(memory_length));
        memory.read(wrap_pc(memory_length + 1));
        memory.read(wrap_pc(memory_length + 2));
    }

    #[test]
    fn test_parts() {
        let parts = Parts::from(123);
        assert_eq!(parts.hundreds, 1);
        assert_eq!(parts.tens, 2);
        assert_eq!(parts.ones, 3);
    }
}
