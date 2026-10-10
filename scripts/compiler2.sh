#!/usr/bin/bash/env bash

echo "==== Running .nxd compiler IR handoff validation ===="
echo "==== frontend serialization / backend deserialization ===="

if command -v python >/dev/null 2>&1; then
    python run_LVL2_handoff_test.py
elif command -v python3 >/dev/null 2>&1; then
    python3 run_LVL2_handoff_test.py
else
    echo "Neither 'Python' nor 'Python3' was found."
    exit 1
fi