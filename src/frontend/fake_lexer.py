import re

TOKEN_SPEC = [
    ("NEWLINE", r"\n"),
    ("SKIP", r"[ \t]+"),
    ("NUMBER", r"\d+(\.\d+)?"),
    ("STRING", r"\"([^\"\\]|\\.)*\""),
    ("LBRACE", r"\{"),
    ("RBRACE", r"\}"),
    ("LBRACK", r"\["),
    ("RBRACK", r"\]"),
    ("LPAREN", r"\("),
    ("RPAREN", r"\)"),
    ("COLON", r":"),
    ("COMMA", r","),
    ("ARROW", r"=>"),
    ("OP", r"ADD|SUB|MUL|DIV|MOD|EQ|NEQ|GT|LT|GTE|LTE|AND|OR|NOT|AS|IS|PIPE"),
    ("KEYWORD", r"MODULE|IMPORT|TYPE|ENUM|STRUCT|UNION|TRAIT|IMPL|FUNC|LET|CONST|RETURN|IF|ELSE|MATCH|CASE|OTHERWISE|LOOP|SPAWN|SEND|RECV|AWAIT|TRY|CATCH|FINALLY"),
    ("IDENT", r"[A-Z][A-Z0-9_]*"),
    ("LOWTYPE", r"int|float|string|bool|none"),
    ("FN", r"fn"),
    ("LOWNAME", r"[a-z_][a-z0-9_]*"),
]

MASTER = re.compile("|".join(f"(?P<{n}>{r})" for n, r in TOKEN_SPEC))

def lex(src: str):
    tokens = []
    line = 1
    col = 1
    for m in MASTER.finditer(src):
        kind = m.lastgroup
        val = m.group()
        if kind == "SKIP":
            col += len(val)
            continue
        if kind == "NEWLINE":
            tokens.append(("NEWLINE", val, line, col))
            line += 1
            col = 1
            continue
        tokens.append((kind, val, line, col))
        col += len(val)
    tokens.append(("EOF", "", line, col))
    return tokens
