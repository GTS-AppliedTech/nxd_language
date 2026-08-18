# run_st003.py

from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json

src = Path("tests/fixtures/st003.nxd").read_text()
from src.frontend.lexer.scanner import lex

print(lex(src))

compile_to_ir_json(
    src,
    "tests/generated/st003_ir.json"
)

print("IR generated")