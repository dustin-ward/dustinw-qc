use std::env;
use std::process;

use dustinw_qc::lexer;
use dustinw_qc::parser;
use dustinw_qc::parser::Instruction;

use dustinw_qc::optimize::cz_cancel;
use dustinw_qc::optimize::deadcode;
use dustinw_qc::optimize::native_translation;
use dustinw_qc::optimize::reorder;
use dustinw_qc::optimize::rotation_merge;

const MAX_OP_PASSES: u32 = 100;

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

    // Translate program to native instructions only
    let translation_result = native_translation::native_translation_pass(program);
    if let Err(err) = translation_result {
        println!("native instruction translation: {}", err);
        process::exit(1);
    }
    program = translation_result.unwrap();

    // Code passes
    let code_passes: Vec<(
        &str,
        fn(Vec<Instruction>) -> Result<Vec<Instruction>, String>,
    )> = vec![
        ("reorder", reorder::reorder_pass),
        ("rotation_merge", rotation_merge::rotation_merge_pass),
        ("cz_cancel", cz_cancel::cz_cancel_pass),
        ("deadcode", deadcode::deadcode_pass),
    ];

    let mut prog_len = program.len();
    for _round in 0..MAX_OP_PASSES {
        // Perform optimization passes
        for (name, pass_func) in &code_passes {
            match pass_func(program) {
                Ok(new_prog) => program = new_prog,
                Err(err) => {
                    println!("{}: {}", name, err);
                    process::exit(1);
                }
            }
        }

        // Terminate optimization if program length is not changing
        if program.len() == prog_len {
            break;
        } else {
            prog_len = program.len();
        }
    }

    dbg!(program);
}
