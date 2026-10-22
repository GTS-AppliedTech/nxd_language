Write-Host "==== Running a .nxd -> .nim negative test ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL3_full_negative_nim_test.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL3_full_negative_nim_test.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}