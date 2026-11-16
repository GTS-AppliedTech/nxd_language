use crate::frontend::lexer::token_kind::{TOKEN_SPEC, TokenKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInfo {
    pub label: String,
    pub detail: String,
}

pub fn hover_at(source: &str, line: u32, column: u32) -> Option<HoverInfo> {
    let word = word_at_position(source, line, column)?;

    if word == "PRINTLN" {
        return Some(HoverInfo {
            label: "PRINTLN".to_string(),
            detail: "Built-in function\n\nPRINTLN(value: any) -> none".to_string(),
        });
    }

    for rule in TOKEN_SPEC {
        if !matches!(rule.kind, TokenKind::Keyword | TokenKind::Op) {
            continue;
        }

        if rule.pattern.split('|').any(|candidate| candidate == word) {
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
