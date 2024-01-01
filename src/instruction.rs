use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    INVALID,
    RX(f64, u32),
    RZ(f64, u32),
    CZ(u32, u32),
    MEASURE(u32),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, ftr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::RX(f, q) => {
                write!(ftr, "RX({}) {}", f, q)
            }
            Instruction::RZ(f, q) => {
                write!(ftr, "RZ({}) {}", f, q)
            }
            Instruction::CZ(q1, q2) => {
                write!(ftr, "CZ {} {}", q1, q2)
            }
            Instruction::MEASURE(q) => {
                write!(ftr, "MEASURE {}", q)
            }
            Instruction::INVALID => {
                panic!("invalid instruction")
            }
        }
    }
}
