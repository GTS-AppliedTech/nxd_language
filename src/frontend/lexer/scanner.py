import re

def lex(src: str):
    src = strip_line_comments(src)
    print("=== STRIPPED SOURCE ===")
    print(src)
    print("=======================")
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
        if kind == "MISMATCH":
            raise SyntaxError(
        f"Unexpected character {val!r} at line {line}, column {col}"
            )

        tokens.append((kind, val, line, col))
        col += len(val)

    tokens.append(("EOF", "", line, col))
    return tokens

def strip_line_comments(source: str) -> str:
    """
    Removes NXD // line comments while preserving:
    - newline characters
    - // inside string literals
    - escaped quotation marks inside strings
    """

    output: list[str] = []
    index = 0
    in_string = False
    escaped = False

    while index < len(source):
        char = source[index]

        if in_string:
            output.append(char)

            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False

            index += 1
            continue

        if char == '"':
            in_string = True
            output.append(char)
            index += 1
            continue

        if (
            char == "/"
            and index + 1 < len(source)
            and source[index + 1] == "/"
        ):
            # Ignore everything from // through the end of this line.
            index += 2

            while index < len(source) and source[index] not in "\r\n":
                index += 1

            # Do not consume the newline. The normal loop preserves it,
            # retaining useful line numbers for parser diagnostics.
            continue

        output.append(char)
        index += 1

    return "".join(output)

def tokenize(source):
    tokens = []
    position = 0

    for match in TOKEN_SPEC.finditer(source):
        if match.start() != position:
            invalid_text = source[position:match.start()]
            line = source.count("\n", 0, position) + 1
            last_newline = source.rfind("\n", 0, position)
            column = position + 1 if last_newline == -1 else position - last_newline

            raise SyntaxError(
                f"Unexpected character sequence {invalid_text!r} "
                f"at line {line}, column {column}"
            )

        kind = match.lastgroup
        value = match.group()

        if kind == "SKIP":
            pass
        elif kind == "NEWLINE":
            tokens.append((kind, value))
        elif kind == "MISMATCH":
            raise SyntaxError(
                f"Unexpected character {value!r}"
            )
        else:
            tokens.append((kind, value))

        position = match.end()

    if position != len(source):
        invalid_text = source[position:]
        line = source.count("\n", 0, position) + 1
        last_newline = source.rfind("\n", 0, position)
        column = position + 1 if last_newline == -1 else position - last_newline

        raise SyntaxError(
            f"Unexpected character sequence {invalid_text!r} "
            f"at line {line}, column {column}"
        )

    tokens.append(("EOF", ""))
    return tokens

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
    ("KEYWORD", r"MODULE|IMPORT|TYPE|ENUM|STRUCT|UNION|TRAIT|IMPL|FUNC|LET|CONST|RETURN|IF|ELSE|MATCH|CASE|OTHERWISE|LOOP|SPAWN|SEND|RECV|AWAIT|TRY|CATCH|FINALLY|SET|FOR"),
    ("OP", r"ADD|SUB|MUL|DIV|MOD|EQ|NEQ|GTE|LTE|GT|LT|AND|OR|NOT|AS|IS|PIPE|MOVE|CLONE|BORROW"),
    ("IDENT", r"[A-Z][A-Z0-9_]*"),
    ("LOWTYPE", r"int|float|string|bool"),
    ("FN", r"fn"),
    ("LOWNAME", r"[a-z_][a-z0-9_]*"),
    ("MISMATCH", r".")
]

MASTER = re.compile("|".join(f"(?P<{n}>{r})" for n, r in TOKEN_SPEC))

