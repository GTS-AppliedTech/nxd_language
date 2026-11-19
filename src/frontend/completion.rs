use crate::diagnostics::parser::parse_with_diagnostics;
use crate::diagnostics::validation::build_ir;
use crate::frontend::ast::nodes::{AstExpr, AstNode, AstStatement};
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

fn object_initializer_type_at_position(source: &str, line: u32, column: u32) -> Option<String> {
    let cursor_offset: usize = source
        .lines()
        .take(line.saturating_sub(1) as usize)
        .map(|line| line.len() + 1)
        .sum::<usize>()
        + source
            .lines()
            .nth(line.saturating_sub(1) as usize)?
            .chars()
            .take(column.saturating_sub(1) as usize)
            .map(char::len_utf8)
            .sum::<usize>();

    let before_cursor = &source[..cursor_offset];

    let open_brace = before_cursor.rfind('{')?;

    let prefix = before_cursor[..open_brace].trim_end();

    let type_name = prefix
        .chars()
        .rev()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    if type_name.is_empty() {
        None
    } else {
        Some(type_name)
    }
}

fn object_initializer_fields(
    functions: &[crate::frontend::ast::nodes::AstFunction],
    type_name: &str,
) -> Option<Vec<String>> {
    for function in functions {
        for statement in &function.body {
            let value = match statement {
                AstStatement::Let(stmt) => &stmt.value,
                AstStatement::Const(stmt) => &stmt.value,
                _ => continue,
            };

            let AstExpr::ObjectInit(init) = value else {
                continue;
            };

            if init.type_name != type_name {
                continue;
            }

            let mut fields: Vec<String> = init.fields.keys().cloned().collect();

            fields.sort();

            return Some(fields);
        }
    }

    None
}

pub fn object_initializer_completion_words(
    source: &str,
    line: u32,
    column: u32,
) -> Option<Vec<String>> {
    let type_name = object_initializer_type_at_position(source, line, column)?;

    let source_line = source.lines().nth(line.saturating_sub(1) as usize)?;

    let cursor = column.saturating_sub(1) as usize;

    let cursor_byte_offset: usize = source
        .lines()
        .take(line.saturating_sub(1) as usize)
        .map(|line| line.len() + 1)
        .sum::<usize>()
        + source_line
            .chars()
            .take(cursor)
            .map(char::len_utf8)
            .sum::<usize>();

    let mut completion_source = source.to_string();

    let line_prefix: String = source_line.chars().take(cursor).collect();

    let partial_field_length = line_prefix
        .chars()
        .rev()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .map(char::len_utf8)
        .sum::<usize>();

    if partial_field_length > 0 {
        let partial_field_start = cursor_byte_offset.checked_sub(partial_field_length)?;

        completion_source.replace_range(partial_field_start..cursor_byte_offset, "");
    }

    let (_, ast_types, functions, _) = parse_with_diagnostics(&completion_source).ok()?;

    let present_fields = object_initializer_fields(&functions, &type_name)?;

    let struct_decl = ast_types.iter().find_map(|node| {
        let AstNode::Struct(struct_decl) = node else {
            return None;
        };

        if struct_decl.name == type_name {
            Some(struct_decl)
        } else {
            None
        }
    })?;

    let mut remaining: Vec<String> = struct_decl
        .fields
        .iter()
        .map(|field| field.name.clone())
        .filter(|field| !present_fields.contains(field))
        .collect();

    remaining.sort();

    Some(remaining)
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

        let target = member_target_at_position(source, line, column);

        assert_eq!(target, Some("P".to_string()));

        let words = member_completion_words(source, line, column);

        assert_eq!(words, Some(vec!["AGE".to_string(), "NAME".to_string(),]));
    }

    #[test]
    fn detects_object_initializer_type() {
        let source = r#"
    MODULE TEST

    TYPE PERSON STRUCT:
        NAME: string
        AGE: int

    FUNC MAIN():
        LET P SET PERSON {
            NAME: "gabriel",

        }
    "#;

        let line = source
            .lines()
            .position(|line| line.contains("NAME: \"gabriel\""))
            .unwrap() as u32
            + 2;

        let column = 1;

        let type_name = object_initializer_type_at_position(source, line, column);

        assert_eq!(type_name, Some("PERSON".to_string()));
    }

    #[test]
    fn completes_remaining_object_initializer_fields() {
        let source = r#"
    MODULE TEST

    TYPE PERSON STRUCT:
        NAME: string
        AGE: int

    FUNC MAIN():
        LET P SET PERSON {
            NAME: "gabriel",

        }
    "#;

        let line = source
            .lines()
            .position(|line| line.contains("NAME: \"gabriel\""))
            .unwrap() as u32
            + 2;

        let column = 1;

        let words = object_initializer_completion_words(source, line, column);

        assert_eq!(words, Some(vec!["AGE".to_string()]));
    }
}
