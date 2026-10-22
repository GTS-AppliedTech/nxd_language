from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
import subprocess

TESTS = [
    "st403",
    "st405",
    "st406",
    "st412",
    "st417",
    "st426",
    "st433",
    "st439",
    "st440",
    "st410"
]

for test_id in TESTS:

    print(f"\n=== Running {test_id} ===")

    src_path = f"tests/fixtures/d_expected_fail_tests/{test_id}.nxd"
    json_path = f"tests/generated/LVL1_compiler_tests/{test_id}_ir.json"
    d_path = f"tests/generated/LVL3_full_pipeline_d_tests/{test_id}.d"

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
                d_path
            ],
            check=True
        )

        print(f"{test_id}: Semantic + Rust compilation successful")

    except Exception as e:
        print(f"{test_id}: FAILED")
        print(e)