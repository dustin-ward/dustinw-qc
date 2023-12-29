use std::env;
use std::process;

use dustinw_qc::lexer;
use dustinw_qc::parser;
use dustinw_qc::parser::Instruction;

use dustinw_qc::optimize::deadcode;
use dustinw_qc::optimize::native_translation;

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
    let mut program: Vec<Instruction> = program_result.unwrap();

    // Code passes
    let code_passes: Vec<(
        &str,
        fn(Vec<Instruction>) -> Result<Vec<Instruction>, String>,
    )> = vec![
        (
            "native_translation",
            native_translation::native_translation_pass,
        ),
        ("deadcode", deadcode::deadcode_pass),
    ];
    for (name, pass_func) in code_passes {
        match pass_func(program) {
            Ok(new_prog) => program = new_prog,
            Err(err) => {
                println!("{}: {}", name, err);
                process::exit(1);
            }
        }
    }

    dbg!(program);
}
