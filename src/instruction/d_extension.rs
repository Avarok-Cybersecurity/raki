//! D extension Instruction (double-precision floating-point, RV32/64D).
//!
//! OPERAND-FILE CONTRACT (same shape as `f_extension`; the
//! `OpcodeKind::D` discriminant IS the contract):
//!   * FLD                        : rd = f-reg, rs1 = x-reg (base), imm
//!   * FSD                        : rs2 = f-reg, rs1 = x-reg (base), imm
//!   * FADD/FSUB/FMUL/FDIV/FSQRT,
//!     FSGNJ*, FMIN/FMAX,
//!     FCVT.D.S                   : rd/rs1/rs2 = f-reg
//!   * FMADD/FMSUB/FNMSUB/FNMADD  : rd/rs1/rs2/rs3 = f-reg
//!   * FCVT.W.D/FCVT.WU.D/
//!     FCVT.L.D/FCVT.LU.D,
//!     FMV.X.D, FCLASS.D          : rd = x-reg, rs1 = f-reg
//!   * FCVT.D.W/FCVT.D.WU/
//!     FCVT.D.L/FCVT.D.LU,
//!     FMV.D.X                    : rd = f-reg, rs1 = x-reg
//!   * FEQ.D/FLT.D/FLE.D          : rd = x-reg, rs1/rs2 = f-reg
//! `rm` is not modelled (round-to-nearest-even assumed).

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Instructions in the D extension (double precision).
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum DOpcode {
    FLD,
    FSD,
    FMADD_D,
    FMSUB_D,
    FNMSUB_D,
    FNMADD_D,
    FADD_D,
    FSUB_D,
    FMUL_D,
    FDIV_D,
    FSQRT_D,
    FSGNJ_D,
    FSGNJN_D,
    FSGNJX_D,
    FMIN_D,
    FMAX_D,
    FCVT_D_S,
    FCVT_W_D,
    FCVT_WU_D,
    FCVT_L_D,
    FCVT_LU_D,
    FCVT_D_W,
    FCVT_D_WU,
    FCVT_D_L,
    FCVT_D_LU,
    FMV_X_D,
    FMV_D_X,
    FEQ_D,
    FLT_D,
    FLE_D,
    FCLASS_D,
}

impl Display for DOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            DOpcode::FLD => "fld",
            DOpcode::FSD => "fsd",
            DOpcode::FMADD_D => "fmadd.d",
            DOpcode::FMSUB_D => "fmsub.d",
            DOpcode::FNMSUB_D => "fnmsub.d",
            DOpcode::FNMADD_D => "fnmadd.d",
            DOpcode::FADD_D => "fadd.d",
            DOpcode::FSUB_D => "fsub.d",
            DOpcode::FMUL_D => "fmul.d",
            DOpcode::FDIV_D => "fdiv.d",
            DOpcode::FSQRT_D => "fsqrt.d",
            DOpcode::FSGNJ_D => "fsgnj.d",
            DOpcode::FSGNJN_D => "fsgnjn.d",
            DOpcode::FSGNJX_D => "fsgnjx.d",
            DOpcode::FMIN_D => "fmin.d",
            DOpcode::FMAX_D => "fmax.d",
            DOpcode::FCVT_D_S => "fcvt.d.s",
            DOpcode::FCVT_W_D => "fcvt.w.d",
            DOpcode::FCVT_WU_D => "fcvt.wu.d",
            DOpcode::FCVT_L_D => "fcvt.l.d",
            DOpcode::FCVT_LU_D => "fcvt.lu.d",
            DOpcode::FCVT_D_W => "fcvt.d.w",
            DOpcode::FCVT_D_WU => "fcvt.d.wu",
            DOpcode::FCVT_D_L => "fcvt.d.l",
            DOpcode::FCVT_D_LU => "fcvt.d.lu",
            DOpcode::FMV_X_D => "fmv.x.d",
            DOpcode::FMV_D_X => "fmv.d.x",
            DOpcode::FEQ_D => "feq.d",
            DOpcode::FLT_D => "flt.d",
            DOpcode::FLE_D => "fle.d",
            DOpcode::FCLASS_D => "fclass.d",
        };
        write!(f, "{s}")
    }
}

impl Opcode for DOpcode {
    fn get_format(&self) -> InstFormat {
        match self {
            DOpcode::FLD => InstFormat::FpLoadFormat,
            DOpcode::FSD => InstFormat::FpStoreFormat,
            DOpcode::FMADD_D
            | DOpcode::FMSUB_D
            | DOpcode::FNMSUB_D
            | DOpcode::FNMADD_D => InstFormat::FpR4Format,
            DOpcode::FADD_D
            | DOpcode::FSUB_D
            | DOpcode::FMUL_D
            | DOpcode::FDIV_D
            | DOpcode::FSGNJ_D
            | DOpcode::FSGNJN_D
            | DOpcode::FSGNJX_D
            | DOpcode::FMIN_D
            | DOpcode::FMAX_D
            | DOpcode::FEQ_D
            | DOpcode::FLT_D
            | DOpcode::FLE_D => InstFormat::FpR3Format,
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
            | DOpcode::FCLASS_D => InstFormat::FpR2Format,
        }
    }
}
