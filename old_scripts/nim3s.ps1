Write-Host "====  Running a .nxd -> .nim specialty tests ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL3_nim_specialty.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL3_nim_specialty.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}