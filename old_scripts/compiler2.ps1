Write-Host "==== Running .nxd compiler IR handoff validation ===="
Write-Host "==== frontend serialization / backend deserialization ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL2_handoff_test.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL2_handoff_test.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}