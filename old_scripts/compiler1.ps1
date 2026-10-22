Write-Host "====  Running .nxd compiler frontend .json creation test ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL1_compiler_test.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL1_compiler_test.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}