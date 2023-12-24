use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    UNDEF,

    Float(f64),
    Integer(u32),

    LParen,
    RParen,
    Negative,
    EOL,

    RX,
    RZ,
    CZ,
    MEASURE,
}

// Wrap token type with line+pos info
#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub line: u32,
    pub pos: u32,
    pub len: usize,
}

// Read file by line from:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Returns vector of tokens derived from file
pub fn tokenize(filename: &str) -> Result<Vec<Token>, String> {
    match read_lines(filename) {
        Ok(lines) => {
            let mut tokens: Vec<Token> = Vec::new();

            for (lineno, line_result) in lines.enumerate() {
                let line = line_result.unwrap();

                // Iterate over chars with 'peekable' trait to avoid
                // consuming next character
                let mut iter = line.chars().enumerate().peekable();
                while iter.peek() != None {
                    let (pos, ch) = iter.next().unwrap();

                    if ch.is_whitespace() {
                        continue;
                    }

                    let mut new_token = Token{
                        t: TokenType::UNDEF,
                        line: lineno as u32 + 1,
                        pos : pos as u32 + 1,
                        len: 1
                    };

                    new_token.t = match ch {
                        // Alphabetic tokens (RZ, RZ, etc.)
                        ch if ch.is_alphabetic() => {
                            let mut ident = String::from(ch.to_string());

                            while let Some((_, peek_ch)) = iter.peek() {
                                if !peek_ch.is_alphabetic() {
                                    break;
                                }

                                let (_, next_ch) = iter.next().unwrap();
                                new_token.len += 1;
                                ident.push(next_ch);
                            }

                            match ident.as_str() {
                                "RX" => TokenType::RX,
                                "RZ" => TokenType::RZ,
                                "CZ" => TokenType::CZ,
                                "MEASURE" => TokenType::MEASURE,
                                _ => TokenType::UNDEF,
                            }
                        },

                        // Numeric tokens (Floats or Ints)
                        ch if ch.is_numeric() => {
                            let mut ident = String::from(ch.to_string());
                            let mut is_float = false;

                            while let Some((_, peek_ch)) = iter.peek() {
                                if !(peek_ch.is_numeric() || *peek_ch == '.') {
                                    break;
                                }

                                let (_, next_ch) = iter.next().unwrap();
                                if next_ch == '.' {
                                    is_float = true;
                                }
                                new_token.len += 1;
                                ident.push(next_ch);
                            }

                            if is_float {
                                let f: f64 = ident.parse().unwrap();
                                TokenType::Float(f)
                            } else {
                                let u: u32 = ident.parse().unwrap();
                                TokenType::Integer(u)
                            }
                        },

                        // Misc Tokens
                        '(' => TokenType::LParen,
                        ')' => TokenType::RParen,
                        '-' => TokenType::Negative,
                        ';' => TokenType::EOL,
                        _ => TokenType::UNDEF,
                    };

                    if new_token.t == TokenType::UNDEF {
                        return Err(format!("Undefined token at {}:{} \"{}\"", lineno+1, pos+1, &line[pos..pos+new_token.len]));
                    }

                    tokens.push(new_token);
                }

                // If last token was not an end-of-line token, insert one
                if let Some(last_token) = tokens.last() {
                    if last_token.t != TokenType::EOL {
                        tokens.push(Token{
                            t: TokenType::EOL,
                            line: lineno as u32 + 1,
                            pos: line.len() as u32 + 1,
                            len: 1,
                        });
                    }
                }
            }

            return Ok(tokens);
        },
        Err(e) => {
            return Err(format!("Error reading file: {}", e));
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA_DIR: &str = "examples/testdata";

    // General Tests

    #[test]
    fn get_numeric_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::Float(1.234), line: 1, pos: 1, len: 5},
            Token{t: TokenType::EOL, line: 1, pos: 6, len: 1},
            Token{t: TokenType::Integer(0), line: 2, pos: 1, len: 1},
            Token{t: TokenType::EOL, line: 2, pos: 2, len: 1},
            Token{t: TokenType::Float(12345.6789), line: 3, pos: 1, len: 10},
            Token{t: TokenType::EOL, line: 3, pos: 11, len: 1},
            Token{t: TokenType::Float(1.), line: 4, pos: 1, len: 2},
            Token{t: TokenType::EOL, line: 4, pos: 3, len: 1},
            Token{t: TokenType::Float(0.0), line: 5, pos: 1, len: 3},
            Token{t: TokenType::EOL, line: 5, pos: 4, len: 1},
            Token{t: TokenType::Integer(1), line: 6, pos: 1, len: 1},
            Token{t: TokenType::EOL, line: 6, pos: 2, len: 1},
            Token{t: TokenType::Integer(2000000000), line: 7, pos: 1, len: 10},
            Token{t: TokenType::EOL, line: 7, pos: 11, len: 1},
            Token{t: TokenType::Float(0.0001), line: 8, pos: 1, len: 6},
            Token{t: TokenType::EOL, line: 8, pos: 7, len: 1},
            Token{t: TokenType::Integer(12345), line: 9, pos: 1, len: 5},
            Token{t: TokenType::EOL, line: 9, pos: 6, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/get_numeric_tokens.testdata");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }

    #[test]
    fn invalid_numeric_tokens() {
        // .123456
        let mut test_filename = format!("{TESTDATA_DIR}/invalid_numeric_tokens1.testdata");
        let mut err = tokenize(&test_filename).unwrap_err();
        assert_eq!(err, "Undefined token at 1:1 \".\"");
        
        // 0.a132
        test_filename = format!("{TESTDATA_DIR}/invalid_numeric_tokens2.testdata");
        err = tokenize(&test_filename).unwrap_err();
        assert_eq!(err, "Undefined token at 1:3 \"a\"");

        // 1,000,000
        test_filename = format!("{TESTDATA_DIR}/invalid_numeric_tokens3.testdata");
        err = tokenize(&test_filename).unwrap_err();
        assert_eq!(err, "Undefined token at 1:2 \",\"");
    }

    #[test]
    fn get_function_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::RX, line: 1, pos: 1, len: 2},
            Token{t: TokenType::EOL, line: 1, pos: 3, len: 1},
            Token{t: TokenType::RZ, line: 2, pos: 1, len: 2},
            Token{t: TokenType::EOL, line: 2, pos: 3, len: 1},
            Token{t: TokenType::CZ, line: 3, pos: 1, len: 2},
            Token{t: TokenType::EOL, line: 3, pos: 3, len: 1},
            Token{t: TokenType::MEASURE, line: 4, pos: 1, len: 7},
            Token{t: TokenType::EOL, line: 4, pos: 8, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/get_function_tokens.testdata");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }
    
    #[test]
    fn invalid_function_tokens() {
        // rx(0.0) 0
        let mut test_filename = format!("{TESTDATA_DIR}/invalid_function_tokens1.testdata");
        let mut err = tokenize(&test_filename).unwrap_err();
        assert_eq!(err, "Undefined token at 1:1 \"rx\"");
        
        // RAX(1.0) 1
        test_filename = format!("{TESTDATA_DIR}/invalid_function_tokens2.testdata");
        err = tokenize(&test_filename).unwrap_err();
        assert_eq!(err, "Undefined token at 1:1 \"RAX\"");
    }

    #[test]
    fn get_misc_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::LParen, line: 1, pos: 1, len: 1},
            Token{t: TokenType::EOL, line: 1, pos: 2, len: 1},
            Token{t: TokenType::RParen, line: 1, pos: 3, len: 1},
            Token{t: TokenType::EOL, line: 1, pos: 4, len: 1},
            Token{t: TokenType::Negative, line: 2, pos: 1, len: 1},
            Token{t: TokenType::LParen, line: 2, pos: 2, len: 1},
            Token{t: TokenType::EOL, line: 2, pos: 3, len: 1},
            Token{t: TokenType::EOL, line: 3, pos: 1, len: 1},
            Token{t: TokenType::RParen, line: 3, pos: 2, len: 1},
            Token{t: TokenType::EOL, line: 3, pos: 3, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/get_misc_tokens.testdata");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }

    // Sample Input Tests

    #[test]
    fn tokenize_sample_1() {
        let expected_tokens = vec![
            Token{t: TokenType::RX, line: 1, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 1, pos: 3, len: 1},
            Token{t: TokenType::Float(0.45), line: 1, pos: 4, len: 4},
            Token{t: TokenType::RParen, line: 1, pos: 8, len: 1},
            Token{t: TokenType::Integer(0), line: 1, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 1, pos: 11, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_1.quil");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }

    #[test]
    fn tokenize_sample_2() {
        let expected_tokens = vec![
            Token{t: TokenType::RX, line: 1, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 1, pos: 3, len: 1},
            Token{t: TokenType::Float(0.45), line: 1, pos: 4, len: 4},
            Token{t: TokenType::RParen, line: 1, pos: 8, len: 1},
            Token{t: TokenType::Integer(0), line: 1, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 1, pos: 11, len: 1},

            Token{t: TokenType::RZ, line: 2, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 2, pos: 3, len: 1},
            Token{t: TokenType::Negative, line: 2, pos: 4, len: 1},
            Token{t: TokenType::Float(1.0), line: 2, pos: 5, len: 3},
            Token{t: TokenType::RParen, line: 2, pos: 8, len: 1},
            Token{t: TokenType::Integer(0), line: 2, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 2, pos: 11, len: 1},

            Token{t: TokenType::RZ, line: 3, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 3, pos: 3, len: 1},
            Token{t: TokenType::Float(1.0), line: 3, pos: 4, len: 3},
            Token{t: TokenType::RParen, line: 3, pos: 7, len: 1},
            Token{t: TokenType::Integer(1), line: 3, pos: 9, len: 1},
            Token{t: TokenType::EOL, line: 3, pos: 10, len: 1},

            Token{t: TokenType::RX, line: 4, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 4, pos: 3, len: 1},
            Token{t: TokenType::Float(0.45), line: 4, pos: 4, len: 4},
            Token{t: TokenType::RParen, line: 4, pos: 8, len: 1},
            Token{t: TokenType::Integer(1), line: 4, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 4, pos: 11, len: 1},
            
            Token{t: TokenType::CZ, line: 5, pos: 1, len: 2},
            Token{t: TokenType::Integer(0), line: 5, pos: 4, len: 1},
            Token{t: TokenType::Integer(1), line: 5, pos: 6, len: 1},
            Token{t: TokenType::EOL, line: 5, pos: 7, len: 1},

            Token{t: TokenType::MEASURE, line: 6, pos: 1, len: 7},
            Token{t: TokenType::Integer(0), line: 6, pos: 9, len: 1},
            Token{t: TokenType::EOL, line: 6, pos: 10, len: 1},

            Token{t: TokenType::MEASURE, line: 7, pos: 1, len: 7},
            Token{t: TokenType::Integer(1), line: 7, pos: 9, len: 1},
            Token{t: TokenType::EOL, line: 7, pos: 10, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_2.quil");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }

    #[test]
    fn tokenize_sample_3() {
        let expected_tokens = vec![
            Token{t: TokenType::RX, line: 1, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 1, pos: 3, len: 1},
            Token{t: TokenType::Float(0.45), line: 1, pos: 4, len: 4},
            Token{t: TokenType::RParen, line: 1, pos: 8, len: 1},
            Token{t: TokenType::Integer(0), line: 1, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 1, pos: 11, len: 1},

            Token{t: TokenType::RZ, line: 2, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 2, pos: 3, len: 1},
            Token{t: TokenType::Negative, line: 2, pos: 4, len: 1},
            Token{t: TokenType::Float(1.0), line: 2, pos: 5, len: 3},
            Token{t: TokenType::RParen, line: 2, pos: 8, len: 1},
            Token{t: TokenType::Integer(0), line: 2, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 2, pos: 11, len: 1},

            Token{t: TokenType::RZ, line: 3, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 3, pos: 3, len: 1},
            Token{t: TokenType::Float(1.0), line: 3, pos: 4, len: 3},
            Token{t: TokenType::RParen, line: 3, pos: 7, len: 1},
            Token{t: TokenType::Integer(1), line: 3, pos: 9, len: 1},
            Token{t: TokenType::EOL, line: 3, pos: 10, len: 1},

            Token{t: TokenType::RX, line: 4, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 4, pos: 3, len: 1},
            Token{t: TokenType::Float(0.45), line: 4, pos: 4, len: 4},
            Token{t: TokenType::RParen, line: 4, pos: 8, len: 1},
            Token{t: TokenType::Integer(1), line: 4, pos: 10, len: 1},
            Token{t: TokenType::EOL, line: 4, pos: 11, len: 1},
            
            Token{t: TokenType::CZ, line: 5, pos: 1, len: 2},
            Token{t: TokenType::Integer(1), line: 5, pos: 4, len: 1},
            Token{t: TokenType::Integer(0), line: 5, pos: 6, len: 1},
            Token{t: TokenType::EOL, line: 5, pos: 7, len: 1},

            Token{t: TokenType::RZ, line: 6, pos: 1, len: 2},
            Token{t: TokenType::LParen, line: 6, pos: 3, len: 1},
            Token{t: TokenType::Float(1.5707963267948966), line: 6, pos: 4, len: 18},
            Token{t: TokenType::RParen, line: 6, pos: 22, len: 1},
            Token{t: TokenType::Integer(1), line: 6, pos: 24, len: 1},
            Token{t: TokenType::EOL, line: 6, pos: 25, len: 1},
        ];

        let test_filename = format!("{TESTDATA_DIR}/sample_3.quil");
        let actual_tokens = tokenize(&test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }
}
