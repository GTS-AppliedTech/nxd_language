import contextlib
import io
import json
import sys

from src.frontend.parser.parser import parse, ParserError
from src.frontend.warnings import find_unused_variables

source = sys.stdin.read()

try:
    # Suppress lexer/parser debug prints so stdout contains JSON only.
    with contextlib.redirect_stdout(io.StringIO()):
        module, ast_types, ast_functions = parse(source)

    warnings = find_unused_variables(module)
    diagnostics = []

    for warning in warnings:
        diagnostics.append(
            {
                "line": warning["line"],
                "column": warning["col"],
                "message": warning["message"],
                "severity": "warning"
            }
        )

    print(json.dumps({
        "ok": True,
        "diagnostics": diagnostics
    }))

except ParserError as error:
    print(json.dumps({
        "ok": False,
        "diagnostics": [
            {
                "line": error.line,
                "column": error.col,
                "message": error.message,
                "severity": "error"
            }
        ]
    }))