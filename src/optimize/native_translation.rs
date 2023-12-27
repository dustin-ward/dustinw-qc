use std::f64::consts::PI;

use crate::parser::Instruction;

pub fn native_translation_pass(program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    for instr in program {
        match instr {
            Instruction::RX(val, q) => {
                if !(val == 0.0 || val.abs() == PI || val.abs() == PI/2.0) {
                    // Use provided identity to translate non-native RX
                    new_prog.push(Instruction::RZ(PI/2.0, q));
                    new_prog.push(Instruction::RX(PI/2.0, q));
                    new_prog.push(Instruction::RZ(val, q));
                    new_prog.push(Instruction::RX(-PI/2.0, q));
                    new_prog.push(Instruction::RZ(-PI/2.0, q));
                } else {
                    new_prog.push(instr);
                }
            },
            _ => new_prog.push(instr), 
        }
    }

    return Ok(new_prog);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
