# run_st001.py

from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json

src = Path("tests/fixtures/st001.nxd").read_text()

compile_to_ir_json(
    src,
    "tests/generated/st001_ir.json"
)

print("IR generated")