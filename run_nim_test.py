from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json

src = Path("tests/fixtures/st013.nxd").read_text()

compile_to_ir_json(
    src,
    "tests/generated/st013_ir.json"
)

print("IR generated")
import subprocess

json_path = "tests/generated/partial_pipeline_tests/st013_ir.json"
nim_path = "tests/generated/python_rust_ir_handoff/st013_json_rust.nim"

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