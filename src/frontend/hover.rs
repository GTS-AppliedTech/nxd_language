use crate::diagnostics::parser::parse_with_diagnostics;
use crate::diagnostics::validation::build_ir;
use crate::frontend::lexer::token_kind::{TOKEN_SPEC, TokenKind};
use crate::orchestrator::tooling_analysis_from_ir;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInfo {
    pub label: String,
    pub detail: String,
}

fn number_at_position(source: &str, line: u32, column: u32) -> Option<String> {
    let source_line = source.lines().nth(line.saturating_sub(1) as usize)?;

    let characters: Vec<char> = source_line.chars().collect();

    if characters.is_empty() {
        return None;
    }

    let mut cursor = column.saturating_sub(1) as usize;

    if cursor >= characters.len() {
        cursor = characters.len().saturating_sub(1);
    }

    if !characters[cursor].is_ascii_digit() && characters[cursor] != '.' {
        return None;
    }

    let mut start = cursor;

    while start > 0 && (characters[start - 1].is_ascii_digit() || characters[start - 1] == '.') {
        start -= 1;
    }

    let mut end = cursor + 1;

    while end < characters.len() && (characters[end].is_ascii_digit() || characters[end] == '.') {
        end += 1;
    }

    let number: String = characters[start..end].iter().collect();

    if number.parse::<i64>().is_ok() || number.parse::<f64>().is_ok() {
        Some(number)
    } else {
        None
    }
}

pub fn hover_at(source: &str, line: u32, column: u32) -> Option<HoverInfo> {
    if let Some(number) = number_at_position(source, line, column) {
        if number.contains('.') {
            return Some(HoverInfo {
                label: number,
                detail: "NXD floating-point literal\n\ntype: float".to_string(),
            });
        }

        return Some(HoverInfo {
            label: number,
            detail: "NXD integer literal\n\ntype: int".to_string(),
        });
    }

    let word = word_at_position(source, line, column)?;

    if word.parse::<i64>().is_ok() {
        return Some(HoverInfo {
            label: word.to_string(),
            detail: "NXD integer literal\n\ntype: int".to_string(),
        });
    }

    if word.parse::<f64>().is_ok() {
        return Some(HoverInfo {
            label: word.to_string(),
            detail: "NXD floating-point literal\n\ntype: float".to_string(),
        });
    }

    if let Ok((module, ast_types, functions, _)) = parse_with_diagnostics(source) {
        let ir_root = build_ir(&module, &ast_types, &functions);

        let tooling = tooling_analysis_from_ir(&ir_root).ok();

        if let Some(tooling) = &tooling {
            for function in tooling.functions.values() {
                let inside_function = line >= function.span.line && line <= function.span.end_line;

                if !inside_function {
                    continue;
                }

                if let Some(variable) = function.variables.get(&word) {
                    return Some(HoverInfo {
                        label: word.to_string(),
                        detail: format!("NXD variable\n\n{}: {}", word, variable.ty,),
                    });
                }

                if let Some(constant) = function.constants.get(&word) {
                    return Some(HoverInfo {
                        label: word.to_string(),
                        detail: format!("NXD constant\n\n{}: {}", word, constant.ty,),
                    });
                }
            }
        }

        for function in &functions {
            if function.name == word {
                let params = function
                    .params
                    .iter()
                    .map(|param| format!("{}: {}", param.name, param.ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                let return_type = function.return_type.as_deref().unwrap_or("none");

                let kind = if function.is_async {
                    "NXD async function"
                } else {
                    "NXD function"
                };

                return Some(HoverInfo {
                    label: function.name.clone(),
                    detail: format!(
                        "{}\n\n{}({}) -> {}",
                        kind, function.name, params, return_type,
                    ),
                });
            }

            for param in &function.params {
                if param.name == word {
                    return Some(HoverInfo {
                        label: param.name.clone(),
                        detail: format!("NXD parameter\n\n{}: {}", param.name, param.ty,),
                    });
                }
            }
        }
    }

    if word == "PRINTLN" {
        return Some(HoverInfo {
            label: "PRINTLN".to_string(),
            detail: "Built-in function\n\nPRINTLN(value: any) -> none".to_string(),
        });
    }

    match word.as_str() {
        "true" | "false" => {
            return Some(HoverInfo {
                label: word.to_string(),
                detail: "NXD boolean literal".to_string(),
            });
        }

        "none" => {
            return Some(HoverInfo {
                label: word.to_string(),
                detail: "NXD none literal".to_string(),
            });
        }

        _ => {}
    }

    for rule in TOKEN_SPEC {
        if rule.kind == TokenKind::LowType {
            if rule.pattern.split('|').any(|candidate| candidate == word) {
                return Some(HoverInfo {
                    label: word.to_string(),
                    detail: "NXD primitive type".to_string(),
                });
            }
        }

        if matches!(rule.kind, TokenKind::Keyword | TokenKind::Op)
            && rule.pattern.split('|').any(|candidate| candidate == word)
        {
            let kind = match rule.kind {
                TokenKind::Keyword => "NXD keyword",
                TokenKind::Op => "NXD operator",
                _ => unreachable!(),
            };

            return Some(HoverInfo {
                label: word.to_string(),
                detail: kind.to_string(),
            });
        }
    }

    None
}

fn word_at_position(source: &str, line: u32, column: u32) -> Option<String> {
    let source_line = source.lines().nth(line.saturating_sub(1) as usize)?;
    let characters: Vec<char> = source_line.chars().collect();

    if characters.is_empty() {
        return None;
    }

    let mut cursor = column.saturating_sub(1) as usize;

    if cursor >= characters.len() {
        cursor = characters.len().saturating_sub(1);
    }

    if !is_identifier_character(characters[cursor])
        && cursor > 0
        && is_identifier_character(characters[cursor - 1])
    {
        cursor -= 1;
    }

    if !is_identifier_character(characters[cursor]) {
        return None;
    }

    let mut start = cursor;

    while start > 0 && is_identifier_character(characters[start - 1]) {
        start -= 1;
    }

    let mut end = cursor + 1;

    while end < characters.len() && is_identifier_character(characters[end]) {
        end += 1;
    }

    Some(characters[start..end].iter().collect())
}

fn is_identifier_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}
