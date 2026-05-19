pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::f_extension::FOpcode;
    use crate::Isa;

    /// Decode a single-precision (F-extension) instruction. The caller
    /// (`parse_extension`) has already established this is `Extensions::F`
    /// (LOAD/STORE-FP funct3=010, or OP-FP/FMA with fmt=00).
    pub fn parse_opcode(inst: u32, _isa: Isa) -> Result<FOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let funct7: u8 = u8::try_from(inst.slice(31, 25)).unwrap();
        let rs2f: u8 = u8::try_from(inst.slice(24, 20)).unwrap();

        match opmap {
            0b000_0111 => Ok(FOpcode::FLW),
            0b010_0111 => Ok(FOpcode::FSW),
            0b100_0011 => Ok(FOpcode::FMADD_S),
            0b100_0111 => Ok(FOpcode::FMSUB_S),
            0b100_1011 => Ok(FOpcode::FNMSUB_S),
            0b100_1111 => Ok(FOpcode::FNMADD_S),
            0b101_0011 => match funct7 {
                0b000_0000 => Ok(FOpcode::FADD_S),
                0b000_0100 => Ok(FOpcode::FSUB_S),
                0b000_1000 => Ok(FOpcode::FMUL_S),
                0b000_1100 => Ok(FOpcode::FDIV_S),
                0b010_1100 => Ok(FOpcode::FSQRT_S),
                0b001_0000 => match funct3 {
                    0b000 => Ok(FOpcode::FSGNJ_S),
                    0b001 => Ok(FOpcode::FSGNJN_S),
                    0b010 => Ok(FOpcode::FSGNJX_S),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                0b001_0100 => match funct3 {
                    0b000 => Ok(FOpcode::FMIN_S),
                    0b001 => Ok(FOpcode::FMAX_S),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                // FCVT.S.D: double->single narrow (rs2 field == 00001).
                0b010_0000 => match rs2f {
                    0b00001 => Ok(FOpcode::FCVT_S_D),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b110_0000 => match rs2f {
                    0b00000 => Ok(FOpcode::FCVT_W_S),
                    0b00001 => Ok(FOpcode::FCVT_WU_S),
                    0b00010 => Ok(FOpcode::FCVT_L_S),
                    0b00011 => Ok(FOpcode::FCVT_LU_S),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b110_1000 => match rs2f {
                    0b00000 => Ok(FOpcode::FCVT_S_W),
                    0b00001 => Ok(FOpcode::FCVT_S_WU),
                    0b00010 => Ok(FOpcode::FCVT_S_L),
                    0b00011 => Ok(FOpcode::FCVT_S_LU),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b111_0000 => match funct3 {
                    0b000 => Ok(FOpcode::FMV_X_W),
                    0b001 => Ok(FOpcode::FCLASS_S),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                0b111_1000 => Ok(FOpcode::FMV_W_X),
                0b101_0000 => match funct3 {
                    0b010 => Ok(FOpcode::FEQ_S),
                    0b001 => Ok(FOpcode::FLT_S),
                    0b000 => Ok(FOpcode::FLE_S),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// rd: bits 11:7 — an f-reg for FLW/arith/cvt-to-fp/FMV.W.X, an
    /// x-reg for FCVT.x.S/FMV.X.W/FCLASS/FEQ/FLT/FLE. FSW has no rd.
    pub fn parse_rd(inst: u32, opkind: &FOpcode) -> Option<usize> {
        match opkind {
            FOpcode::FSW => None,
            _ => Some(inst.slice(11, 7) as usize),
        }
    }

    /// rs1: bits 19:15 — an x-reg for FLW/FSW (base) and the
    /// int->fp/FMV.W.X forms, otherwise an f-reg.
    pub fn parse_rs1(inst: u32, opkind: &FOpcode) -> Option<usize> {
        let _ = opkind;
        Some(inst.slice(19, 15) as usize)
    }

    /// rs2: bits 24:20 — an f-reg for FSW(value)/binary-arith/FMA/
    /// compare/sign-inject; the cvt forms encode their variant here so
    /// it is NOT a register operand for FSQRT/FCVT/FMV.X/FCLASS.
    pub fn parse_rs2(inst: u32, opkind: &FOpcode) -> Option<usize> {
        match opkind {
            FOpcode::FSQRT_S
            | FOpcode::FCVT_S_D
            | FOpcode::FCVT_W_S
            | FOpcode::FCVT_WU_S
            | FOpcode::FCVT_L_S
            | FOpcode::FCVT_LU_S
            | FOpcode::FCVT_S_W
            | FOpcode::FCVT_S_WU
            | FOpcode::FCVT_S_L
            | FOpcode::FCVT_S_LU
            | FOpcode::FMV_X_W
            | FOpcode::FMV_W_X
            | FOpcode::FCLASS_S
            | FOpcode::FLW => None,
            _ => Some(inst.slice(24, 20) as usize),
        }
    }

    /// rs3: bits 31:27 — ONLY the FMA family has a 4th f-reg operand.
    pub fn parse_rs3_f(inst: u32, opkind: &FOpcode) -> Option<usize> {
        match opkind {
            FOpcode::FMADD_S
            | FOpcode::FMSUB_S
            | FOpcode::FNMSUB_S
            | FOpcode::FNMADD_S => Some(inst.slice(31, 27) as usize),
            _ => None,
        }
    }

    /// imm: only FLW (I-type, bits 31:20) and FSW (S-type) have one.
    pub fn parse_imm(inst: u32, opkind: &FOpcode) -> Option<i32> {
        match opkind {
            FOpcode::FLW => {
                let imm12 = inst.slice(31, 20) as i32;
                Some(inst.to_signed_nbit(imm12, 12))
            }
            FOpcode::FSW => {
                let hi = inst.slice(31, 25);
                let lo = inst.slice(11, 7);
                let imm12 = ((hi << 5) | lo) as i32;
                Some(inst.to_signed_nbit(imm12, 12))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod test_f {
    #[test]
    #[allow(overflowing_literals)]
    fn f_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::f_extension::FOpcode;
        use crate::{Decode, Isa, OpcodeKind};

        // flw f1, 8(x2)  : imm=8 rs1=2 funct3=010 rd=1 op=0000111
        test_32_in_rv64(
            (8u32 << 20) | (2 << 15) | (0b010 << 12) | (1 << 7) | 0b000_0111,
            OpcodeKind::F(FOpcode::FLW),
            Some(1),
            Some(2),
            None,
            Some(8),
        );
        // fadd.s f3, f1, f2 : funct7=0000000 rs2=2 rs1=1 rd=3 op=1010011
        test_32_in_rv64(
            (0u32 << 25) | (2 << 20) | (1 << 15) | (3 << 7) | 0b101_0011,
            OpcodeKind::F(FOpcode::FADD_S),
            Some(3),
            Some(1),
            Some(2),
            None,
        );
        // fmadd.s f4, f1, f2, f5 : rs3=5 fmt=00 funct2 rs2=2 rs1=1 rd=4
        let fmadd: u32 =
            (5u32 << 27) | (0 << 25) | (2 << 20) | (1 << 15) | (4 << 7) | 0b100_0011;
        let inst = fmadd.decode(Isa::Rv64).unwrap();
        assert_eq!(inst.opc, OpcodeKind::F(FOpcode::FMADD_S));
        assert_eq!(inst.rd, Some(4));
        assert_eq!(inst.rs1, Some(1));
        assert_eq!(inst.rs2, Some(2));
        assert_eq!(inst.rs3, Some(5));
    }
}
