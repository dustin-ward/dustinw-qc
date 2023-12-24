use crate::lexer;
use crate::lexer::TokenType;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    INVALID,
    RX{f: f64, q: u32},
    RZ{f: f64, q: u32},
    CZ{q1: u32, q2: u32},
    MEASURE{q: u32},
}

pub fn parse(tokens: &Vec<lexer::Token>) -> Result<Vec<Instruction>, String> {
    let mut program: Vec<Instruction> = Vec::new();

    let mut iter = tokens.iter();
    while let Some(inst_token) = iter.next() {

        // First inst_token in line should match function tokens.
        // (RX, RZ, etc.)
        let mut new_inst = match inst_token.t {
            TokenType::RX => Instruction::RX{f: 0.0, q: 0},
            TokenType::RZ => Instruction::RZ{f: 0.0, q: 0},
            TokenType::CZ => Instruction::CZ{q1: 0, q2: 0},
            TokenType::MEASURE => Instruction::MEASURE{q: 0},
            _ => Instruction::INVALID,
        };
        if new_inst == Instruction::INVALID {
            return Err(format!("Invalid inst_token at {}:{}, expected instruction. (RX, RZ, etc.)", inst_token.line, inst_token.pos));
        }

        // Match the rest of the tokens up to EOL
        let mut rem_tokens: VecDeque<&lexer::Token> = VecDeque::new();
        while let Some(next_token) = iter.next() {
            if next_token.t == TokenType::EOL {
                break
            }
            rem_tokens.push_back(next_token);
        }
        if rem_tokens.len() == 0 {
            return Err(format!("Invalid or missing token sequence after instruction at {}:{}", inst_token.line, inst_token.pos));
        }

        match new_inst {
            Instruction::RX{..} | Instruction::RZ{..} => {
                // Remaining tokens in the form '(', optional '-', Float|Int, ')', Int
                let mut f_val;
                let q_val;

                // Left paren token
                if let Some(token) = rem_tokens.pop_front() {
                    if token.t != TokenType::LParen {
                        return Err(format!("Unexpected token at {}:{}, expected '('", token.line, token.pos));
                    }
                } else {
                    return Err(format!("Missing '(' after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                // Float value
                if let Some(mut token) = rem_tokens.pop_front() {
                    // Handle possible negative sign
                    let mut negative = false;
                    if token.t == TokenType::Negative {
                        negative = true;
                        let res = rem_tokens.pop_front();
                        if res == None {
                            return Err(format!("Missing parameter for instruction at {}:{}", inst_token.line, inst_token.pos));
                        } else {
                            token = res.unwrap();
                        }
                    }

                    // Convert possible integer to float
                    match token.t {
                        TokenType::Float(f) => f_val = f,
                        TokenType::Integer(u) => f_val = u as f64,
                        _ => return Err(format!("Invalid float value for instruction at {}:{}", inst_token.line, inst_token.pos)),
                    }
                    if negative {
                        f_val = -f_val;
                    }
                } else {
                    return Err(format!("Missing parameter for instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                // Right paren token
                if let Some(token) = rem_tokens.pop_front() {
                    if token.t != TokenType::RParen {
                        return Err(format!("Unexpected token at {}:{}, expected ')'", token.line, token.pos));
                    }
                } else {
                    return Err(format!("Missing ')' after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                // QBit index
                if let Some(token) = rem_tokens.pop_front() {
                    match token.t {
                        TokenType::Integer(u) => q_val = u,
                        _ => return Err(format!("Unexpected token at {}:{}, expected qbit index", token.line, token.pos)),
                    }
                } else {
                    return Err(format!("Missing qbit index after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                new_inst = match new_inst {
                    Instruction::RX{..} => Instruction::RX{f:f_val, q:q_val},
                    Instruction::RZ{..} => Instruction::RZ{f:f_val, q:q_val},
                    _ => Instruction::INVALID,
                }
            },

            Instruction::CZ{..} => {
                let q1_val;
                let q2_val;

                // QBit index 1
                if let Some(token) = rem_tokens.pop_front() {
                    match token.t {
                        TokenType::Integer(u) => q1_val = u,
                        _ => return Err(format!("Unexpected token at {}:{}, expected qbit index", token.line, token.pos)),
                    }
                } else {
                    return Err(format!("Missing qbit index after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                // QBit index 2
                if let Some(token) = rem_tokens.pop_front() {
                    match token.t {
                        TokenType::Integer(u) => q2_val = u,
                        _ => return Err(format!("Unexpected token at {}:{}, expected qbit index", token.line, token.pos)),
                    }
                } else {
                    return Err(format!("Missing qbit index after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                new_inst = match new_inst {
                    Instruction::CZ{..} => Instruction::CZ{q1:q1_val, q2:q2_val},
                    _ => Instruction::INVALID,
                }
            },

            Instruction::MEASURE{..} => {
                let q_val;

                // QBit index
                if let Some(token) = rem_tokens.pop_front() {
                    match token.t {
                        TokenType::Integer(u) => q_val = u,
                        _ => return Err(format!("Unexpected token at {}:{}, expected qbit index", token.line, token.pos)),
                    }
                } else {
                    return Err(format!("Missing qbit index after instruction at {}:{}", inst_token.line, inst_token.pos));
                }

                new_inst = match new_inst {
                    Instruction::MEASURE{..} => Instruction::MEASURE{q:q_val},
                    _ => Instruction::INVALID,
                }
            },

            _ => {},
        };

        program.push(new_inst);
    }

    return Ok(program);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA_DIR: &str = "examples/testdata";

    #[test]
    fn parse_sample_1() {
        let expected_instr = vec![
            Instruction::RX{f: 0.45, q: 0},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_1.quil");
        let tokens = lexer::tokenize(&test_filename).unwrap();
        let actual_instr = parse(&tokens).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, ex_instr) in expected_instr.iter().enumerate() {
            assert_eq!(ex_instr, &actual_instr[i]);
        }
    }

    #[test]
    fn parse_sample_2() {
        let expected_instr = vec![
            Instruction::RX{f: 0.45, q: 0},
            Instruction::RZ{f: -1.0, q: 0},
            Instruction::RZ{f: 1.0, q: 1},
            Instruction::RX{f: 0.45, q: 1},
            Instruction::CZ{q1: 0, q2: 1},
            Instruction::MEASURE{q: 0},
            Instruction::MEASURE{q: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_2.quil");
        let tokens = lexer::tokenize(&test_filename).unwrap();
        let actual_instr = parse(&tokens).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, ex_instr) in expected_instr.iter().enumerate() {
            assert_eq!(ex_instr, &actual_instr[i]);
        }
    }

    #[test]
    fn parse_sample_3() {
        let expected_instr = vec![
            Instruction::RX{f: 0.45, q: 0},
            Instruction::RZ{f: -1.0, q: 0},
            Instruction::RZ{f: 1.0, q: 1},
            Instruction::RX{f: 0.45, q: 1},
            Instruction::CZ{q1: 1, q2: 0},
            Instruction::RZ{f: 1.5707963267948966, q: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_3.quil");
        let tokens = lexer::tokenize(&test_filename).unwrap();
        let actual_instr = parse(&tokens).unwrap();

        assert_eq!(expected_instr.len(), actual_instr.len());

        for (i, ex_instr) in expected_instr.iter().enumerate() {
            assert_eq!(ex_instr, &actual_instr[i]);
        }
    }
}
