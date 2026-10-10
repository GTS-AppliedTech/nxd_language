#!/usr/bin/bash/env bash

echo "==== Running a .nxd -> .nim specialty test ===="
if command -v python >/dev/null 2>&1; then
    python run_LVL3_nim_specialty.py
elif command -v python3 >/dev/null 2>&1; then
    python3 run_LVL3_nim_specialty.py
else
    echo "Neither 'Python' nor 'Python3' was found."
    exit 1
fi