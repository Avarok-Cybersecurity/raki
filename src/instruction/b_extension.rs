//! B extension (bit-manipulation) instructions — the register-register
//! (RFormat) subset of Zba/Zbb/Zbs plus Zicond, PLUS the immediate-shift
//! and unary forms (rori/bclri/…, clz/cpop/rev8/orc.b/…). Added to the
//! Avarok fork so the Ares lifter can support bitmanip-using binaries
//! (modern toolchains emit `rv64gc_zba_zbb_zbs` by default) instead of
//! failing loud. The immediate/unary forms carry no `rs2`; their operand
//! shape is `(rd, rs1[, shamt])`, so they use `RShamtFormat` for Display.

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
    // -- Zbc carry-less multiply --
    CLMUL,
    CLMULH,
    CLMULR,
    // -- RV64-only W-forms --
    ROLW,
    RORW,
    ADD_UW,
    SH1ADD_UW,
    SH2ADD_UW,
    SH3ADD_UW,
    // -- Zbs single-bit immediate --
    BCLRI,
    BEXTI,
    BINVI,
    BSETI,
    // -- Zbb rotate immediate --
    RORI,
    RORIW,
    // -- Zba shift-add immediate --
    SLLI_UW,
    // -- Zbb unary (count / sign-/zero-extend / byte ops) --
    CLZ,
    CTZ,
    CPOP,
    SEXT_B,
    SEXT_H,
    ZEXT_H,
    REV8,
    ORC_B,
    CLZW,
    CTZW,
    CPOPW,
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
            BOpcode::CLMUL => "clmul",
            BOpcode::CLMULH => "clmulh",
            BOpcode::CLMULR => "clmulr",
            BOpcode::ROLW => "rolw",
            BOpcode::RORW => "rorw",
            BOpcode::ADD_UW => "add.uw",
            BOpcode::SH1ADD_UW => "sh1add.uw",
            BOpcode::SH2ADD_UW => "sh2add.uw",
            BOpcode::SH3ADD_UW => "sh3add.uw",
            BOpcode::BCLRI => "bclri",
            BOpcode::BEXTI => "bexti",
            BOpcode::BINVI => "binvi",
            BOpcode::BSETI => "bseti",
            BOpcode::RORI => "rori",
            BOpcode::RORIW => "roriw",
            BOpcode::SLLI_UW => "slli.uw",
            BOpcode::CLZ => "clz",
            BOpcode::CTZ => "ctz",
            BOpcode::CPOP => "cpop",
            BOpcode::SEXT_B => "sext.b",
            BOpcode::SEXT_H => "sext.h",
            BOpcode::ZEXT_H => "zext.h",
            BOpcode::REV8 => "rev8",
            BOpcode::ORC_B => "orc.b",
            BOpcode::CLZW => "clzw",
            BOpcode::CTZW => "ctzw",
            BOpcode::CPOPW => "cpopw",
        };
        write!(f, "{s}")
    }
}

impl Opcode for BOpcode {
    /// Register-register ops are `RFormat` (rd, rs1, rs2). The immediate
    /// and unary Zbb/Zbs/Zba forms carry no `rs2`; `RShamtFormat` prints
    /// just `(rd, rs1)` (the only shape whose Display never touches a
    /// possibly-`None` `rs2`/`imm`), which is correct for both.
    fn get_format(&self) -> InstFormat {
        match self {
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
            | BOpcode::CPOPW => InstFormat::RShamtFormat,
            _ => InstFormat::RFormat,
        }
    }
}
