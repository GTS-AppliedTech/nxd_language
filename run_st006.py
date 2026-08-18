import subprocess

json_path = "tests/generated/st003_ir.json"
nim_path = "tests/generated/st006_rust.nim"

subprocess.run(
    [
        "cargo",
        "run",
        json_path,
        nim_path
    ],
    check=True
)