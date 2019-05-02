use crate::opcode::{DataRegister, Opcode};
use crate::vm::memory::MEMORY_LENGTH;
use crate::vm::{Vm, HEIGHT, WIDTH};

pub fn execute(vm: &mut Vm, opcode: Opcode) {
    let mut new_pc = vm.pc + 2;

    // TODO: Should this happen while we're waiting for a key press,
    if vm.st > 0 {
        vm.st -= 1;
    }
    if vm.dt > 0 {
        vm.dt -= 1;
    }

    match opcode {
        Opcode::DisplayClear => vm.frame_buffer.clear(),

        Opcode::JP(address) => new_pc = address,

        Opcode::JPB(nnn) => new_pc = vm.registers.read(DataRegister::V0) as u16 + nnn,

        Opcode::CALL(address) => {
            vm.stack.push(wrap_pc(vm.pc + 2)).expect("stack overflow");
            new_pc = address;
        }

        Opcode::RET => new_pc = vm.stack.pop().expect("stack underflow"),

        Opcode::SYS(_address) => { /* NO OP */ }

        Opcode::SKP(x) => {
            if let Some(key) = vm.key {
                if key as u8 == vm.registers.read(x) {
                    new_pc += 2;
                }
            }
        }

        Opcode::SKNP(x) => {
            if let Some(key) = vm.key {
                if key as u8 != vm.registers.read(x) {
                    new_pc += 2;
                }
            }
        }

        Opcode::LD6(register, value) => vm.registers.write(register, value),

        Opcode::LdB(x) => {
            let x = vm.registers.read(x);
            vm.memory.write(vm.registers.i, x / 100 % 10);
            vm.memory.write(vm.registers.i + 1, x / 10 % 10);
            vm.memory.write(vm.registers.i + 2, x % 10);
        }

        Opcode::LDI(value) => vm.registers.i = value,

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

        Opcode::LdF(x) => vm.registers.i = vm.registers.read(x).into(),

        Opcode::LdKey(x) => match vm.key {
            Some(key) => {
                vm.registers.write(x, key as u8);
            }
            None => new_pc = vm.pc,
        },

        Opcode::LdSt(x) => vm.st = vm.registers.read(x),

        Opcode::LdDt(x) => vm.dt = vm.registers.read(x),

        Opcode::LdDtToReg(x) => vm.registers.write(x, vm.dt),

        Opcode::LD8(x, y) => {
            let y = vm.registers.read(y);
            vm.registers.write(x, y);
        }

        Opcode::SE3(register, value) => {
            if vm.registers.read(register) == value {
                new_pc += 2
            }
        }

        Opcode::SNE4(register, value) => {
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

        Opcode::DRW(x_register, y_register, height) => {
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

        Opcode::ADD(x, kk) => {
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

        Opcode::RND(x, kk) => {
            let rand = vm.rand();
            vm.registers.write(x, rand & kk);
        }
    }

    vm.pc = wrap_pc(new_pc);
}

fn wrap_pc(pc: u16) -> u16 {
    pc & (MEMORY_LENGTH as u16 - 1)
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
}
