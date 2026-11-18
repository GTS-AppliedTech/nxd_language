use crate::diagnostics::parser::parse_with_diagnostics;
use crate::diagnostics::validation::build_ir;
use crate::frontend::lexer::token_kind::{TOKEN_SPEC, TokenKind};
use crate::orchestrator::tooling_analysis_from_ir;

pub fn completion_words() -> Vec<String> {
    let mut words = Vec::new();

    for rule in TOKEN_SPEC {
        if matches!(rule.kind, TokenKind::Keyword | TokenKind::Op) {
            words.extend(rule.pattern.split('|').map(str::to_string));
        }
    }

    words.push("PRINTLN".to_string());

    words
}

fn member_target_at_position(source: &str, line: u32, column: u32) -> Option<String> {
    let source_line = source.lines().nth(line.saturating_sub(1) as usize)?;

    let cursor = column.saturating_sub(1) as usize;

    let prefix: String = source_line.chars().take(cursor).collect();

    let prefix = prefix.trim_end();

    let target = prefix.strip_suffix('.')?;

    let target = target
        .chars()
        .rev()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    if target.is_empty() {
        None
    } else {
        Some(target)
    }
}

pub fn member_completion_words(source: &str, line: u32, column: u32) -> Option<Vec<String>> {
    let target = member_target_at_position(source, line, column)?;

    let (module, ast_types, functions, _) = parse_with_diagnostics(source).ok()?;

    let ir_root = build_ir(&module, &ast_types, &functions);

    let tooling = tooling_analysis_from_ir(&ir_root).ok()?;

    for function in tooling.functions.values() {
        let inside_function = line >= function.span.line && line <= function.span.end_line;

        if !inside_function {
            continue;
        }

        let symbol = function
            .variables
            .get(&target)
            .or_else(|| function.constants.get(&target))?;

        let fields = tooling.types.get(&symbol.ty)?;

        let mut words: Vec<String> = fields.keys().cloned().collect();

        words.sort();

        return Some(words);
    }

    None
}
