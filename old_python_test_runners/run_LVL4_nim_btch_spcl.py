from pathlib import Path
from src.frontend.ir.main import compile_to_ir_json
import subprocess

TESTS = [
    "st201",
    "st202",
    "st203",
    "st204",
    "st205",
    "st206",
    "st207",
    "st208",
    "st209",
    "st210"
]

for test_id in TESTS:

    print(f"\n=== Running {test_id} ===")

    src_path = f"tests/fixtures/nim_specialty_tests/{test_id}.nxd"
    json_path = f"tests/generated/LVL1_compiler_tests/{test_id}_ir.json"
    nim_path = f"tests/generated/LVL3_full_pipeline_nim_tests/{test_id}.nim"

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
                nim_path
            ],
            check=True
        )

        print(f"{test_id}: Semantic + Rust compilation successful")

    except Exception as e:
        print(f"{test_id}: FAILED")
        print(e)