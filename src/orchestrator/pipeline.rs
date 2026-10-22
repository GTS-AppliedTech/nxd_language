use crate::backend::nim::emitter;
use crate::diagnostics::diagnostics::{SemanticDiagnostic, SemanticDiagnosticResponse};
use crate::diagnostics::errors::SemanticError;
use crate::ir::nodes::IRRoot;
use crate::semantic::analyzer::Analyzer;
use crate::semantic::symbols::Symbol;
use crate::semantic::traits::TraitRegistry;

fn analyze_ir(ir_root: &IRRoot) -> Result<(), SemanticError> {
    let traits = TraitRegistry::new();

    let mut analyzer = Analyzer::new(traits);

    analyzer.analyze(ir_root)
}

fn analyze_ir_with_symbols(ir_root: &IRRoot) -> Result<Analyzer, SemanticError> {
    let traits = TraitRegistry::new();

    let mut analyzer = Analyzer::new(traits);

    analyzer.analyze(ir_root)?;

    Ok(analyzer)
}

pub fn definition_from_ir(ir_root: &IRRoot, symbol: &str) -> Result<String, String> {
    let analyzer = analyze_ir_with_symbols(ir_root).map_err(|error| error.message())?;

    match analyzer.symbols.resolve(symbol) {
        Some(Symbol::Func {
            span: Some(span), ..
        }) => Ok(format!(
            "{}:{}:{}:{}",
            span.line, span.column, span.end_line, span.end_column,
        )),

        Some(_) => Err(format!("'{}' is not a function definition", symbol,)),

        None => Err(format!("Definition not found for '{}'", symbol,)),
    }
}

pub fn compile_from_ir(ir_root: &IRRoot) -> Result<String, String> {
    let nim_code = emitter::emit(ir_root);

    Ok(nim_code)
}

pub fn compile_from_ir_with_semantics(ir_root: &IRRoot) -> Result<String, String> {
    analyze_ir(ir_root).map_err(|error| format!("Semantic error: {}", error.message(),))?;

    let nim_code = emitter::emit(ir_root);

    Ok(nim_code)
}

pub fn semantic_diagnostics_from_ir(ir_root: &IRRoot) -> SemanticDiagnosticResponse {
    match analyze_ir(ir_root) {
        Ok(()) => SemanticDiagnosticResponse {
            ok: true,
            diagnostics: Vec::new(),
        },

        Err(error) => SemanticDiagnosticResponse {
            ok: false,

            diagnostics: vec![SemanticDiagnostic {
                code: error.code().to_string(),

                message: error.message(),

                line: error.line() as usize,

                column: error.column() as usize,

                end_line: error.end_line() as usize,

                end_column: error.end_column() as usize,

                severity: "error".to_string(),
            }],
        },
    }
}
