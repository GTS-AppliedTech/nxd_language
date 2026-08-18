import subprocess

json_path = "tests/generated/st001_ir.json"
nim_path = "tests/generated/st004_rust.nim"

subprocess.run(
    [
        "cargo",
        "run",
        json_path,
        nim_path
    ],
    check=True
)