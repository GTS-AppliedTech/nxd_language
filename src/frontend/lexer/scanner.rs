use regex::Regex;

use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::{TOKEN_SPEC, TokenKind};

pub fn strip_line_comments(source: &str) -> String {
    // Removes NXD // line comments while preserving:
    // - newline characters
    // - // inside string literals
    // - escaped quotation marks inside strings

    let mut output = String::new();
    let chars: Vec<char> = source.chars().collect();

    let mut index = 0;
    let mut in_string = false;
    let mut escaped = false;

    while index < chars.len() {
        let ch = chars[index];

        if in_string {
            output.push(ch);

            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }

            index += 1;
            continue;
        }

        if ch == '"' {
            in_string = true;
            output.push(ch);
            index += 1;
            continue;
        }

        if ch == '/' && index + 1 < chars.len() && chars[index + 1] == '/' {
            // Skip both slash characters.

            index += 2;

            // Skip comment contents, but do not consume the newline.

            while index < chars.len() && chars[index] != '\r' && chars[index] != '\n' {
                index += 1;
            }

            continue;
        }

        output.push(ch);
        index += 1;
    }

    output
}

pub fn lex(src: &str) -> Result<Vec<Token>, String> {
    let src = strip_line_comments(src);

    eprintln!("=== STRIPPED SOURCE ===");
    eprintln!("{}", src);
    eprintln!("=======================");

    // Compile each TOKEN_SPEC entry as a regex that must match
    // at the beginning of the remaining source.

    let rules: Result<Vec<(TokenKind, Regex)>, String> = TOKEN_SPEC
        .iter()
        .map(|rule| {
            let anchored_pattern = format!(r"^(?:{})", rule.pattern);

            Regex::new(&anchored_pattern)
                .map(|regex| (rule.kind, regex))
                .map_err(|error| format!("Invalid lexer regex {:?}: {}", rule.pattern, error))
        })
        .collect();

    let rules = rules?;

    let mut tokens = Vec::new();

    let mut position = 0;
    let mut line = 1;
    let mut column = 1;

    while position < src.len() {
        let remaining_source = &src[position..];
        let mut matched_rule = None;

        for (kind, regex) in &rules {
            if let Some(regex_match) = regex.find(remaining_source) {
                let value = regex_match.as_str();

                matched_rule = Some((*kind, value));
                break;
            }
        }

        let Some((kind, value)) = matched_rule else {
            return Err(format!(
                "Unable to tokenize source at line {}, column {}",
                line, column
            ));
        };

        let character_count = value.chars().count();

        match kind {
            TokenKind::Skip => {
                column += character_count as u32;
            }

            TokenKind::Newline => {
                tokens.push(Token {
                    kind,
                    lexeme: value.to_string(),
                    line,
                    column,
                });

                line += 1;
                column = 1;
            }

            TokenKind::Mismatch => {
                return Err(format!(
                    "Unexpected character {:?} at line {}, column {}",
                    value, line, column
                ));
            }

            _ => {
                tokens.push(Token {
                    kind,
                    lexeme: value.to_string(),
                    line,
                    column,
                });

                column += character_count as u32;
            }
        }

        // Regex positions and string slices use byte offsets,
        // so advance by the matched string's byte length.

        position += value.len();
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        lexeme: String::new(),
        line,
        column,
    });

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_line_comments() {
        let source = "LET VALUE // remove this\nRETURN VALUE";

        let stripped = strip_line_comments(source);

        assert_eq!(stripped, "LET VALUE \nRETURN VALUE");
    }

    #[test]
    fn preserves_comment_markers_inside_strings() {
        let source = r#"LET MESSAGE "https://example.com" // remove this"#;

        let stripped = strip_line_comments(source);

        assert_eq!(stripped, r#"LET MESSAGE "https://example.com" "#);
    }

    #[test]
    fn preserves_escaped_quotes_inside_strings() {
        let source = r#"LET MESSAGE "She said \"hello // world\"" // remove"#;

        let stripped = strip_line_comments(source);

        assert_eq!(stripped, r#"LET MESSAGE "She said \"hello // world\"" "#);
    }

    #[test]
    fn lexes_basic_nxd_source() {
        let source = "LET VALUE 42\nRETURN VALUE";

        let tokens = lex(source).expect("Lexer should accept valid NXD source");

        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[0].lexeme, "LET");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);

        assert_eq!(tokens[1].kind, TokenKind::Ident);
        assert_eq!(tokens[1].lexeme, "VALUE");

        assert_eq!(tokens[2].kind, TokenKind::Number);
        assert_eq!(tokens[2].lexeme, "42");

        assert_eq!(tokens[3].kind, TokenKind::Newline);
        assert_eq!(tokens[3].line, 1);

        assert_eq!(tokens[4].kind, TokenKind::Keyword);
        assert_eq!(tokens[4].lexeme, "RETURN");
        assert_eq!(tokens[4].line, 2);
        assert_eq!(tokens[4].column, 1);

        assert_eq!(tokens[5].kind, TokenKind::Ident);
        assert_eq!(tokens[5].lexeme, "VALUE");

        assert_eq!(tokens[6].kind, TokenKind::Eof);
    }

    #[test]
    fn rejects_invalid_characters() {
        let error = lex("LET VALUE @").expect_err("Lexer should reject @");

        assert!(error.contains("Unexpected character"));
        assert!(error.contains("line 1"));
    }
}
