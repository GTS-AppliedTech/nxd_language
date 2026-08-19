mod orchestrator;
mod ir;
mod backend;
mod semantic;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage:");
        eprintln!("  cargo run <input_json> <output_nim>");
        eprintln!("  cargo run -- --semantics <input_json> <output_nim>");
        std::process::exit(1);
    }

    let use_semantics = args.len() == 4 && args[1] == "--semantics";

    let input_json;
    let output_nim;

    if use_semantics {
        input_json = &args[2];
        output_nim = &args[3];
    } else {
        input_json = &args[1];
        output_nim = &args[2];
    }

    let result = if use_semantics {
        orchestrator::compile_from_ir_json_with_semantics(input_json)
    } else {
        orchestrator::compile_from_ir_json(input_json)
    };

    match result {
        Ok(nim_code) => {
            fs::write(output_nim, nim_code)
                .expect("Failed to write output file");

            if use_semantics {
                println!("Semantic validation successful");
            }

            println!("Compilation successful");
        }

        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            std::process::exit(1);
        }
    }
}