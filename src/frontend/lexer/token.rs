use crate::diagnostics::span::SourceSpan;
use crate::frontend::lexer::token_kind::TokenKind;

impl Token {
    pub fn span(&self) -> SourceSpan {
        let width = self.lexeme.chars().count().max(1) as u32;

        SourceSpan {
            line: self.line,
            column: self.column,
            end_line: self.line,
            end_column: self.column + width,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}
