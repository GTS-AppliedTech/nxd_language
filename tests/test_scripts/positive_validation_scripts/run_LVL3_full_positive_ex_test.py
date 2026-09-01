from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
import subprocess

TEST_ID = "st004"

src_path = f"tests/fixtures/{TEST_ID}.nxd"
json_path = f"tests/generated/LVL1_compiler_tests/{TEST_ID}_ir.json"
ex_path = f"tests/generated/LVL3_full_pipeline_ex_tests/{TEST_ID}.ex"

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
        ex_path
    ],
    check=True
)

print("Semantic + Rust compilation successful")