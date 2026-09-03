import contextlib
import io
import json
import sys

from src.frontend.parser.parser import parse, ParserError


source = sys.stdin.read()

try:
    # Suppress lexer/parser debug prints so stdout contains JSON only.
    with contextlib.redirect_stdout(io.StringIO()):
        parse(source)

    print(json.dumps({
        "ok": True,
        "diagnostics": []
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