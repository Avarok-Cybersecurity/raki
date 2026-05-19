pub mod bit_32 {
    use super::super::{DecodeUtil, DecodingError};
    use crate::instruction::d_extension::DOpcode;
    use crate::Isa;

    /// Decode a double-precision (D-extension) instruction. The caller
    /// (`parse_extension`) has already established this is `Extensions::D`
    /// (LOAD/STORE-FP funct3=011, or OP-FP/FMA with fmt=01). FCVT.S.D
    /// (single<-double) and FCVT.D.S (double<-single) live in OP-FP with
    /// funct7 0100000/0100001 and a fmt that does NOT match the operand
    /// width, so they are classified here under the D ext where fmt=01
    /// for FCVT.D.S and recognized by rs2 for FCVT.S.D.
    pub fn parse_opcode(inst: u32, _isa: Isa) -> Result<DOpcode, DecodingError> {
        let opmap: u8 = u8::try_from(inst.slice(6, 0)).unwrap();
        let funct3: u8 = u8::try_from(inst.slice(14, 12)).unwrap();
        let funct7: u8 = u8::try_from(inst.slice(31, 25)).unwrap();
        let rs2f: u8 = u8::try_from(inst.slice(24, 20)).unwrap();

        match opmap {
            0b000_0111 => Ok(DOpcode::FLD),
            0b010_0111 => Ok(DOpcode::FSD),
            0b100_0011 => Ok(DOpcode::FMADD_D),
            0b100_0111 => Ok(DOpcode::FMSUB_D),
            0b100_1011 => Ok(DOpcode::FNMSUB_D),
            0b100_1111 => Ok(DOpcode::FNMADD_D),
            0b101_0011 => match funct7 {
                0b000_0001 => Ok(DOpcode::FADD_D),
                0b000_0101 => Ok(DOpcode::FSUB_D),
                0b000_1001 => Ok(DOpcode::FMUL_D),
                0b000_1101 => Ok(DOpcode::FDIV_D),
                0b010_1101 => Ok(DOpcode::FSQRT_D),
                0b001_0001 => match funct3 {
                    0b000 => Ok(DOpcode::FSGNJ_D),
                    0b001 => Ok(DOpcode::FSGNJN_D),
                    0b010 => Ok(DOpcode::FSGNJX_D),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                0b001_0101 => match funct3 {
                    0b000 => Ok(DOpcode::FMIN_D),
                    0b001 => Ok(DOpcode::FMAX_D),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                0b010_0001 => match rs2f {
                    0b00000 => Ok(DOpcode::FCVT_D_S),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b110_0001 => match rs2f {
                    0b00000 => Ok(DOpcode::FCVT_W_D),
                    0b00001 => Ok(DOpcode::FCVT_WU_D),
                    0b00010 => Ok(DOpcode::FCVT_L_D),
                    0b00011 => Ok(DOpcode::FCVT_LU_D),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b110_1001 => match rs2f {
                    0b00000 => Ok(DOpcode::FCVT_D_W),
                    0b00001 => Ok(DOpcode::FCVT_D_WU),
                    0b00010 => Ok(DOpcode::FCVT_D_L),
                    0b00011 => Ok(DOpcode::FCVT_D_LU),
                    _ => Err(DecodingError::InvalidFunct5),
                },
                0b111_0001 => match funct3 {
                    0b000 => Ok(DOpcode::FMV_X_D),
                    0b001 => Ok(DOpcode::FCLASS_D),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                0b111_1001 => Ok(DOpcode::FMV_D_X),
                0b101_0001 => match funct3 {
                    0b010 => Ok(DOpcode::FEQ_D),
                    0b001 => Ok(DOpcode::FLT_D),
                    0b000 => Ok(DOpcode::FLE_D),
                    _ => Err(DecodingError::InvalidFunct3),
                },
                _ => Err(DecodingError::InvalidFunct7),
            },
            _ => Err(DecodingError::InvalidOpcode),
        }
    }

    /// rd: bits 11:7 — f-reg for FLD/arith/cvt-to-fp/FMV.D.X, x-reg for
    /// FCVT.x.D/FMV.X.D/FCLASS/FEQ/FLT/FLE. FSD has no rd.
    pub fn parse_rd(inst: u32, opkind: &DOpcode) -> Option<usize> {
        match opkind {
            DOpcode::FSD => None,
            _ => Some(inst.slice(11, 7) as usize),
        }
    }

    /// rs1: bits 19:15 — x-reg for FLD/FSD (base) and int->fp/FMV.D.X,
    /// otherwise an f-reg.
    pub fn parse_rs1(inst: u32, opkind: &DOpcode) -> Option<usize> {
        let _ = opkind;
        Some(inst.slice(19, 15) as usize)
    }

    /// rs2: bits 24:20 — f-reg for FSD(value)/binary-arith/FMA/compare/
    /// sign-inject; NOT a register for the 2-operand cvt/mv/sqrt forms.
    pub fn parse_rs2(inst: u32, opkind: &DOpcode) -> Option<usize> {
        match opkind {
            DOpcode::FSQRT_D
            | DOpcode::FCVT_D_S
            | DOpcode::FCVT_W_D
            | DOpcode::FCVT_WU_D
            | DOpcode::FCVT_L_D
            | DOpcode::FCVT_LU_D
            | DOpcode::FCVT_D_W
            | DOpcode::FCVT_D_WU
            | DOpcode::FCVT_D_L
            | DOpcode::FCVT_D_LU
            | DOpcode::FMV_X_D
            | DOpcode::FMV_D_X
            | DOpcode::FCLASS_D
            | DOpcode::FLD => None,
            _ => Some(inst.slice(24, 20) as usize),
        }
    }

    /// rs3: bits 31:27 — ONLY the FMA family has a 4th f-reg operand.
    pub fn parse_rs3_d(inst: u32, opkind: &DOpcode) -> Option<usize> {
        match opkind {
            DOpcode::FMADD_D
            | DOpcode::FMSUB_D
            | DOpcode::FNMSUB_D
            | DOpcode::FNMADD_D => Some(inst.slice(31, 27) as usize),
            _ => None,
        }
    }

    /// rm: static rounding-mode (bits 14:12). Present for the D-ext
    /// arith/sqrt/cvt/FMA forms; `None` for FSGNJ*, FMIN/FMAX, FMV.*,
    /// FCLASS, FEQ/FLT/FLE (14:12 is a funct3 selector there) and
    /// FLD/FSD. FCVT.D.S has rm too (the spec lists it; widening
    /// single->double is always exact so the mode is a no-op, but the
    /// field is still decoded for completeness/SSOT).
    pub fn parse_rm_d(inst: u32, opkind: &DOpcode) -> Option<u8> {
        match opkind {
            DOpcode::FADD_D
            | DOpcode::FSUB_D
            | DOpcode::FMUL_D
            | DOpcode::FDIV_D
            | DOpcode::FSQRT_D
            | DOpcode::FCVT_D_S
            | DOpcode::FCVT_W_D
            | DOpcode::FCVT_WU_D
            | DOpcode::FCVT_L_D
            | DOpcode::FCVT_LU_D
            | DOpcode::FCVT_D_W
            | DOpcode::FCVT_D_WU
            | DOpcode::FCVT_D_L
            | DOpcode::FCVT_D_LU
            | DOpcode::FMADD_D
            | DOpcode::FMSUB_D
            | DOpcode::FNMSUB_D
            | DOpcode::FNMADD_D => Some(u8::try_from(inst.slice(14, 12)).unwrap()),
            _ => None,
        }
    }

    /// imm: only FLD (I-type) and FSD (S-type) have one.
    pub fn parse_imm(inst: u32, opkind: &DOpcode) -> Option<i32> {
        match opkind {
            DOpcode::FLD => {
                let imm12 = inst.slice(31, 20) as i32;
                Some(inst.to_signed_nbit(imm12, 12))
            }
            DOpcode::FSD => {
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
mod test_d {
    #[test]
    #[allow(overflowing_literals)]
    fn d_decode_test() {
        use crate::decode::inst_32::test_32_in_rv64;
        use crate::instruction::d_extension::DOpcode;
        use crate::{Decode, Isa, OpcodeKind};

        // fsd f0, 256(x10) = the real fibonacci/hello blocker word
        // 0x10053027 : imm=256 rs2=0 rs1=10 funct3=011 op=0100111
        test_32_in_rv64(
            0x1005_3027,
            OpcodeKind::D(DOpcode::FSD),
            None,
            Some(10),
            Some(0),
            Some(256),
        );
        // fld f1, 16(x10) : imm=16 rs1=10 funct3=011 rd=1 op=0000111
        test_32_in_rv64(
            (16u32 << 20) | (10 << 15) | (0b011 << 12) | (1 << 7) | 0b000_0111,
            OpcodeKind::D(DOpcode::FLD),
            Some(1),
            Some(10),
            None,
            Some(16),
        );
        // fmul.d f3, f1, f2 : funct7=0001001 rs2=2 rs1=1 rd=3 op=1010011
        test_32_in_rv64(
            (0b000_1001u32 << 25) | (2 << 20) | (1 << 15) | (3 << 7) | 0b101_0011,
            OpcodeKind::D(DOpcode::FMUL_D),
            Some(3),
            Some(1),
            Some(2),
            None,
        );
        // fnmadd.d f4, f1, f2, f6 : rs3=6 fmt=01 rs2=2 rs1=1 rd=4
        let fnmadd: u32 = (6u32 << 27)
            | (0b01 << 25)
            | (2 << 20)
            | (1 << 15)
            | (4 << 7)
            | 0b100_1111;
        let inst = fnmadd.decode(Isa::Rv64).unwrap();
        assert_eq!(inst.opc, OpcodeKind::D(DOpcode::FNMADD_D));
        assert_eq!(inst.rs3, Some(6));
    }
}
