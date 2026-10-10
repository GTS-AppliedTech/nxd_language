#!/usr/bin/bash/env bash
set -e

echo "================================="
echo "Project root verification"
echo "================================="
cd ~/Projects/nxd_language

echo "================================="
echo "Ensuring build integrity"
echo "================================="
cargo build

echo "================================="
echo "Moving to LSP root"
echo "================================="
cd ~/Projects/nxd_language/nxd-lsp/NXD

echo "================================="
echo "Building LSP"
echo "================================="
npx vsce package

echo "================================="
echo "NXD .vsix build completed!" 
echo "================================="

