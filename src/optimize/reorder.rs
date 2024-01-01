use std::cmp::Ordering;

use crate::instruction::Instruction;

use super::cz_cancel::cancellable;

pub fn reorder_pass(program: Vec<Instruction>) -> Result<Vec<Instruction>, String> {
    let mut new_prog: Vec<Instruction> = Vec::new();

    // Assuming That Reording can only take place after
    // native instruction translation...
    // For a given range of swappable instructions:
    // - Separate all RZs and CZ
    //   - No reason to have RZ-CZ-RZ-CZ, instead of RZ-RZ-CZ-CZ etc.
    // - Determine if CZ or RZ should go first.
    //   - Determine number of optimizations that can be made,
    //     then compare with reverse. Take max

    let mut i = 0;
    while i < program.len() {
        match program[i] {
            Instruction::RZ(_, q1) | Instruction::CZ(q1, _) => {
                let mut range_instrs: Vec<Instruction> = Vec::new();
                range_instrs.push(program[i]);
                i += 1;

                // Find range of swappable instructions
                while i < program.len() {
                    match program[i] {
                        Instruction::RZ(_, q2) | Instruction::CZ(q2, _) => {
                            if q1 == q2 {
                                range_instrs.push(program[i]);
                                i += 1;
                                continue;
                            }
                        }
                        _ => {}
                    }

                    i -= 1;
                    break;
                }

                // Prefer the order RZ, CZ, CZ1, CZ2, where CZ1 & CZ2
                // are cancellable
                range_instrs.sort_by(|a, b| match a {
                    Instruction::RZ(_, _) => Ordering::Less,
                    Instruction::CZ(_, _) => match b {
                        Instruction::RZ(_, _) => Ordering::Greater,
                        Instruction::CZ(_, _) => {
                            if cancellable(a, b) {
                                Ordering::Equal
                            } else {
                                Ordering::Less
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                });

                new_prog.append(&mut range_instrs);
            }
            _ => {
                new_prog.push(program[i]);
            }
        }

        i += 1;
    }

    return Ok(new_prog);
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_reordering_pass() {
        let init_instrs = vec![
            Instruction::RX(0.0, 1),
            Instruction::RZ(1.1, 1),
            Instruction::RZ(1.1, 2),
            Instruction::CZ(3, 2),
            Instruction::RZ(1.1, 1),
            Instruction::CZ(1, 0),
            Instruction::CZ(0, 2),
            Instruction::RZ(0.45, 2),
            Instruction::CZ(1, 2),
            Instruction::RZ(1.4, 2),
        ];

        let expected_instrs = init_instrs.clone();

        let actual_instrs = reorder_pass(init_instrs).unwrap();

        println!("Expected: {:?}", expected_instrs);
        println!("Actual:   {:?}", actual_instrs);

        assert_eq!(expected_instrs.len(), actual_instrs.len());

        for (i, instr) in expected_instrs.iter().enumerate() {
            assert_eq!(instr, &actual_instrs[i]);
        }
    }

    #[test]
    fn test_reorder_pass() {
        let init_instrs = vec![
            Instruction::RX(0.0, 1),
            Instruction::RZ(1.1, 1),
            Instruction::RZ(1.1, 1),
            Instruction::CZ(1, 2),
            Instruction::RZ(1.1, 1),
            Instruction::CZ(1, 0),
            Instruction::CZ(1, 2),
            Instruction::RZ(0.45, 2),
            Instruction::CZ(1, 2),
            Instruction::RZ(1.4, 1),
        ];

        let expected_instrs = vec![
            Instruction::RX(0.0, 1),
            Instruction::RZ(1.1, 1),
            Instruction::RZ(1.1, 1),
            Instruction::RZ(1.1, 1),
            Instruction::CZ(1, 0),
            Instruction::CZ(1, 2),
            Instruction::CZ(1, 2),
            Instruction::RZ(0.45, 2),
            Instruction::RZ(1.4, 1),
            Instruction::CZ(1, 2),
        ];

        let actual_instrs = reorder_pass(init_instrs).unwrap();

        println!("Expected: {:?}", expected_instrs);
        println!("Actual:   {:?}", actual_instrs);

        assert_eq!(expected_instrs.len(), actual_instrs.len());

        for (i, instr) in expected_instrs.iter().enumerate() {
            assert_eq!(instr, &actual_instrs[i]);
        }
    }
}
