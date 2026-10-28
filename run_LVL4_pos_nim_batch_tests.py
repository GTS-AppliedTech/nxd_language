import subprocess

TESTS = [
    "st001",
    "st002",
    "st003",
    "st004",
    "st005",
    "st006",
    "st007",
    "st008",
    "st009",
    "st010",
]


for test_id in TESTS:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/nim_expected_pass_tests/"
        f"{test_id}.nxd"
    )

    json_path = (
        "tests/generated/LVL1_compiler_tests/"
        f"{test_id}_ir.json"
    )

    nim_path = (
        "tests/generated/LVL3_full_pipeline_nim_tests/"
        f"{test_id}.nim"
    )

    try:
        subprocess.run(
            [
                "cargo",
                "run",
                "--bin",
                "nxd_language",
                "--",
                src_path,
                json_path,
                nim_path,
            ],
            check=True,
        )

        print(f"{test_id}: PASSED")

    except subprocess.CalledProcessError as error:
        print(f"{test_id}: FAILED")
        print(
            f"Compiler exit code: "
            f"{error.returncode}"
        )