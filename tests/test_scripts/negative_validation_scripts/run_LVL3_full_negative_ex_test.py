import subprocess

TEST_ID = [
    "st768",
]


for test_id in TEST_ID:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/ex_expected_fail_tests/"
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