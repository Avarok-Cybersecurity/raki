//! C extension Insturction.

use super::{InstFormat, Opcode};
use core::fmt::{self, Display, Formatter};

/// Insturctions in C Extension.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum COpcode {
    ADDI4SPN,
    LW,
    SW,
    NOP,
    ADDI,
    JAL,
    LI,
    ADDI16SP,
    LUI,
    SRLI,
    SRAI,
    ANDI,
    SUB,
    XOR,
    OR,
    AND,
    J,
    BEQZ,
    BNEZ,
    SLLI,
    LWSP,
    JR,
    MV,
    EBREAK,
    JALR,
    ADD,
    SWSP,

    //-- rv64 only --
    LD,
    SD,
    ADDIW,
    SUBW,
    ADDW,
    LDSP,
    SDSP,

    //-- C extension of D (RV64 reuses these encodings for f64; the
    //   RV32-only C.FLW/C.FSW share the same primary funct3 but are
    //   NOT decoded here — RV64GC has no C.FLW/C.FSW). The rd'/rs2'
    //   operand is an f-register, the base rs1'/sp an x-register. --
    FLD,
    FSD,
    FLDSP,
    FSDSP,
}

impl Display for COpcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            COpcode::ADDI4SPN => write!(f, "C.addi4spn"),
            COpcode::LW => write!(f, "C.lw"),
            COpcode::SW => write!(f, "C.sw"),
            COpcode::NOP => write!(f, "C.nop"),
            COpcode::ADDI => write!(f, "C.addi"),
            COpcode::JAL => write!(f, "C.jal"),
            COpcode::LI => write!(f, "C.li"),
            COpcode::ADDI16SP => write!(f, "C.addi16sp"),
            COpcode::LUI => write!(f, "C.lui"),
            COpcode::SRLI => write!(f, "C.srli"),
            COpcode::SRAI => write!(f, "C.srai"),
            COpcode::ANDI => write!(f, "C.andi"),
            COpcode::SUB => write!(f, "C.sub"),
            COpcode::XOR => write!(f, "C.xor"),
            COpcode::OR => write!(f, "C.or"),
            COpcode::AND => write!(f, "C.and"),
            COpcode::J => write!(f, "C.j"),
            COpcode::BEQZ => write!(f, "C.beqz"),
            COpcode::BNEZ => write!(f, "C.bnez"),
            COpcode::SLLI => write!(f, "C.slli"),
            COpcode::LWSP => write!(f, "C.lwsp"),
            COpcode::JR => write!(f, "C.jr"),
            COpcode::MV => write!(f, "C.mv"),
            COpcode::EBREAK => write!(f, "C.ebreak"),
            COpcode::JALR => write!(f, "C.jalr"),
            COpcode::ADD => write!(f, "C.add"),
            COpcode::SWSP => write!(f, "C.swsp"),
            COpcode::LD => write!(f, "C.ld"),
            COpcode::SD => write!(f, "C.sd"),
            COpcode::ADDIW => write!(f, "C.addiw"),
            COpcode::SUBW => write!(f, "C.subw"),
            COpcode::ADDW => write!(f, "C.addw"),
            COpcode::LDSP => write!(f, "C.ldsp"),
            COpcode::SDSP => write!(f, "C.sdsp"),
            COpcode::FLD => write!(f, "C.fld"),
            COpcode::FSD => write!(f, "C.fsd"),
            COpcode::FLDSP => write!(f, "C.fldsp"),
            COpcode::FSDSP => write!(f, "C.fsdsp"),
        }
    }
}

impl Opcode for COpcode {
    #[allow(clippy::match_same_arms)]
    fn get_format(&self) -> InstFormat {
        match self {
            // Quadrant 0
            COpcode::LW | COpcode::LD => InstFormat::ClFormat,
            COpcode::ADDI4SPN => InstFormat::CiwFormat,
            COpcode::SW | COpcode::SD => InstFormat::CsFormat,

            // Quadrant 1
            COpcode::JAL | COpcode::J => InstFormat::CjFormat,
            COpcode::BEQZ | COpcode::ANDI | COpcode::SRLI | COpcode::SRAI | COpcode::BNEZ => {
                InstFormat::CbFormat
            }
            COpcode::LI | COpcode::ADDI | COpcode::ADDIW | COpcode::ADDI16SP | COpcode::LUI => {
                InstFormat::CiFormat
            }
            COpcode::NOP => InstFormat::NoOperand,
            COpcode::SUB
            | COpcode::XOR
            | COpcode::OR
            | COpcode::AND
            | COpcode::SUBW
            | COpcode::ADDW => InstFormat::CaFormat,

            // Quadrant 2
            COpcode::LDSP | COpcode::SLLI | COpcode::LWSP => InstFormat::CiFormat,
            COpcode::SDSP | COpcode::SWSP => InstFormat::CssFormat,
            COpcode::JR | COpcode::JALR | COpcode::MV | COpcode::ADD => InstFormat::CrFormat,
            COpcode::EBREAK => InstFormat::NoOperand,

            // RV64 C.FLD/C.FLDSP: rd=f-reg, base=x-reg, imm. The shared
            // FP load/store Display arms render the f-register operand
            // correctly (the integer Cl/Cs arms would mis-name it).
            COpcode::FLD | COpcode::FLDSP => InstFormat::FpLoadFormat,
            COpcode::FSD | COpcode::FSDSP => InstFormat::FpStoreFormat,
        }
    }
}
