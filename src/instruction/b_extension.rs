//! B extension (bit-manipulation) instructions — the register-register
//! (RFormat) subset of Zba/Zbb/Zbs plus Zicond. Added to the Avarok fork
//! so the Ares lifter can support bitmanip-using binaries (modern
//! toolchains emit `rv64gc_zba_zbb_zbs` by default) instead of failing
//! loud. Immediate-shift / unary forms (rori, bseti, clz, rev8, …) are
//! intentionally not yet decoded — they still fail loud (safe).

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Register-register bit-manipulation opcodes (Zba/Zbb/Zbs/Zicond).
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BOpcode {
    // -- Zbb logical-with-negate / min-max / rotate --
    ANDN,
    ORN,
    XNOR,
    MIN,
    MINU,
    MAX,
    MAXU,
    ROL,
    ROR,
    // -- Zba shift-add --
    SH1ADD,
    SH2ADD,
    SH3ADD,
    // -- Zbs single-bit --
    BCLR,
    BEXT,
    BINV,
    BSET,
    // -- Zicond conditional-zero --
    CZERO_EQZ,
    CZERO_NEZ,
    // -- RV64-only W-forms --
    ROLW,
    RORW,
    ADD_UW,
    SH1ADD_UW,
    SH2ADD_UW,
    SH3ADD_UW,
}

impl Display for BOpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            BOpcode::ANDN => "andn",
            BOpcode::ORN => "orn",
            BOpcode::XNOR => "xnor",
            BOpcode::MIN => "min",
            BOpcode::MINU => "minu",
            BOpcode::MAX => "max",
            BOpcode::MAXU => "maxu",
            BOpcode::ROL => "rol",
            BOpcode::ROR => "ror",
            BOpcode::SH1ADD => "sh1add",
            BOpcode::SH2ADD => "sh2add",
            BOpcode::SH3ADD => "sh3add",
            BOpcode::BCLR => "bclr",
            BOpcode::BEXT => "bext",
            BOpcode::BINV => "binv",
            BOpcode::BSET => "bset",
            BOpcode::CZERO_EQZ => "czero.eqz",
            BOpcode::CZERO_NEZ => "czero.nez",
            BOpcode::ROLW => "rolw",
            BOpcode::RORW => "rorw",
            BOpcode::ADD_UW => "add.uw",
            BOpcode::SH1ADD_UW => "sh1add.uw",
            BOpcode::SH2ADD_UW => "sh2add.uw",
            BOpcode::SH3ADD_UW => "sh3add.uw",
        };
        write!(f, "{s}")
    }
}

impl Opcode for BOpcode {
    /// Every modelled B-extension op is a register-register RFormat
    /// instruction (rd, rs1, rs2).
    fn get_format(&self) -> InstFormat {
        InstFormat::RFormat
    }
}
