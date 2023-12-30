use crate::parser::Instruction;

pub fn cancellable(cz1: &Instruction, cz2: &Instruction) -> bool {
    if let Instruction::CZ(q1, q2) = cz1 {
        if let Instruction::CZ(q3, q4) = cz2 {
            if (q1 == q3 && q2 == q4) || (q1 == q4 && q2 == q3) {
                return true;
            }
        }
    }
    false
}

pub fn cz_cancel_pass(program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    let mut i = 0;
    while i < program.len() {
        if i < program.len() - 1 {
            if cancellable(&program[i], &program[i + 1]) {
                i += 1;
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
    fn test_cancellable() {
        assert!(cancellable(&Instruction::CZ(1,0), &Instruction::CZ(1,0)));
        assert!(cancellable(&Instruction::CZ(1,0), &Instruction::CZ(0,1)));
        assert!(!cancellable(&Instruction::CZ(1,0), &Instruction::CZ(1,2)));
        assert!(!cancellable(&Instruction::CZ(3,0), &Instruction::CZ(1,2)));
    }

    #[test]
    fn no_consecutive_cz_instructions() {
        let init_instr = vec![
            Instruction::RZ(0.45, 0),
            Instruction::CZ(0, 1),
            Instruction::RX(PI/2.0, 0),
            Instruction::CZ(0, 1),
            Instruction::RX(-PI/2.0, 0),
            Instruction::CZ(0, 1),
        ];

        let expected_instr = init_instr.clone();

        let actual_instr = cz_cancel_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }

    #[test]
    fn cancel_consecutive_cz_instructions() {
        let init_instr = vec![
            Instruction::CZ(1, 2),
            Instruction::CZ(1, 2),
            Instruction::RZ(0.0, 77),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
            Instruction::RX(0.0, 77),
            Instruction::CZ(3, 1),
            Instruction::CZ(1, 3),
        ];

        let expected_instr = vec![
            Instruction::CZ(1, 2),
            Instruction::RZ(0.0, 77),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
            Instruction::RX(0.0, 77),
            Instruction::CZ(1, 3),
        ];

        let actual_instr = cz_cancel_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }
}
