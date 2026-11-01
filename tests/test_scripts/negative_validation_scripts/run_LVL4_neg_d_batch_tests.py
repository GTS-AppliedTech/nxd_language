import subprocess

TESTS = [
    "st465",
    "st466",
    "st467",
    "st458",
    "st459",
    "st460",
    "st461",
    "st462",
    "st463",
    "st464",
]


for test_id in TESTS:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/d_expected_fail_tests/"
        f"{test_id}.nxd"
    )

    json_path = (
        "tests/generated/LVL1_compiler_tests/"
        f"{test_id}_ir.json"
    )

    d_path = (
        "tests/generated/LVL3_full_pipeline_d_tests/"
        f"{test_id}.d"
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
                d_path,
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