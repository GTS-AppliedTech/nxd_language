Write-Host "====  Running .nxd -> .nim bulk specialty tests ===="

if (Get-Command python -ErrorAction SilentlyContinue) {
    python run_LVL4_nim_btch_spcl.py
}
elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    python3 run_LVL4_nim_btch_spcl.py
}
else {
    Write-Error "Neither 'Python' or 'Python3' was found."
    exit 1
}