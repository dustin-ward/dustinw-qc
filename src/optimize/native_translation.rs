use std::f64::consts::PI;

use crate::parser::Instruction;

pub fn is_native_instruction(instr: &Instruction) -> bool {
    match instr {
        Instruction::RX(val, _) => {
            *val == 0.0 || val.abs() == PI || val.abs() == PI/2.0
        },
        _ => true,
    }
}

pub fn native_translation_pass(program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    for instr in program {
        if !is_native_instruction(&instr) {
            if let Instruction::RX(val, q) = instr {
                // Use provided identity to translate non-native RX
                new_prog.push(Instruction::RZ(PI/2.0, q));
                new_prog.push(Instruction::RX(PI/2.0, q));
                new_prog.push(Instruction::RZ(val, q));
                new_prog.push(Instruction::RX(-PI/2.0, q));
                new_prog.push(Instruction::RZ(-PI/2.0, q));
            } else {unreachable!()}
        } else {
            new_prog.push(instr);
        }
    }

    return Ok(new_prog);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_native_instruction() {
        assert!(is_native_instruction(&Instruction::RZ(1.11, 0)));
        assert!(is_native_instruction(&Instruction::RX(0.0, 1)));
        assert!(is_native_instruction(&Instruction::RX(PI/2.0, 2)));
        assert!(is_native_instruction(&Instruction::RX(-PI/2.0, 3)));
        assert!(is_native_instruction(&Instruction::RX(PI, 4)));
        assert!(is_native_instruction(&Instruction::RX(-PI, 5)));
        assert!(!is_native_instruction(&Instruction::RX(1.11, 6)));
    }

    #[test]
    fn no_non_native_instructions() {
        let init_instr = vec![
            Instruction::RZ(0.45, 0),
            Instruction::RX(0.0, 0),
            Instruction::RX(PI/2.0, 0),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RX(PI, 0),
            Instruction::RX(-PI, 0),
            Instruction::CZ(0, 1),
            Instruction::MEASURE(1),
        ];

        let expected_instr = init_instr.clone();

        let actual_instr = native_translation_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }

    #[test]
    fn expand_non_native_instruction() {
        let init_instr = vec![
            Instruction::RX(0.45, 0),
        ];

        let expected_instr = vec![
            Instruction::RZ(PI/2.0, 0),
            Instruction::RX(PI/2.0, 0),
            Instruction::RZ(0.45, 0),
            Instruction::RX(-PI/2.0, 0),
            Instruction::RZ(-PI/2.0, 0),
        ];

        let actual_instr = native_translation_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }   

    #[test]
    fn non_native_fp_precision() {
        let init_instr = vec![
            // Enough decimal places to be considered "equal" to PI/2
            // Just going with the default Rust equality. Could
            // implement an 'epsilon' value for varying degrees of
            // accuracy.
            Instruction::RX(1.5707963267948966, 0),

            // One decimal place too short
            Instruction::RX(1.570796326794896, 1),
        ];

        let expected_instr = vec![
            Instruction::RX(1.5707963267948966, 0),

            Instruction::RZ(PI/2.0, 1),
            Instruction::RX(PI/2.0, 1),
            Instruction::RZ(1.570796326794896, 1),
            Instruction::RX(-PI/2.0, 1),
            Instruction::RZ(-PI/2.0, 1),

        ];

        let actual_instr = native_translation_pass(init_instr).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, instr) in expected_instr.iter().enumerate() {
            assert_eq!(instr, &actual_instr[i]);
        }
    }   
}
