from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
import subprocess

TEST_ID = "st001"

src_path = f"tests/fixtures/{TEST_ID}.nxd"
json_path = f"tests/generated/{TEST_ID}_ir.json"
nim_path = f"tests/generated/{TEST_ID}.nim"

src = Path(src_path).read_text()

compile_to_ir_json(
    src,
    json_path
)

print("IR generated")

subprocess.run(
    [
        "cargo",
        "run",
        "--",
        "--semantics",
        json_path,
        nim_path
    ],
    check=True
)

print("Semantic + Rust compilation successful")