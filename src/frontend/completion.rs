use crate::frontend::lexer::token_kind::{TOKEN_SPEC, TokenKind};

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
