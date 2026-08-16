import re

TOKEN_REGEX = [
    ("NUMBER", r"\d+(\.\d+)?"),
    ("STRING", r"\".*?\""),
    ("IDENT", r"[A-Z_][A-Z0-9_]*"),
    ("NAME", r"[a-zA-Z_][a-zA-Z0-9_]*"),
    ("SYMBOL", r"[\(\)\{\}

\[\]

,:]"),
    ("OP", r"ADD|SUB|MUL|DIV|MOD|EQ|NEQ|GT|LT|GTE|LTE|AND|OR|NOT"),
    ("NEWLINE", r"\n"),
    ("SKIP", r"[ \t]+"),
]

MASTER = re.compile("|".join(f"(?P<{name}>{regex})" for name, regex in TOKEN_REGEX))

def lex(source: str):
    tokens = []
    for match in MASTER.finditer(source):
        kind = match.lastgroup
        value = match.group()
        if kind == "SKIP":
            continue
        tokens.append((kind, value))
    return tokens
