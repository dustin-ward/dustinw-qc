use std::env;
use std::process;

pub mod lexer;
pub mod parser;

fn main() {
    // Parse Args
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("No filename provided.\nUsage: '{} filename.quil'", args[0]);
        process::exit(1)
    }
    let filename = &args[1];

    // Tokenize File
    let tokens_result = lexer::tokenize(filename);
    if let Err(err) = tokens_result {
        println!("lexer: {}", err);
        process::exit(1);
    }
    let tokens = tokens_result.as_ref().unwrap();

    // Parse Tokens
    let program_result = parser::parse(tokens);
    if let Err(err) = tokens_result {
        println!("parser: {}", err);
        process::exit(1);
    }
    let program: Vec<parser::Instruction> = program_result.unwrap();


    dbg!(program);
}
