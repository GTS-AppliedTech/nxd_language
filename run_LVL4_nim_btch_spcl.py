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
    "st210",
]


for test_id in TESTS:
    print(f"\n=== Running {test_id} ===")

    src_path = (
        "tests/fixtures/nim_specialty_tests/"
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