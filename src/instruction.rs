#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    INVALID,
    RX(f64, u32),
    RZ(f64, u32),
    CZ(u32, u32),
    MEASURE(u32),
}
