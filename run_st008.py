from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json

src = Path("tests/fixtures/st008.nxd").read_text()

compile_to_ir_json(
    src,
    "tests/generated/st008_ir.json"
)

print("IR generated")
import subprocess

json_path = "tests/generated/st008_ir.json"
nim_path = "tests/generated/st008_json_rust.nim"

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