use crate::parser::Instruction;

pub fn deadcode_pass(program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let new_prog = program
        .into_iter()
        .filter(|instr| match instr {
            Instruction::RZ(val,_) | Instruction::RX(val,_) => {
                if *val == 0.0 {
                    false
                } else {
                    true
                }
            },
            _ => true,
        })
        .collect();

        return Ok(new_prog);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn no_noop_instructions() {
        let init_instr = vec![
            Instruction::RZ(0.45, 0),
            Instruction::RX(PI/2.0, 0),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RX(PI, 0),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
        ];

        let expected_instr = init_instr.clone();

        let actual_instr = deadcode_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }

    #[test]
    fn remove_noop_instructions() {
        let init_instr = vec![
            Instruction::RZ(0.0, 77),
            Instruction::RZ(0.45, 0),
            Instruction::RX(PI/2.0, 0),
            Instruction::RZ(0.0, 77),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RX(PI, 0),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
            Instruction::RX(0.0, 77),
        ];

        let expected_instr = vec![
            Instruction::RZ(0.45, 0),
            Instruction::RX(PI/2.0, 0),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RX(PI, 0),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
        ];

        let actual_instr = deadcode_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }
}
