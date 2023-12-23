use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum TokenType {
    UNDEF,

    Float(f64),
    QIdx(u32),

    LParen,
    RParen,
    Negative,
    EOF,

    RX,
    RZ,
    CZ,
    MEASURE,
}

#[derive(Debug)]
struct Token {
    t: TokenType,
    line: u32,
    pos: u32,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    // Parse Args
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("No filename provided.\nUsage: '{} filename.quil'", args[0]);
        process::exit(1)
    }
    let filename = &args[1];

    // Tokenize File
    let mut tokens_result = tokenize(filename);
    if let Err(err) = tokens_result {
        println!("lexer: {}", err);
        process::exit(1);
    }
    let mut tokens: Vec<Token> = tokens_result.unwrap();


    dbg!(tokens);
}
