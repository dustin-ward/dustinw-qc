use crate::parser::Instruction;

pub fn reorder_pass(mut program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    // Assuming That Reording can only take place after
    // native instruction translation...
    // For a given range of swappable instructions:
    // - Separate all RZs and CZ
    //   - No reason to have RZ-CZ-RZ-CZ, instead of RZ-RZ-CZ-CZ etc.
    // - Determine if CZ or RZ should go first.
    //   - Determine number of optimizations that can be made,
    //     then compare with reverse. Take max

    return Ok(new_prog);
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

}
