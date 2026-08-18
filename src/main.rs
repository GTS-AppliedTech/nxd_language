mod orchestrator;
mod ir;
mod backend;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <input_json> <output_nim>");
        std::process::exit(1);
    }

    let input_json = &args[1];
    let output_nim = &args[2];

    match orchestrator::compile_from_ir_json(input_json) {
        Ok(nim_code) => {
            fs::write(output_nim, nim_code)
                .expect("Failed to write output file");

            println!("Compilation successful");
        }
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            std::process::exit(1);        
        }
    }
}