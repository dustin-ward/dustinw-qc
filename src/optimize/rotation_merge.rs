use crate::parser::Instruction;

pub fn rotation_merge_pass(mut program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    let mut i = 0;
    while i < program.len() {
        if i < program.len() - 1 {
            if let Instruction::RX(f1, q1) = program[i] {
                if let Instruction::RX(ref mut f2, q2) = program[i + 1] {
                    if q1 == q2 {
                        // Add to next instruction and skip
                        *f2 += f1;
                        i += 1;
                    }
                }
            }
            if let Instruction::RZ(f1, q1) = program[i] {
                if let Instruction::RZ(ref mut f2, q2) = program[i + 1] {
                    if q1 == q2 {
                        *f2 += f1;
                        i += 1;
                    }
                }
            }
        }

        new_prog.push(program[i]);
        i += 1
    }

    return Ok(new_prog);
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn no_rotation_merges() {
        let init_instr = vec![
            Instruction::RZ(0.45, 0),
            Instruction::RX(0.45, 0),
            Instruction::CZ(0, 1),
            Instruction::RX(PI/2.0, 0),
            Instruction::RZ(PI/2.0, 0),
            Instruction::CZ(0, 1),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RX(-PI/2.0, 1),
            Instruction::CZ(0, 1),
        ];

        let expected_instr = init_instr.clone();

        let actual_instr = rotation_merge_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }

    #[test]
    fn cancel_consecutive_cz_instructions() {
        let init_instr = vec![
            Instruction::RZ(0.1, 77),
            Instruction::RZ(0.2, 77),
            Instruction::RX(1.123, 1),
            Instruction::RX(2.234, 3),
            Instruction::MEASURE(1),
            Instruction::RX(0.34, 77),
            Instruction::CZ(3, 1),
            Instruction::RX(PI/2.0, 0),
            Instruction::RX(-PI/2.0, 0),
        ];

        let expected_instr = vec![
            // Floating point shenanigans. Leaving it as is for now...
            Instruction::RZ(0.30000000000000004, 77),
            Instruction::RX(1.123, 1),
            Instruction::RX(2.234, 3),
            Instruction::MEASURE(1),
            Instruction::RX(0.34, 77),
            Instruction::CZ(3, 1),
            Instruction::RX(0.0, 0),
        ];

        let actual_instr = rotation_merge_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }
}
