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

    let mut completion_source = source.to_string();

    let source_line = source.lines().nth(line.saturating_sub(1) as usize)?;

    let cursor = column.saturating_sub(1) as usize;

    let byte_offset: usize = source
        .lines()
        .take(line.saturating_sub(1) as usize)
        .map(|line| line.len() + 1)
        .sum::<usize>()
        + source_line
            .chars()
            .take(cursor)
            .map(char::len_utf8)
            .sum::<usize>();

    if byte_offset == 0 || completion_source.as_bytes()[byte_offset - 1] != b'.' {
        return None;
    }

    completion_source.remove(byte_offset - 1);

    let (module, ast_types, functions, _) = parse_with_diagnostics(&completion_source).ok()?;

    let ir_root = build_ir(&module, &ast_types, &functions);

    let tooling = tooling_analysis_from_ir(&ir_root).ok()?;

    for function in tooling.functions.values() {
        println!(
            "function span: {}:{} -> {}:{}",
            function.span.line,
            function.span.column,
            function.span.end_line,
            function.span.end_column,
        );

        println!("variables: {:?}", function.variables);

        println!("types: {:?}", tooling.types);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completes_struct_members() {
        let source = r#"
MODULE TEST

TYPE PERSON STRUCT:
    NAME: string
    AGE: int

FUNC MAIN():
    LET P SET PERSON {
        NAME: "Gabriel",
        AGE: 42
    }

    P.
"#;

        let line = source.lines().position(|line| line.trim() == "P.").unwrap() as u32 + 1;

        let column = source
            .lines()
            .nth((line - 1) as usize)
            .unwrap()
            .chars()
            .count() as u32
            + 1;

        println!("position: {line}:{column}");

        let target = member_target_at_position(source, line, column);

        println!("target: {target:?}");

        let words = member_completion_words(source, line, column);

        assert_eq!(words, Some(vec!["AGE".to_string(), "NAME".to_string(),]));
    }
}
