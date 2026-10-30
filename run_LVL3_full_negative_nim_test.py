import subprocess

TEST_ID = [
    "TEST",
]


for test_id in TEST_ID:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/nim_expected_fail_tests/"
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