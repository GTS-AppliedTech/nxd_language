#!/usr/bin/bash/env bash

echo "==== Running .nxd -> .nim negative batch tests ===="

if command -v python >/dev/null 2>&1; then
    python run_LVL4_neg_nim_batch_tests.py
elif command -v python3 >/dev/null 2>&1; then
    python3 run_LVL4_neg_nim_batch_tests.py
else
    echo "Neither 'Python' nor 'Python3' was found."
    exit 1
fi