pub mod bit_32 {
    use super::super::{only_rv64, DecodeUtil, DecodingError};
    use crate::instruction::b_extension::BOpcode;
    use crate::Isa;

    /// Decode a B-extension instruction. `parse_extension` only routes
    /// recognised encodings here (see [`is_b_ext`] for the register-register
    /// forms and [`is_b_imm`] for the immediate/unary forms); an
    /// unrecognised one is a loud error (safety net — never a silent
    /// mis-decode).
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
                // Zbc carry-less multiply.
                (0b000_0101, 0b001) => Ok(BOpcode::CLMUL),
                (0b000_0101, 0b011) => Ok(BOpcode::CLMULH),
                (0b000_0101, 0b010) => Ok(BOpcode::CLMULR),
                // zext.h (rv32): `pack rd, rs1, x0` with rs2 == 0.
                (0b000_0100, 0b100) if inst.slice(24, 20) == 0 => Ok(BOpcode::ZEXT_H),
                _ => Err(DecodingError::InvalidFunct7),
            },
            0b011_1011 => match (funct7, funct3) {
                (0b011_0000, 0b001) => only_rv64(BOpcode::ROLW, isa),
                (0b011_0000, 0b101) => only_rv64(BOpcode::RORW, isa),
                (0b000_0100, 0b000) => only_rv64(BOpcode::ADD_UW, isa),
                (0b001_0000, 0b010) => only_rv64(BOpcode::SH1ADD_UW, isa),
                (0b001_0000, 0b100) => only_rv64(BOpcode::SH2ADD_UW, isa),
                (0b001_0000, 0b110) => only_rv64(BOpcode::SH3ADD_UW, isa),
                // zext.h (rv64): `packw rd, rs1, x0` with rs2 == 0.
                (0b000_0100, 0b100) if inst.slice(24, 20) == 0 => only_rv64(BOpcode::ZEXT_H, isa),
                _ => Err(DecodingError::InvalidFunct7),
            },
            // OP-IMM / OP-IMM-32: immediate-shift and unary Zbb/Zbs/Zba.
            0b001_0011 | 0b001_1011 => {
                classify_imm(inst, isa).ok_or(DecodingError::InvalidFunct7)
            }
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// Classify an OP-IMM (`0b001_0011`) / OP-IMM-32 (`0b001_1011`)
    /// immediate-shift or unary B-extension encoding. Returns `None` for
    /// anything that is NOT a modelled B-ext form (so the caller routes it
    /// to base-I / fails loud). SSOT for both [`is_b_imm`] and
    /// [`parse_opcode`].
    fn classify_imm(inst: u32, isa: Isa) -> Option<BOpcode> {
        let opmap = inst.slice(6, 0);
        let funct3 = inst.slice(14, 12);
        let funct6 = inst.slice(31, 26);
        let funct7 = inst.slice(31, 25);
        let rs2 = inst.slice(24, 20);
        let imm12 = inst.slice(31, 20);
        let rv64 = matches!(isa, Isa::Rv64);
        match opmap {
            0b001_0011 => match funct3 {
                0b001 => match funct6 {
                    0b010010 => Some(BOpcode::BCLRI),
                    0b011010 => Some(BOpcode::BINVI),
                    0b001010 => Some(BOpcode::BSETI),
                    // Unary: funct7 == 0b0110000 (bit25 == 0), sub-op in rs2.
                    0b011000 if funct7 == 0b011_0000 => match rs2 {
                        0b00000 => Some(BOpcode::CLZ),
                        0b00001 => Some(BOpcode::CTZ),
                        0b00010 => Some(BOpcode::CPOP),
                        0b00100 => Some(BOpcode::SEXT_B),
                        0b00101 => Some(BOpcode::SEXT_H),
                        _ => None,
                    },
                    _ => None,
                },
                0b101 => match funct6 {
                    0b011000 => Some(BOpcode::RORI),
                    0b010010 => Some(BOpcode::BEXTI),
                    // orc.b: full imm12 == 0b0010100_00111.
                    0b001010 if imm12 == 0b0010100_00111 => Some(BOpcode::ORC_B),
                    // rev8 (rv64): imm12 == 0b0110101_11000.
                    0b011010 if rv64 && imm12 == 0b0110101_11000 => Some(BOpcode::REV8),
                    _ => None,
                },
                _ => None,
            },
            0b001_1011 if rv64 => match funct3 {
                0b001 => match funct6 {
                    0b000010 => Some(BOpcode::SLLI_UW),
                    0b011000 if funct7 == 0b011_0000 => match rs2 {
                        0b00000 => Some(BOpcode::CLZW),
                        0b00001 => Some(BOpcode::CTZW),
                        0b00010 => Some(BOpcode::CPOPW),
                        _ => None,
                    },
                    _ => None,
                },
                0b101 if funct7 == 0b011_0000 => Some(BOpcode::RORIW),
                _ => None,
            },
            _ => None,
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
                    | (0b000_0101, 0b001..=0b111)
                    | (0b011_0000, 0b001 | 0b101)
                    | (0b001_0000, 0b010 | 0b100 | 0b110)
                    | (0b010_0100, 0b001 | 0b101)
                    | (0b011_0100, 0b001)
                    | (0b001_0100, 0b001)
                    | (0b000_0111, 0b101 | 0b111)
                    | (0b000_0100, 0b100)
            ),
            0b011_1011 => matches!(
                (funct7, funct3),
                (0b011_0000, 0b001 | 0b101)
                    | (0b000_0100, 0b000 | 0b100)
                    | (0b001_0000, 0b010 | 0b100 | 0b110)
            ),
            _ => false,
        }
    }

    /// True iff `inst` is a modelled OP-IMM / OP-IMM-32 immediate-shift or
    /// unary B-extension encoding — routes to `Extensions::B` BEFORE base-I
    /// (which would otherwise fail loud on the reserved funct6/funct7).
    /// Uses the RV64 superset for routing; the strict per-ISA gate lives in
    /// [`parse_opcode`] (an RV32 decode of an RV64-only form then fails
    /// loud there, never mis-decodes).
    pub fn is_b_imm(inst: u32) -> bool {
        classify_imm(inst, Isa::Rv64).is_some()
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rd(inst: u32, _opkind: &BOpcode) -> Option<usize> {
        Some(inst.slice(11, 7) as usize)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn parse_rs1(inst: u32, _opkind: &BOpcode) -> Option<usize> {
        Some(inst.slice(19, 15) as usize)
    }

    /// The immediate/unary forms carry NO `rs2` (bits 24:20 are a shamt or
    /// a sub-op selector, not a register) — expose `None` so the lifter
    /// never reads a phantom source register.
    pub fn parse_rs2(inst: u32, opkind: &BOpcode) -> Option<usize> {
        if is_imm_or_unary(opkind) {
            None
        } else {
            Some(inst.slice(24, 20) as usize)
        }
    }

    /// Shamt for the immediate-shift forms (`rori`/`bclri`/… and the W
    /// variants); `None` for the register-register and unary forms.
    pub fn parse_imm(inst: u32, opkind: &BOpcode, _isa: Isa) -> Option<i32> {
        match opkind {
            // RV64 6-bit shamt (bits 25:20).
            BOpcode::BCLRI
            | BOpcode::BEXTI
            | BOpcode::BINVI
            | BOpcode::BSETI
            | BOpcode::RORI
            | BOpcode::SLLI_UW => Some(inst.slice(25, 20) as i32),
            // W-form 5-bit shamt (bits 24:20).
            BOpcode::RORIW => Some(inst.slice(24, 20) as i32),
            _ => None,
        }
    }

    /// True for the immediate-shift and unary Zbb/Zbs/Zba forms (no `rs2`).
    fn is_imm_or_unary(opkind: &BOpcode) -> bool {
        matches!(
            opkind,
            BOpcode::BCLRI
                | BOpcode::BEXTI
                | BOpcode::BINVI
                | BOpcode::BSETI
                | BOpcode::RORI
                | BOpcode::RORIW
                | BOpcode::SLLI_UW
                | BOpcode::CLZ
                | BOpcode::CTZ
                | BOpcode::CPOP
                | BOpcode::SEXT_B
                | BOpcode::SEXT_H
                | BOpcode::ZEXT_H
                | BOpcode::REV8
                | BOpcode::ORC_B
                | BOpcode::CLZW
                | BOpcode::CTZW
                | BOpcode::CPOPW
        )
    }
}
