//! F extension Instruction (single-precision floating-point, RV32/64F).
//!
//! OPERAND-FILE CONTRACT (the `OpcodeKind::F` discriminant IS this
//! contract — a consumer matching on `OpcodeKind::F(_)` must apply it):
//!   * FLW                       : rd = f-reg, rs1 = x-reg (base), imm
//!   * FSW                       : rs2 = f-reg, rs1 = x-reg (base), imm
//!   * FADD/FSUB/FMUL/FDIV/FSQRT,
//!     FSGNJ/FSGNJN/FSGNJX,
//!     FMIN/FMAX, FCVT.S.D       : rd/rs1/rs2 = f-reg
//!   * FMADD/FMSUB/FNMSUB/FNMADD : rd/rs1/rs2/rs3 = f-reg
//!   * FCVT.W.S/FCVT.WU.S/
//!     FCVT.L.S/FCVT.LU.S,
//!     FMV.X.W, FCLASS.S         : rd = x-reg, rs1 = f-reg
//!   * FCVT.S.W/FCVT.S.WU/
//!     FCVT.S.L/FCVT.S.LU,
//!     FMV.W.X                   : rd = f-reg, rs1 = x-reg
//!   * FEQ.S/FLT.S/FLE.S         : rd = x-reg, rs1/rs2 = f-reg
//! The RISC-V `rm` (rounding-mode) field is not modelled (the consumer
//! assumes round-to-nearest-even — documented in the ares translator).

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Instructions in the F extension (single precision).
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum FOpcode {
    FLW,
    FSW,
    FMADD_S,
    FMSUB_S,
    FNMSUB_S,
    FNMADD_S,
    FADD_S,
    FSUB_S,
    FMUL_S,
    FDIV_S,
    FSQRT_S,
    FSGNJ_S,
    FSGNJN_S,
    FSGNJX_S,
    FMIN_S,
    FMAX_S,
    /// FCVT.S.D — double->single narrowing (result is single, so the
    /// `fmt` field is 00 and `parse_extension` routes it to F).
    FCVT_S_D,
    FCVT_W_S,
    FCVT_WU_S,
    FCVT_L_S,
    FCVT_LU_S,
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_S_L,
    FCVT_S_LU,
    FMV_X_W,
    FMV_W_X,
    FEQ_S,
    FLT_S,
    FLE_S,
    FCLASS_S,
}

impl Display for FOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            FOpcode::FLW => "flw",
            FOpcode::FSW => "fsw",
            FOpcode::FMADD_S => "fmadd.s",
            FOpcode::FMSUB_S => "fmsub.s",
            FOpcode::FNMSUB_S => "fnmsub.s",
            FOpcode::FNMADD_S => "fnmadd.s",
            FOpcode::FADD_S => "fadd.s",
            FOpcode::FSUB_S => "fsub.s",
            FOpcode::FMUL_S => "fmul.s",
            FOpcode::FDIV_S => "fdiv.s",
            FOpcode::FSQRT_S => "fsqrt.s",
            FOpcode::FSGNJ_S => "fsgnj.s",
            FOpcode::FSGNJN_S => "fsgnjn.s",
            FOpcode::FSGNJX_S => "fsgnjx.s",
            FOpcode::FMIN_S => "fmin.s",
            FOpcode::FMAX_S => "fmax.s",
            FOpcode::FCVT_S_D => "fcvt.s.d",
            FOpcode::FCVT_W_S => "fcvt.w.s",
            FOpcode::FCVT_WU_S => "fcvt.wu.s",
            FOpcode::FCVT_L_S => "fcvt.l.s",
            FOpcode::FCVT_LU_S => "fcvt.lu.s",
            FOpcode::FCVT_S_W => "fcvt.s.w",
            FOpcode::FCVT_S_WU => "fcvt.s.wu",
            FOpcode::FCVT_S_L => "fcvt.s.l",
            FOpcode::FCVT_S_LU => "fcvt.s.lu",
            FOpcode::FMV_X_W => "fmv.x.w",
            FOpcode::FMV_W_X => "fmv.w.x",
            FOpcode::FEQ_S => "feq.s",
            FOpcode::FLT_S => "flt.s",
            FOpcode::FLE_S => "fle.s",
            FOpcode::FCLASS_S => "fclass.s",
        };
        write!(f, "{s}")
    }
}

impl Opcode for FOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            FOpcode::FLW => InstFormat::FpLoadFormat,
            FOpcode::FSW => InstFormat::FpStoreFormat,
            FOpcode::FMADD_S
            | FOpcode::FMSUB_S
            | FOpcode::FNMSUB_S
            | FOpcode::FNMADD_S => InstFormat::FpR4Format,
            FOpcode::FADD_S
            | FOpcode::FSUB_S
            | FOpcode::FMUL_S
            | FOpcode::FDIV_S
            | FOpcode::FSGNJ_S
            | FOpcode::FSGNJN_S
            | FOpcode::FSGNJX_S
            | FOpcode::FMIN_S
            | FOpcode::FMAX_S
            | FOpcode::FEQ_S
            | FOpcode::FLT_S
            | FOpcode::FLE_S => InstFormat::FpR3Format,
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
            | FOpcode::FCLASS_S => InstFormat::FpR2Format,
        }
    }
}
