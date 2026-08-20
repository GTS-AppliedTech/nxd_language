from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json

src = Path("tests/fixtures/st013.nxd").read_text()

compile_to_ir_json(
    src,
    "tests/generated/st013_ir.json"
)

print("IR generated")
import subprocess

json_path = "tests/generated/LVL1_compiler_tests/st013_ir.json"
nim_path = "tests/generated/LVL2_handoff_tests/st013_json_rust.nim"

subprocess.run(
    [
        "cargo",
        "run",
        json_path,
        nim_path
    ],
    check=True
)

print("Rust compilation successful")