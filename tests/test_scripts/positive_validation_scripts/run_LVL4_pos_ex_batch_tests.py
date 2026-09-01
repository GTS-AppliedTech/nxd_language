from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
import subprocess

TESTS = [
    "st601",
    "st602",
    "st603",
    "st604",
    "st605",
    "st606",
    "st607",
    "st608",
    "st609",
    "st610"
]

for test_id in TESTS:

    print(f"\n=== Running {test_id} ===")

    src_path = f"tests/fixtures/expected_pass_tests/{test_id}.nxd"
    json_path = f"tests/generated/LVL1_compiler_tests/{test_id}_ir.json"
    ex_path = f"tests/generated/LVL3_full_pipeline_ex_tests/{test_id}.ex"

    try:
        src = Path(src_path).read_text()

        compile_to_ir_json(
            src,
            json_path
        )

        print(f"{test_id}: IR generated")

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

        print(f"{test_id}: Semantic + Rust compilation successful")

    except Exception as e:
        print(f"{test_id}: FAILED")
        print(e)