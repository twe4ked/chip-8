#[derive(Debug)]
pub enum Opcode {
    DisplayClear,
    RET,
    SKP(DataRegister),
    SKNP(DataRegister),
    CALL(Nnn),
    SYS(Nnn),
    LDI(Nnn),
    LD6(DataRegister, Kk),
    LD8(DataRegister, DataRegister),
    LdDtToReg(DataRegister),
    LdDt(DataRegister),
    LdSt(DataRegister),
    LdB(DataRegister),
    LdF(DataRegister),
    LdAll(DataRegister),
    LdAllI(DataRegister),
    LdKey(DataRegister),
    OR8(DataRegister, DataRegister),
    AND8(DataRegister, DataRegister),
    XOR8(DataRegister, DataRegister),
    ADD8(DataRegister, DataRegister),
    ADD(DataRegister, Kk),
    AddI(DataRegister),
    SUB8(DataRegister, DataRegister),
    SHR8(DataRegister, DataRegister),
    SUBN8(DataRegister, DataRegister),
    SHL8(DataRegister, DataRegister),
    SNE4(DataRegister, Kk),
    SE5(DataRegister, DataRegister),
    SNE(DataRegister, DataRegister),
    SE3(DataRegister, Kk),
    JP(Nnn),
    JPB(Nnn),
    DRW(DataRegister, DataRegister, N),
    RND(DataRegister, Kk),
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataRegister {
    V0, V1, V2, V3, V4, V5, V6, V7,
    V8, V9, VA, VB, VC, VD, VE, VF,
}

#[rustfmt::skip]
impl DataRegister {
    pub fn from(n: u8) -> Self {
        match n {
            0x0 => DataRegister::V0, 0x1 => DataRegister::V1, 0x2 => DataRegister::V2, 0x3 => DataRegister::V3,
            0x4 => DataRegister::V4, 0x5 => DataRegister::V5, 0x6 => DataRegister::V6, 0x7 => DataRegister::V7,
            0x8 => DataRegister::V8, 0x9 => DataRegister::V9, 0xa => DataRegister::VA, 0xb => DataRegister::VB,
            0xc => DataRegister::VC, 0xd => DataRegister::VD, 0xe => DataRegister::VE, 0xf => DataRegister::VF,
            _ => panic!("invalid data register: {}", n),
        }
    }
}

#[derive(Debug)]
pub struct Nnn(pub u16);

#[derive(Debug)]
pub struct Kk(pub u8);

#[derive(Debug)]
pub struct N(pub u8);

fn decode_parts(instruction: u16) -> (Nnn, Kk, DataRegister, DataRegister, N) {
    (
        Nnn(instruction & 0x0fff),
        Kk((instruction & 0x00ff) as u8),
        DataRegister::from(((instruction >> 8) & 0xf) as u8),
        DataRegister::from(((instruction >> 4) & 0xf) as u8),
        N((instruction & 0x000f) as u8),
    )
}

pub fn decode(instruction: u16) -> Opcode {
    let (nnn, kk, x, y, n) = decode_parts(instruction);

    match (instruction >> 12) & 0xf {
        0x0 => match instruction {
            0x00e0 => Opcode::DisplayClear,
            0x00ee => Opcode::RET,
            _ => Opcode::SYS(nnn),
        },
        0x1 => Opcode::JP(nnn),
        0x2 => Opcode::CALL(nnn),
        0x3 => Opcode::SE3(x, kk),
        0x4 => Opcode::SNE4(x, kk),
        0x5 => Opcode::SE5(x, y),
        0x6 => Opcode::LD6(x, kk),
        0x7 => Opcode::ADD(x, kk),
        0x8 => match n.0 {
            0x0 => Opcode::LD8(x, y),
            0x1 => Opcode::OR8(x, y),
            0x2 => Opcode::AND8(x, y),
            0x3 => Opcode::XOR8(x, y),
            0x4 => Opcode::ADD8(x, y),
            0x5 => Opcode::SUB8(x, y),
            0x6 => Opcode::SHR8(x, y),
            0x7 => Opcode::SUBN8(x, y),
            0xE => Opcode::SHL8(x, y),
            _ => invalid_instruction(instruction),
        },
        0x9 => Opcode::SNE(x, y),
        0xa => Opcode::LDI(nnn),
        0xb => Opcode::JPB(nnn),
        0xc => Opcode::RND(x, kk),
        0xd => Opcode::DRW(x, y, n),
        0xe => match kk.0 {
            0x9e => Opcode::SKP(x),
            0xa1 => Opcode::SKNP(x),
            _ => invalid_instruction(instruction),
        },
        0xf => match kk.0 {
            0x07 => Opcode::LdDtToReg(x),
            0x0a => Opcode::LdKey(x),
            0x15 => Opcode::LdDt(x),
            0x18 => Opcode::LdSt(x),
            0x1e => Opcode::AddI(x),
            0x29 => Opcode::LdF(x),
            0x33 => Opcode::LdB(x),
            0x55 => Opcode::LdAllI(x),
            0x65 => Opcode::LdAll(x),
            _ => invalid_instruction(instruction),
        },
        _ => invalid_instruction(instruction),
    }
}

fn invalid_instruction(instruction: u16) -> ! {
    panic!("invalid instruction: {:#06x}", instruction);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_parts() {
        let (nnn, kk, x, y, n) = decode_parts(0b1111_1110_1100_1000);

        assert_eq!(nnn.0, 0b1110_1100_1000);
        assert_eq!(kk.0, 0b1100_1000);
        assert_eq!(x, DataRegister::VE); // 14 (0b1110)
        assert_eq!(y, DataRegister::VC); // 12 (0b1100)
        assert_eq!(n.0, 0b1000);

        let x = 456;
        let ones = x % 10;
        let tens = x / 10 % 10;
        let hundreds = x / 100 % 10;

        assert_eq!(hundreds, 4);
        assert_eq!(tens, 5);
        assert_eq!(ones, 6);
    }
}
