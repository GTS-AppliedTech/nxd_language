---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO950",
  "title": "Read me",
  "description": "nxdlang project repository read-me",
  "layer": "Root",
  "category": "read me",
  "keywords": [],
  "doc_version": "2.1",
  "status": "active"
}
---

# RO950 README : NXD

##### Copyright (c) 2026 G.T.S. Applied Technologies LLC

***NXD is a statically typed programming language designed to target Nim, D, and Elixir from a common semantic and intermediate representation.***

NXD is an experimental language and compiler project focused on creating a portable, behavior-defined programming model that can be implemented across multiple runtime and ecosystem targets without changing the meaning of user code.

##### ***Warning: NXD is not production-ready. Specifications, compiler architecture, syntax, runtime behavior, and implementation details may change as the project evolves.***

---

# Project Status

### Stage

Core Compiler & Open Source

### Compiler Status

Implemented

### Current Focus

Compiler Maturation, Semantic Validation, Runtime Research, and Conformance Testing

### Validation Suite

- 40 Base Positive Validation Tests
- 38 Base Negative Validation Tests
- 10 Nim Specialty Backend Tests

### Current Results

#### Positive Tests

- 38 Passing
- 2 Failing

#### Negative Tests

- 25 Passing
- 13 Failing

#### Nim Specialty Tests

- 8 Passing
- 2 Failing

Validation numbers are expected to change as parser coverage expands and current failure groups are resolved.

---

# Current Compiler Pipeline

NXD currently utilizes a Python frontend and a Rust semantic backend connected through a JSON Intermediate Representation (IR).

```text
Scanner
→ Parser
→ AST
→ Lowering
→ IR JSON
→ Rust Loader
→ IR Root
→ NXD Semantics
→ Backend Transpilation
```

This separation creates clear validation boundaries and allows frontend, semantic, and backend validation to occur independently.

---

# Recent Compiler Milestones

### Diagnostic Infrastructure

During the current compiler maturation phase, NXD completed full semantic source location propagation through:

```text
Scanner
→ AST
→ IR
→ JSON
→ Rust Semantic Loader
→ Semantic Diagnostics
→ VS Code LSP
```

---

# Diagnostic Coverage

NXD provides parser, warning, semantic, and compiler diagnostics through the integrated semantic analysis and VS Code LSP infrastructure.

All diagnostics support source location tracking and editor integration.

## Parser Diagnostics (NXD-P Series)

Parser diagnostics are emitted when a valid token stream cannot be transformed into a valid NXD syntax structure.

| Code | Description |
|--------|-------------|
| NXD-P1001 | Expected `{kind}`, got `"..."` |
| NXD-P1002 | Expected `{val}`, got `"..."` |
| NXD-P1003 | Expected `STRUCT`, `ENUM`, `UNION`, or `TRAIT`, got `"..."` |
| NXD-P1004 | Expected `CLONE`, got `"..."` |
| NXD-P1005 | Expected source identifier after `CLONE`, got `"..."` |
| NXD-P1006 | Expected `TO` after `CLONE` source, got `"..."` |
| NXD-P1007 | Expected target identifier after `TO`, got `"..."` |
| NXD-P1008 | Expected token in primary |
| NXD-P1009 | Literal expected, got `"..."` |
| NXD-P1010 | Expected map key, got `"..."` |

---

## Warning Diagnostics (NXD-W Series)

Warnings indicate potentially unintended code while still allowing compilation to continue.

### NXD-W2001

```text
Variable '<name>' declared but never used
```

Generated when a variable is declared but never subsequently referenced.

### NXD-W2002

```text
Unreachable code
```

Generated when the compiler determines that a statement or block can never be executed due to control-flow analysis.

### NXD-W2003

```text
Variable shadows existing binding
```

Generated when an identifier hides an existing symbol in the current scope hierarchy.

---

## Semantic Diagnostics (NXD-S Series)

Semantic diagnostics are produced after parsing during semantic analysis.

### NXD-S3001

```text
Undefined symbol
```

Generated when a referenced symbol cannot be resolved from the active symbol table.

### NXD-S3002

```text
Type mismatch
```

Generated when an operation receives incompatible types.

### NXD-S3003

```text
Trait not implemented
```

Diagnostic infrastructure exists, however generic constraint resolution and trait enforcement are not currently implemented.

This diagnostic is reserved for future generic constraint validation.

### NXD-S3004

```text
Invalid cast
```

Generated when a cast is semantically invalid.

---

## Compiler & LSP Diagnostics (NXD-LSP Series)

The NXD VS Code extension communicates with the Rust semantic compiler through the Language Server Protocol (LSP).

These diagnostics indicate compiler infrastructure, semantic validation, or integration failures rather than errors in user source code.

### NXD-LSP0001

```text
Rust semantic compiler has not been built
```

Generated when the extension cannot locate the compiled Rust semantic compiler executable.

Typical resolution:

```bash
cargo build
```

### NXD-LSP0002

```text
Rust semantic validation failed
```

Generated when the Rust semantic compiler encounters an internal validation failure while processing frontend IR.

Typical causes include:

- IR schema mismatches
- Semantic loader failures
- Deserialization failures
- Internal semantic analysis failures

### NXD-LSP0003

```text
Invalid semantic diagnostic response: {error}
```

Generated when the VS Code extension receives an unexpected or malformed semantic diagnostic response.

This diagnostic indicates an integration issue between the frontend diagnostic pipeline and the Rust semantic compiler.

---

## Current Diagnostic Status

| Category | Status |
|----------|----------|
| Parser Diagnostics (NXD-P) | ✅ Active |
| Warning Diagnostics (NXD-W) | ✅ Active |
| Semantic Diagnostics (NXD-S) | ✅ Active |
| Compiler Diagnostics (NXD-LSP) | ✅ Active |
| Source Span Propagation | ✅ Active |
| Full-Range Highlighting | ✅ Active |
| VS Code Integration | ✅ Active |
| Generic Constraint Resolution (S3003) | 🚧 Planned |
### Source Span Support

NXD diagnostics now carry:

```text
line
column
end_line
end_column
```

allowing editor integrations to provide full-range highlighting rather than line-only diagnostics.

### VS Code Extension

The NXD VS Code extension now provides:

- Syntax Highlighting
- Semantic Diagnostics
- Warning Diagnostics
- Full-Range Squiggle Support
- Source Span Integration
- Language Activation
- File Associations

Publisher:

```text
nxdlang
```

Extension Identifier:

```text
nxdlang.nxd
```

Current Extension Version:

```text
0.1.0
```

---

# NXD File Types & Icons

NXD distinguishes between source files, generated artifacts, and tooling/configuration files.

## NXD Source Files

Human-authored NXD source code.

Examples:

```text
main.nxd
api.nxd
user.nxd
```

Used for:

- Applications
- Libraries
- Services
- Runtime source

Features:

- Syntax Highlighting
- Diagnostics
- Semantic Analysis
- LSP Support

---

## Generated NXD Artifacts

Machine-generated files produced by NXD tooling, runtime services, or compiler infrastructure.

Examples:

```text
module.ir.nxd
module.runtime.nxd
module.generated.nxd
```

Used for:

- IR inspection
- Runtime generation
- Build outputs
- Compiler debugging

These files are intended primarily for analysis and troubleshooting rather than direct modification.

---

## NXD Tooling & Configuration Files

Project configuration, runtime metadata, dependency tracking, and ecosystem tooling support.

Examples:

```text
nxd.toml
runtime.nxd.toml
security.nxd.toml
nxd.lock
```

Comparable to:

```text
Cargo.toml
Cargo.lock
package.json
package-lock.json
pyproject.toml
```

Used for:

- Build configuration
- Runtime configuration
- Security policies
- Dependency management
- Toolchain metadata

---

# Active Development Focus

### Compiler

- Parser refinement
- Lexer refinement
- Semantic conformance expansion
- Validation framework growth
- Diagnostic coverage expansion

### Backends

#### Active

- Nim

#### Planned

- D
- Elixir

### Runtime Research

Current runtime exploration areas include:

```text
runtime/
├── async/
├── channels/
├── experimental/
├── lowering/
├── runtime_ir/
├── scheduler/
└── tasks/
```

Current runtime work remains exploratory and research-oriented.

---

# Project Maturity

## Completed

- Core Language Specification
- Scanner
- Parser Framework
- AST Generation
- Lowering Framework
- JSON Intermediate Representation
- Python → Rust IR Handoff
- Rust IR Loader
- IR Root Construction
- NXD Semantic Analysis Framework
- Semantic Diagnostic Framework
- Source Span Propagation
- Full Diagnostic Range Support
- Nim Backend
- Validation Framework
- Metadata Standards
- Contributor Documentation
- Public Repository
- Public Website

## In Progress

- Parser Enhancement
- Lexer Enhancement
- Semantic Conformance Expansion
- Nim Backend Validation
- Validation Documentation Expansion
- Runtime Architecture Research

## Planned

- D Backend
- Elixir Backend
- Generic Constraint Resolution
- Trait Constraint Enforcement
- Async Runtime Prototype
- Runtime IR Expansion
- Public Playground
- Multi-Backend Conformance Matrix
- Release Candidate Program
- NXD v1.0

---

# Language Philosophy

NXD is developed around a simple principle:

***Transpilation is an implementation strategy, not the definition of the language.***

The language specification defines the observable behavior.

Compilers, runtimes, generated support code, and backend strategies exist to realize that behavior.

By defining semantics first and implementation strategies second, NXD seeks to prevent any backend target from becoming the language's de facto definition.

---

# Target Ecosystems

NXD initially targets:

### Nim

Native systems development and broad backend support.

### D

Native systems programming with high-performance execution.

### Elixir

Distributed and fault-tolerant execution through the Erlang VM.

Each ecosystem was selected to explore how a single semantic model can be realized across dramatically different execution environments while preserving observable behavior.

---

# Why Follow NXD?

NXD may be interesting to developers interested in:

- Programming Language Design
- Compiler Construction
- Intermediate Representations
- Runtime Architecture
- Static Type Systems
- Systems Programming
- Distributed Systems
- Multi-Target Compilation
- Security-Oriented Development
- AI-Assisted Software Engineering

---

# Vision

The long-term vision of NXD is a language whose meaning is defined by its specification rather than any specific compiler, runtime, backend, or implementation strategy.

Whether executed through transpilation, generated runtime support, native compilation, or a future standalone execution model, NXD remains defined by its semantics.

---

# Status

##### Compiler: Active Validation

##### Stage: Core Compiler & Open Source

##### License: MIT License

##### Current Focus: Compiler Maturation, Semantic Validation, Runtime Research, and Backend Conformance