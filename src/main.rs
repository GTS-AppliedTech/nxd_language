mod backend;
mod diagnostics;
mod frontend;
mod ir;
mod orchestrator;
mod semantic;

use crate::ir::compiler::compile_to_ir;
use crate::ir::nodes::IRRoot;

use std::env;
use std::fs;

fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|error| format!("Failed to read source file '{}': {}", path, error,))
}

fn build_ir(path: &str) -> Result<IRRoot, String> {
    let source = read_source(path)?;

    compile_to_ir(&source).map_err(|error| format!("Failed to compile source to IR: {}", error,))
}

fn write_ir_json(ir_root: &IRRoot, output_path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(ir_root)
        .map_err(|error| format!("Failed to serialize IR JSON: {}", error,))?;

    fs::write(output_path, json)
        .map_err(|error| format!("Failed to write IR JSON file '{}': {}", output_path, error,))
}

fn print_semantic_diagnostics(ir_root: &IRRoot) -> Result<(), String> {
    println!();
    println!("=== SEMANTIC DIAGNOSTICS ===");

    let response = orchestrator::semantic_diagnostics_from_ir(ir_root);

    if response.ok {
        println!("No semantic errors.");

        return Ok(());
    }

    for diagnostic in &response.diagnostics {
        eprintln!("{}: {}", diagnostic.code, diagnostic.message,);

        eprintln!(
            "  at {}:{}-{}:{} [{}]",
            diagnostic.line,
            diagnostic.column,
            diagnostic.end_line,
            diagnostic.end_column,
            diagnostic.severity,
        );
    }

    Err(format!(
        "Semantic validation failed with {} diagnostic(s)",
        response.diagnostics.len(),
    ))
}

fn write_nim(ir_root: &IRRoot, output_path: &str) -> Result<(), String> {
    println!();
    println!("=== NIM EMISSION ===");

    /*
     * Semantics were already checked by
     * print_semantic_diagnostics().
     *
     * Use compile_from_ir() here so semantics are not run
     * a second time.
     */
    let nim_code = orchestrator::compile_from_ir(ir_root)?;

    fs::write(output_path, nim_code).map_err(|error| {
        format!(
            "Failed to write Nim output file '{}': {}",
            output_path, error,
        )
    })?;

    println!("Nim output written to '{}'.", output_path,);

    Ok(())
}

fn print_usage() {
    eprintln!("Usage:");

    eprintln!(
        "  cargo run -- \
<input.nxd> <output.json> <output.nim>"
    );

    eprintln!(
        "  cargo run -- \
--semantic-diagnostics <input.nxd>"
    );

    eprintln!(
        "  cargo run -- \
--definition <input.nxd> <symbol>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
     * Diagnostic-only command:
     *
     * cargo run -- \
     *     --semantic-diagnostics \
     *     input.nxd
     */
    if args.len() == 3 && args[1] == "--semantic-diagnostics" {
        let ir_root = match build_ir(&args[2]) {
            Ok(ir_root) => ir_root,

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        if let Err(error) = print_semantic_diagnostics(&ir_root) {
            eprintln!("{}", error);
            std::process::exit(1);
        }

        return;
    }

    /*
     * Definition lookup:
     *
     * cargo run -- \
     *     --definition \
     *     input.nxd \
     *     SYMBOL
     */
    if args.len() == 4 && args[1] == "--definition" {
        let ir_root = match build_ir(&args[2]) {
            Ok(ir_root) => ir_root,

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        };

        let definition = orchestrator::definition_from_ir(&ir_root, &args[3]);

        match definition {
            Ok(result) => {
                println!("{}", result);
            }

            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }

        return;
    }

    /*
     * Complete compiler pipeline:
     *
     * cargo run -- \
     *     input.nxd \
     *     output.json \
     *     output.nim
     */
    if args.len() != 4 {
        print_usage();
        std::process::exit(1);
    }

    let input_path = &args[1];

    let json_output_path = &args[2];

    let nim_output_path = &args[3];

    println!();
    println!("=== FRONTEND ===");

    /*
     * This is the only build_ir() call.
     *
     * Therefore:
     *
     * - source is read once
     * - stripped source is printed once
     * - parser runs once
     * - AST is printed once
     * - warnings are printed once
     * - lowering runs once
     */
    let ir_root = match build_ir(input_path) {
        Ok(ir_root) => ir_root,

        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    println!();
    println!("=== IR JSON OUTPUT ===");

    if let Err(error) = write_ir_json(&ir_root, json_output_path) {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    println!("IR JSON written to '{}'.", json_output_path,);

    /*
     * Run semantic analysis exactly once and expose the
     * diagnostic details directly in the terminal.
     */
    if let Err(error) = print_semantic_diagnostics(&ir_root) {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    /*
     * Emit Nim from the same IRRoot.
     */
    if let Err(error) = write_nim(&ir_root, nim_output_path) {
        eprintln!("Compilation failed: {}", error,);

        std::process::exit(1);
    }

    println!();
    println!("=== COMPLETE ===");
    println!("IR JSON generation successful");
    println!("Semantic validation successful");
    println!("Nim compilation successful");
}
