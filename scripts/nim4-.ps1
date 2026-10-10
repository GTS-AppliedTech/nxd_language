Write-Host "==== Running .nxd -> .nim negative batch tests ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL4_neg_nim_batch_tests.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL4_neg_nim_batch_tests.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}