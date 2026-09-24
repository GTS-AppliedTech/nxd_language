import contextlib
import io
import json
import sys
import os
import subprocess
import tempfile

from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
from src.frontend.parser.parser import (parse_with_diagnostics, ParserError,)
from src.frontend.warnings import find_unused_variables
from src.frontend.warnings import find_unreachable_code
from src.frontend.warnings import find_shadowed_bindings

source = sys.stdin.read()

def get_semantic_diagnostics(source):
    repo_root = Path(__file__).resolve().parent

    compiler_name = (
        "nxd-compiler.exe"
        if os.name == "nt"
        else "nxd-compiler"
    )

    compiler_path = (
        repo_root
        / "target"
        / "debug"
        / compiler_name
    )

    if not compiler_path.exists():
        return [
            {
                "code": "NXD-LSP0001",
                "message": (
                    "NXD-LSP0001: Rust semantic compiler "
                    "has not been built."
                ),
                "line": 1,
                "column": 1,
                "severity": "error",
            }
        ]

    with tempfile.TemporaryDirectory() as temp_dir:
        ir_path = Path(temp_dir) / "lsp_semantic_ir.json"

        # compile_to_ir_json currently prints AST/debug information.
        # Suppress it so lsp_validate.py keeps stdout JSON-only.
        with contextlib.redirect_stdout(io.StringIO()):
            compile_to_ir_json(
                source,
                str(ir_path),
            )

        result = subprocess.run(
            [
                str(compiler_path),
                "--semantic-diagnostics",
                str(ir_path),
            ],
            cwd=repo_root,
            capture_output=True,
            text=True,
        )

        if result.returncode != 0:
            message = (
                result.stderr.strip()
                or result.stdout.strip()
                or "Rust semantic validation failed."
            )

            return [
                {
                    "code": "NXD-LSP0002",
                    "message": (
                        f"NXD-LSP0002: {message}"
                    ),
                    "line": 1,
                    "column": 1,
                    "severity": "error",
                }
            ]

        try:
            response = json.loads(
                result.stdout.strip()
            )

        except json.JSONDecodeError as error:
            return [
                {
                    "code": "NXD-LSP0003",
                    "message": (
                        "NXD-LSP0003: Invalid semantic "
                        f"diagnostic response: {error}"
                    ),
                    "line": 1,
                    "column": 1,
                    "severity": "error",
                }
            ]

        return response.get("diagnostics", [])

try:
    # Suppress lexer/parser debug prints so stdout contains JSON only.
    with contextlib.redirect_stdout(io.StringIO()):
        module, ast_types, ast_functions, parser_errors = (parse_with_diagnostics(source)
        )

    unused_warnings = find_unused_variables(module)
    unreachable_warnings = find_unreachable_code(module)
    shadowed_warnings = find_shadowed_bindings(module)

    diagnostics = []

    for error in parser_errors:
        diagnostics.append(
            {
                "code": error.code,
                "message": error.message,
                "line": error.line,
                "column": error.col,
                "end_column": error.end_col,
                "severity": error.severity
            }
        )

    for warning in unused_warnings:
        diagnostics.append(
            {
                "code": warning["code"],
                "message": warning["message"],
                "line": warning["line"],
                "column": warning["col"],
                "end_line": warning.get(
                    "end_line",
                    warning["line"],
                ),
                "end_column": warning.get(
                    "end_col",
                    warning["col"] +1,
                ),
                "severity": "warning"
            }
        )

    for warning in unreachable_warnings:
        diagnostics.append(
            {
                "code": warning["code"],
                "message": warning["message"],
                "line": warning["line"],
                "column": warning["col"],
                "end_line": warning.get(
                    "end_line",
                    warning["line"],
                ),
                "end_column": warning.get(
                    "end_col",
                    warning["col"] +1,
                ),
                "severity": "warning"
            }
        )

    for warning in shadowed_warnings:
        diagnostics.append(
            {
                "code": warning["code"],
                "message": warning["message"],
                "line": warning["line"],
                "column": warning["col"],
                "end_line": warning.get(
                    "end_line",
                    warning["line"],
                ),
                "end_column": warning.get(
                    "end_col",
                    warning["col"] +1,
                ),
                "severity": "warning"
            }
        )

    if not parser_errors:
        semantic_diagnostics = get_semantic_diagnostics(
            source
        )

        diagnostics.extend(
            semantic_diagnostics
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
                "end_column": error.end_col,
                "severity": error.severity
            }
        ]
    }))