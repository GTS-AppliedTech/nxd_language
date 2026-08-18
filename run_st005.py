import subprocess

json_path = "tests/generated/st002_ir.json"
nim_path = "tests/generated/st005_rust.nim"

subprocess.run(
    [
        "cargo",
        "run",
        json_path,
        nim_path
    ],
    check=True
)