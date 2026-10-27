#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Newline,
    Skip,
    Number,
    String,
    LBrace,
    RBrace,
    Eof,
    LBrack,
    RBrack,
    LParen,
    RParen,
    Colon,
    Comma,
    Arrow,
    Keyword,
    Op,
    Ident,
    LowType,
    Fn,
    LowName,
    Mismatch,
}

pub struct TokenRule {
    pub kind: TokenKind,
    pub pattern: &'static str,
}

pub const TOKEN_SPEC: &[TokenRule] = &[
    TokenRule {
        kind: TokenKind::Newline,
        pattern: r"\r\n|\r|\n",
    },
    // TODO: Verify string regex translation
    TokenRule {
        kind: TokenKind::String,
        pattern: r#"\"([^\"\\]|\\.)*\""#,
    },
    TokenRule {
        kind: TokenKind::Number,
        pattern: r"\d+(\.\d+)?",
    },
    TokenRule {
        kind: TokenKind::Skip,
        pattern: r"[ \t]+",
    },
    TokenRule {
        kind: TokenKind::LBrace,
        pattern: r"\{",
    },
    TokenRule {
        kind: TokenKind::RBrace,
        pattern: r"\}",
    },
    TokenRule {
        kind: TokenKind::LBrack,
        pattern: r"\[",
    },
    TokenRule {
        kind: TokenKind::RBrack,
        pattern: r"\]",
    },
    TokenRule {
        kind: TokenKind::LParen,
        pattern: r"\(",
    },
    TokenRule {
        kind: TokenKind::RParen,
        pattern: r"\)",
    },
    TokenRule {
        kind: TokenKind::Colon,
        pattern: r":",
    },
    TokenRule {
        kind: TokenKind::Comma,
        pattern: r",",
    },
    TokenRule {
        kind: TokenKind::Arrow,
        pattern: r"=>",
    },
    TokenRule {
        kind: TokenKind::Keyword,
        pattern: r"MODULE|IMPORT|TYPE|ENUM|STRUCT|UNION|TRAIT|IMPL|FUNC|LET|CONST|RETURN|IF|ELSE|MATCH|CASE|OTHERWISE|LOOP|ASYNC|SYNC|SPAWN|TASK|CHANNEL|SEND|RECV|AWAIT|PING|ESC|TIMEOUT|TRY|CATCH|FINALLY|SET|FOR|TO",
    },
    TokenRule {
        kind: TokenKind::Op,
        pattern: r"ADD|SUB|MUL|DIV|MOD|EQ|NEQ|GTE|LTE|GT|LT|AND|OR|NOT|AS|IS|PIPE|MOVE|CLONE|BORROW",
    },
    TokenRule {
        kind: TokenKind::Ident,
        pattern: r"[A-Z][A-Z0-9_]*",
    },
    TokenRule {
        kind: TokenKind::LowType,
        pattern: r"int|float|string|bool",
    },
    TokenRule {
        kind: TokenKind::Fn,
        pattern: r"fn",
    },
    TokenRule {
        kind: TokenKind::LowName,
        pattern: r"[a-z_][a-z0-9_]*",
    },
    TokenRule {
        kind: TokenKind::Mismatch,
        pattern: r".",
    },
];
