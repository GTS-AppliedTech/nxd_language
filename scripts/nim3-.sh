#!/usr/bin/bash/env bash

echo "==== Running a .nxd -> .nim negative test ===="

if command -v python >/dev/null 2>&1; then
    python run_LVL3_full_negative_nim_test.py
elif command -v python3 >/dev/null 2>&1; then
    python3 run_LVL3_full_negative_nim_test.py
else
    echo "Neither 'Python' nor 'Python3' was found."
    exit 1
fi