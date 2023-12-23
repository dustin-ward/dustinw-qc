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

#[derive(Debug, PartialEq)]
pub struct Token {
    t: TokenType,
    line: u32,
    pos: u32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn tokenize(filename: &str) -> Result<Vec<Token>, String> {
    match read_lines(filename) {
        Ok(lines) => {
            let mut tokens: Vec<Token> = Vec::new();

            for (lineno, line_result) in lines.enumerate() {
                let line = line_result.unwrap();

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
                    };

                    new_token.t = match ch {
                        ch if ch.is_alphabetic() => {
                            let mut ident = String::from(ch.to_string());

                            while let Some((_, peek_ch)) = iter.peek() {
                                if !peek_ch.is_alphabetic() {
                                    break;
                                }

                                let (_, next_ch) = iter.next().unwrap();
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

                        '(' => TokenType::LParen,
                        ')' => TokenType::RParen,
                        '-' => TokenType::Negative,
                        ';' => TokenType::EOL,
                        _ => TokenType::UNDEF,
                    };

                    if new_token.t == TokenType::UNDEF {
                        return Err(format!("Undefined token at {}:{} \"{}\"", lineno+1, pos+1, &line[pos..pos+1]));
                    }

                    tokens.push(new_token);
                }

                if let Some(last_token) = tokens.last() {
                    if last_token.t != TokenType::EOL {
                        tokens.push(Token{
                            t: TokenType::EOL,
                            line: lineno as u32 + 1,
                            pos: line.len() as u32 + 1,
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

    #[test]
    fn get_numeric_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::Float(1.234), line: 1, pos: 1},
            Token{t: TokenType::EOL, line: 1, pos: 6},
            Token{t: TokenType::Integer(0), line: 2, pos: 1},
            Token{t: TokenType::EOL, line: 2, pos: 2},
            Token{t: TokenType::Float(12345.6789), line: 3, pos: 1},
            Token{t: TokenType::EOL, line: 3, pos: 11},
            Token{t: TokenType::Float(1.), line: 4, pos: 1},
            Token{t: TokenType::EOL, line: 4, pos: 3},
            Token{t: TokenType::Float(0.0), line: 5, pos: 1},
            Token{t: TokenType::EOL, line: 5, pos: 4},
            Token{t: TokenType::Integer(1), line: 6, pos: 1},
            Token{t: TokenType::EOL, line: 6, pos: 2},
            Token{t: TokenType::Integer(2000000000), line: 7, pos: 1},
            Token{t: TokenType::EOL, line: 7, pos: 11},
            Token{t: TokenType::Float(0.0001), line: 8, pos: 1},
            Token{t: TokenType::EOL, line: 8, pos: 7},
            Token{t: TokenType::Integer(12345), line: 9, pos: 1},
            Token{t: TokenType::EOL, line: 9, pos: 6},
        ];

        let test_filename = "examples/testdata/get_numeric_tokens.testdata";
        let actual_tokens = tokenize(test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }

    #[test]
    fn get_function_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::RX, line: 1, pos: 1},
            Token{t: TokenType::EOL, line: 1, pos: 3},
            Token{t: TokenType::RZ, line: 2, pos: 1},
            Token{t: TokenType::EOL, line: 2, pos: 3},
            Token{t: TokenType::CZ, line: 3, pos: 1},
            Token{t: TokenType::EOL, line: 3, pos: 3},
            Token{t: TokenType::MEASURE, line: 4, pos: 1},
            Token{t: TokenType::EOL, line: 4, pos: 8},
        ];

        let test_filename = "examples/testdata/get_function_tokens.testdata";
        let actual_tokens = tokenize(test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }
    
    #[test]
    fn get_misc_tokens() {
        let expected_tokens = vec![
            Token{t: TokenType::LParen, line: 1, pos: 1},
            Token{t: TokenType::EOL, line: 1, pos: 2},
            Token{t: TokenType::RParen, line: 1, pos: 3},
            Token{t: TokenType::EOL, line: 1, pos: 4},
            Token{t: TokenType::Negative, line: 2, pos: 1},
            Token{t: TokenType::LParen, line: 2, pos: 2},
            Token{t: TokenType::EOL, line: 2, pos: 3},
            Token{t: TokenType::EOL, line: 3, pos: 1},
            Token{t: TokenType::RParen, line: 3, pos: 2},
            Token{t: TokenType::EOL, line: 3, pos: 3},
        ];

        let test_filename = "examples/testdata/get_misc_tokens.testdata";
        let actual_tokens = tokenize(test_filename).unwrap();

        assert_eq!(expected_tokens.len(), actual_tokens.len());

        for (i, ex_token) in expected_tokens.iter().enumerate() {
            assert_eq!(ex_token, &actual_tokens[i]);
        }
    }
}
