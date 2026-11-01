import subprocess

TESTS = [
    "st310",
    "st317",
    "st318",
    "st341",
    "st329",
    "st331",
    "st337",
    "st360",
    "st319",
    "st340",
]


for test_id in TESTS:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/d_expected_pass_tests/"
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