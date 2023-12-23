use std::env;
use std::process;

pub mod lexer;

fn main() {
    // Parse Args
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("No filename provided.\nUsage: '{} filename.quil'", args[0]);
        process::exit(1)
    }
    let filename = &args[1];

    // Tokenize File
    let mut tokens_result = lexer::tokenize(filename);
    if let Err(err) = tokens_result {
        println!("lexer: {}", err);
        process::exit(1);
    }
    let mut tokens: Vec<lexer::Token> = tokens_result.unwrap();


    dbg!(tokens);
}
