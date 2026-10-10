#!/usr/bin/bash/env bash

echo "==== Running .nxd -> .nim bulk specialty tests ===="

if command -v python >/dev/null 2>&1; then
    python run_LVL4_nim_btch_spcl.py
elif command -v python3 >/dev/null 2>&1; then
    python3 run_LVL4_nim_btch_spcl.py
else
    echo "Neither 'Python' nor 'Python3' was found."
    exit 1
fi