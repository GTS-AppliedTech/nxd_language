import subprocess

TESTS = [
    "st610",
    "st617",
    "st618",
    "st641",
    "st629",
    "st631",
    "st637",
    "st660",
    "st619",
    "st640",
]


for test_id in TESTS:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/ex_expected_pass_tests/"
        f"{test_id}.nxd"
    )

    json_path = (
        "tests/generated/LVL1_compiler_tests/"
        f"{test_id}_ir.json"
    )

    ex_path = (
        "tests/generated/LVL3_full_pipeline_ex_tests/"
        f"{test_id}.ex"
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
                ex_path,
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