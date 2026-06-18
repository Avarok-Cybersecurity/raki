pub mod bit_32 {
    use super::super::{only_rv64, DecodeUtil, DecodingError};
    use crate::instruction::b_extension::BOpcode;
    use crate::Isa;

    /// Decode a register-register B-extension instruction. `parse_extension`
    /// only routes recognised (opcode, funct3, funct7) tuples here (see
    /// [`is_b_ext`]); an unrecognised one is a loud error (safety net —
    /// never a silent mis-decode).
    pub fn parse_opcode(inst: u32, isa: Isa) -> Result<BOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let funct7: u8 = u8::try_from(inst.slice(31, 25)).unwrap();

        match opmap {
            0b011_0011 => match (funct7, funct3) {
                (0b010_0000, 0b100) => Ok(BOpcode::XNOR),
                (0b010_0000, 0b110) => Ok(BOpcode::ORN),
                (0b010_0000, 0b111) => Ok(BOpcode::ANDN),
                (0b000_0101, 0b100) => Ok(BOpcode::MIN),
                (0b000_0101, 0b101) => Ok(BOpcode::MINU),
                (0b000_0101, 0b110) => Ok(BOpcode::MAX),
                (0b000_0101, 0b111) => Ok(BOpcode::MAXU),
                (0b011_0000, 0b001) => Ok(BOpcode::ROL),
                (0b011_0000, 0b101) => Ok(BOpcode::ROR),
                (0b001_0000, 0b010) => Ok(BOpcode::SH1ADD),
                (0b001_0000, 0b100) => Ok(BOpcode::SH2ADD),
                (0b001_0000, 0b110) => Ok(BOpcode::SH3ADD),
                (0b010_0100, 0b001) => Ok(BOpcode::BCLR),
                (0b010_0100, 0b101) => Ok(BOpcode::BEXT),
                (0b011_0100, 0b001) => Ok(BOpcode::BINV),
                (0b001_0100, 0b001) => Ok(BOpcode::BSET),
                (0b000_0111, 0b101) => Ok(BOpcode::CZERO_EQZ),
                (0b000_0111, 0b111) => Ok(BOpcode::CZERO_NEZ),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b011_1011 => match (funct7, funct3) {
                (0b011_0000, 0b001) => only_rv64(BOpcode::ROLW, isa),
                (0b011_0000, 0b101) => only_rv64(BOpcode::RORW, isa),
                (0b000_0100, 0b000) => only_rv64(BOpcode::ADD_UW, isa),
                (0b001_0000, 0b010) => only_rv64(BOpcode::SH1ADD_UW, isa),
                (0b001_0000, 0b100) => only_rv64(BOpcode::SH2ADD_UW, isa),
                (0b001_0000, 0b110) => only_rv64(BOpcode::SH3ADD_UW, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// True iff `(opmap, funct3, funct7)` is a modelled B-extension
    /// register-register encoding — used by `parse_extension` to route to
    /// `Extensions::B` BEFORE the (now strict) base-I decoder. Must stay
    /// in exact sync with [`parse_opcode`]'s `Ok` arms.
    pub fn is_b_ext(opmap: u8, funct3: u8, funct7: u8) -> bool {
        match opmap {
            0b011_0011 => matches!(
                (funct7, funct3),
                (0b010_0000, 0b100 | 0b110 | 0b111)
                    | (0b000_0101, 0b100..=0b111)
                    | (0b011_0000, 0b001 | 0b101)
                    | (0b001_0000, 0b010 | 0b100 | 0b110)
                    | (0b010_0100, 0b001 | 0b101)
                    | (0b011_0100, 0b001)
                    | (0b001_0100, 0b001)
                    | (0b000_0111, 0b101 | 0b111)
            ),
            0b011_1011 => matches!(
                (funct7, funct3),
                (0b011_0000, 0b001 | 0b101)
                    | (0b000_0100, 0b000)
                    | (0b001_0000, 0b010 | 0b100 | 0b110)
            ),
            _ => false,
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, _opkind: &BOpcode) -> Option<usize> {
        Some(inst.slice(11, 7) as usize)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, _opkind: &BOpcode) -> Option<usize> {
        Some(inst.slice(19, 15) as usize)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs2(inst: u32, _opkind: &BOpcode) -> Option<usize> {
        Some(inst.slice(24, 20) as usize)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_imm(_inst: u32, _opkind: &BOpcode, _isa: Isa) -> Option<i32> {
        None
    }
}
