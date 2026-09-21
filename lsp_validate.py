import contextlib
import io
import json
import sys

from src.frontend.parser.parser import (parse_with_diagnostics, ParserError)
from src.frontend.warnings import find_unused_variables
from src.frontend.warnings import find_unreachable_code

source = sys.stdin.read()

try:
    # Suppress lexer/parser debug prints so stdout contains JSON only.
    with contextlib.redirect_stdout(io.StringIO()):
        module, ast_types, ast_functions, parser_errors = (parse_with_diagnostics(source)
        )

    unused_warnings = find_unused_variables(module)
    unreachable_warnings = find_unreachable_code(module)

    diagnostics = []

    for error in parser_errors:
        diagnostics.append(
            {
                "code": error.code,
                "message": error.message,
                "line": error.line,
                "column": error.col,
                "severity": error.severity
            }
        )

    for warning in unused_warnings:
        diagnostics.append(
            {
                "message": warning["message"],
                "line": warning["line"],
                "column": warning["col"],
                "severity": "warning"
            }
        )

    for warning in unreachable_warnings:
        diagnostics.append(
            {
                "message": warning["message"],
                "line": warning["line"],
                "column": warning["col"],
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
                "code": error.code,
                "message": error.message,
                "line": error.line,
                "column": error.col,
                "severity": error.severity
            }
        ]
    }))