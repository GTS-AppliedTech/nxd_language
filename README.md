---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO950",
  "title": "Read me",
  "description": "nxdlang project repository read-me",
  "layer": "Root",
  "category": "readme",
  "keywords": ["readme"],
  "doc_version": "3.0",
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

Implemented with a Rust-Native Compiler Pipeline

### Current Focus

Compiler Maturation, Semantic Validation, Runtime Research, Backend Conformance, and Expanded Validation

### Rust Migration Status

The NXD compiler frontend has been successfully migrated from Python to Rust.

**The active compiler now uses Rust throughout:**

Scanner
→ Parser
→ AST
→ Lowering
→ IR Root
→ NXD Semantics
→ Backend Transpilation


The NXD Language Server Protocol implementation has also been migrated to Rust, creating a unified Rust compiler and editor tooling pipeline.

Behavioral parity has been achieved between the former Python frontend and the new Rust frontend.

The original Python implementation remains preserved within the repository as an archived reference implementation.

Validation Suite
47 Base Positive Validation Tests
49 Base Negative Validation Tests
10 Nim Specialty Backend Tests
Migration Validation
Rust frontend migration completed
Frontend behavioral parity achieved
Native Rust AST-to-IR lowering implemented
Former JSON rehydration boundary removed from the active compiler pipeline
Rust semantic backend integrated directly with the Rust frontend
NXD LSP migrated to Rust
Original Python frontend retained as an archived reference

Validation totals and individual test results are expected to change as compiler coverage, semantics, diagnostics, and backend conformance continue to expand.

##### Current Compiler Pipeline

NXD now utilizes a unified Rust compiler pipeline.

*NXD Source*
→ Rust Scanner
→ Rust Parser
→ Rust AST
→ Rust Lowering
→ IR Root
→ NXD Semantics
→ Backend Transpilation


The frontend, semantic analyzer, intermediate representation, diagnostics infrastructure, and active backend integration now operate within the same Rust implementation.

The previous active compiler architecture used a Python frontend connected to the Rust semantic backend through serialized JSON:

*Python Scanner*
→ Python Parser
→ Python AST
→ Python Lowering
→ IR JSON
→ Rust Loader
→ IR Root
→ NXD Semantics
→ Backend Transpilation


That serialization and rehydration boundary is no longer part of the active compiler pipeline.

JSON output may still be produced as a compiler debugging and inspection artifact, but it is no longer required as the internal handoff between the NXD frontend and semantic backend.

##### Compiler Architecture

The active NXD compiler is organized around a native Rust data flow:

*Source NXD*
→ Tokens
→ AST
→ IR
→ Semantic Analysis
→ Backend


This architecture provides:

A unified compiler implementation language
Direct transfer of structured data between compiler stages
Native AST-to-IR lowering
Reduced duplication between frontend and backend structures
Consistent source span propagation
Shared diagnostic infrastructure
Direct compiler and LSP integration
JSON generation without JSON rehydration
Clear stage boundaries for validation and debugging

Intermediate JSON output remains available where useful for:

Compiler debugging
IR inspection
Validation artifacts
Regression analysis
Tooling interoperability
Archived Python Reference

The original Python frontend remains preserved as a historical and behavioral reference in the 'nxd-rust-frontend' repository.

The archived implementation provides:

A record of the original parser behavior
A comparison point for future parity verification
Historical AST and lowering behavior
Reference implementations for migrated compiler features
Additional context for compiler regression analysis
Preservation of the original Python-to-Rust compiler architecture

Archived files are not assumed to be part of the active Rust compiler.

Root-level folders and files marked with an old_ prefix represent components from the previous compiler layout unless otherwise documented.

The active Rust implementation is authoritative for current compiler behavior.

Recent Compiler Milestones
Rust Frontend Migration

The NXD frontend has been migrated from Python to Rust, including the active:

Scanner
Parser
AST
Lowering
Diagnostics
IR Construction
Compiler Orchestration


The Rust frontend now connects directly to the existing Rust semantic and backend systems.

Native IR Integration

The compiler no longer requires the frontend to serialize its intermediate representation to JSON before semantic analysis.

The active handoff now occurs directly through native Rust structures:

Rust AST
→ Rust Lowering
→ IR Root
→ Semantic Analysis


This removes the former serialization and deserialization boundary while preserving JSON as an optional diagnostic and debugging artifact.

*Behavioral Parity*

The Rust frontend has achieved behavioral parity with the former Python frontend across the active NXD compiler pipeline.

Parity validation focused on preserving:

Scanner behavior
Parser behavior
AST structure
Lowering behavior
IR construction
Diagnostic behavior
Semantic integration
Backend compatibility
Validation-suite behavior
Rust Language Server

The NXD Language Server Protocol implementation has been migrated to Rust.

The active LSP now operates through the same Rust compiler infrastructure used by the command-line compiler:

Editor
→ Rust LSP
→ Rust Compiler Frontend
→ NXD Semantics
→ Diagnostics
→ Editor Response


This creates a unified implementation for compiler validation and editor diagnostics.

##### Diagnostic Infrastructure

NXD supports source location propagation through the native Rust pipeline:

Scanner
→ AST
→ IR
→ Semantic Diagnostics
→ Rust LSP
→ VS Code


Source spans remain associated with compiler structures throughout parsing, lowering, semantic analysis, and editor reporting.

**Diagnostic Coverage**

NXD provides parser, warning, semantic, and compiler diagnostics through the integrated Rust compiler and Language Server Protocol infrastructure.

All active diagnostics support source location tracking and editor integration.

---

*Parser Diagnostics* (NXD-P Series)

Parser diagnostics are emitted when a valid token stream cannot be transformed into a valid NXD syntax structure.

Code	Description:

NXD-P1001
Expected {kind}, got "..."

NXD-P1002
Expected {val}, got "..."

NXD-P1003	
Expected STRUCT, ENUM, UNION, or TRAIT, got "..."

NXD-P1004	
Expected CLONE, got "..."

NXD-P1005
Expected source identifier after CLONE, got "..."

NXD-P1006	
Expected TO after CLONE source, got "..."

NXD-P1007
Expected target identifier after TO, got "..."

NXD-P1008	
Expected token in primary

NXD-P1009-01
Literal expected, got "..."

NXD-P1009-02  
Invalid integer literal: {} **(Implementation in progress)**

NXD-P1009-03  
Invalid float literal: {} **(Implementation in progress)**

NXD-P1009-04  
Invalid string literal: {} **(Implementation in progress)**

NXD-P1009-05  
Invalid boolean literal: {} **(Implementation in progress)**

NXD-P1009-06  
Invalid none literal: {} **(Implementation in progress)**

NXD-P1009-07  
Invalid list literal: {} **(Implementation in progress)**

NXD-P1009-08  
Invalid map literal: {} **(Implementation in progress)**

NXD-P1009-09  
Invalid lambda literal: {} **(Implementation in progress)**


NXD-P1010	
Expected map key, got "..."

NXD-P1011
Expected TO after MOVE source

NXD-P1012
Expected type after AS/IS, got "..."

---

Warning Diagnostics (NXD-W Series)

Warnings indicate potentially unintended code while still allowing compilation to continue.

NXD-W2001
Variable '<name>' declared but never used


Generated when a variable is declared but never subsequently referenced.

NXD-W2002
Unreachable code


Generated when the compiler determines that a statement or block can never be executed due to control-flow analysis.

NXD-W2003
Variable shadows existing binding


Generated when an identifier hides an existing symbol in the current scope hierarchy.

---

*Semantic Diagnostics* (NXD-S Series)

Semantic diagnostics are produced after parsing and IR lowering during semantic analysis.

NXD-S3001
Undefined symbol


Generated when a referenced symbol cannot be resolved from the active symbol table.

NXD-S3002
Type mismatch


Generated when an operation receives incompatible types.

NXD-S3003
Trait not implemented


Diagnostic infrastructure exists; however, generic constraint resolution and trait enforcement are not currently implemented.

This diagnostic is reserved for future generic constraint validation.

NXD-S3004
Invalid cast


Generated when a cast is semantically invalid.

**NXD-S3005 (ASYNC sub-codes)**

NXD-S3005-01
'AWAIT' outside 'ASYNC' function

NXD-S3005-02
'AWAIT' non-awaitable function

NXD-S3005-03
Invalid 'SPAWN' target

NXD-S3005-04
Invalid 'SEND' target

NXD-S3005-05
Invalid 'RECV' source

NXD-S3005-06
'ASYNC' context violation

NXD-S3005-07
Invalid 'PING' target

NXD-S3005-08
Invalid 'ESC' target

NXD-S3005-09
Invalid 'TIMEOUT' target

NXD-S3005-10
Invalid 'SYNC' target

NXD-S3005-11
Invalid 'CHANNEL' declaration

NXD-S3005-12
Invalid 'TASK' target

---

*Compiler & LSP Diagnostics* (NXD-LSP Series)

The NXD VS Code extension communicates with the Rust NXD Language Server through the Language Server Protocol.

The Rust language server uses the active Rust compiler pipeline for parsing, lowering, semantic validation, and diagnostic generation.

These diagnostics indicate compiler infrastructure, semantic validation, or editor integration failures rather than errors in user source code.

NXD-LSP0001
Rust NXD compiler or language server has not been built


Generated when the extension cannot locate the required compiled Rust executable.

**Typical resolution:**

cargo build

NXD-LSP0002
Rust compiler validation failed


Generated when the Rust compiler encounters an internal validation failure while processing NXD source.

Typical causes may include:

AST or IR construction failures
Semantic analysis failures
Internal compiler failures
Compiler and LSP integration failures
NXD-LSP0003
Invalid compiler diagnostic response: {error}


Generated when the VS Code extension receives an unexpected or malformed diagnostic response from the Rust language server.

This diagnostic indicates an integration issue between the VS Code extension, Rust LSP, and compiler diagnostic pipeline.

*Current Diagnostic Status*
Category	Status
Parser Diagnostics (NXD-P) - Active
Warning Diagnostics (NXD-W)	- Active
Semantic Diagnostics (NXD-S)	- Active
Compiler Diagnostics (NXD-LSP)	- Active
Rust Frontend Diagnostics	- Active
Rust LSP Integration	- Active
Source Span Propagation	- Active
Full-Range Highlighting	- Active
VS Code Integration	- Active
Generic Constraint Resolution (S3003)	- Planned
Source Span Support

NXD diagnostics carry:

line
column
end_line
end_column


This allows editor integrations to provide full-range highlighting rather than line-only diagnostics.

Source spans are propagated through the native Rust compiler pipeline from frontend parsing to semantic analysis and LSP reporting.

##### VS Code Extension

The NXD VS Code extension provides:

Syntax Highlighting
Parser Diagnostics
Semantic Diagnostics
Warning Diagnostics
Full-Range Squiggle Support
Source Span Integration
Rust Language Server Integration
Language Activation
File Associations
---
Publisher:

nxdlang


Extension Identifier:

nxdlang.nxd


Current Extension Version:

0.2.15
---
NXD File Types & Icons

NXD distinguishes between source files, generated artifacts, and tooling/configuration files.

NXD Source Files

Human-authored NXD source code.

Examples:

main.nxd
api.nxd
user.nxd


Used for:

Applications
Libraries
Services
Runtime source

Features:

Syntax Highlighting
Diagnostics
Semantic Analysis
LSP Support
Generated NXD Artifacts

Machine-generated files produced by NXD tooling, runtime services, or compiler infrastructure.

Examples:

module.ir.nxd
module.runtime.nxd
module.generated.nxd


Used for:

IR inspection
Runtime generation
Build outputs
Compiler debugging

These files are intended primarily for analysis and troubleshooting rather than direct modification.

NXD Tooling & Configuration Files

Project configuration, runtime metadata, dependency tracking, and ecosystem tooling support.

Examples:

nxd.toml
runtime.nxd.toml
security.nxd.toml
nxd.lock


Comparable to:

Cargo.toml
Cargo.lock
package.json
package-lock.json
pyproject.toml


Used for:

Build configuration
Runtime configuration
Security policies
Dependency management
Toolchain metadata
Active Development Focus
Compiler
Rust parser refinement
Rust scanner refinement
AST and IR maturation
Semantic conformance expansion
Validation framework growth
Diagnostic coverage expansion
Compiler pipeline stabilization
Archived Python reference organization
Language Server
Rust LSP stabilization
Compiler and editor diagnostic parity
Editor integration testing
Diagnostic response validation
Additional language intelligence
Backends
Active
Nim
Planned
D
Elixir
Runtime Research
---
Current runtime exploration areas include:

runtime/
├── async/
├── channels/
├── experimental/
├── lowering/
├── runtime_ir/
├── scheduler/
└── tasks/


Current runtime work remains exploratory and research-oriented.
---
Project Maturity
Completed
Core Language Specification
Rust Scanner
Rust Parser
Rust AST Generation
Rust Lowering
Native Rust IR Construction
IR Root Construction
Python Frontend to Rust Frontend Migration
Frontend Behavioral Parity
Removal of the Active JSON Rehydration Boundary
Optional JSON Debug and Inspection Output
NXD Semantic Analysis Framework
Semantic Diagnostic Framework
Parser Diagnostic Framework
Warning Diagnostic Framework
Source Span Propagation
Full Diagnostic Range Support
Rust Language Server
VS Code LSP Integration
Nim Backend
Validation Framework
Metadata Standards
Contributor Documentation
Archived Python Reference Implementation
Public Repository
Public Website
In Progress
Rust Parser Enhancement
Rust Scanner Enhancement
AST and IR Refinement
Semantic Conformance Expansion
Nim Backend Validation
Rust LSP Maturation
Validation Documentation Expansion
Runtime Architecture Research
Repository Consolidation
Planned
D Backend
Elixir Backend
Generic Constraint Resolution
Trait Constraint Enforcement
Async Runtime Prototype
Runtime IR Expansion
Public Playground
Multi-Backend Conformance Matrix
Release Candidate Program
NXD v1.0
Language Philosophy

NXD is developed around a simple principle:

Transpilation is an implementation strategy, not the definition of the language.

The language specification defines the observable behavior.

Compilers, runtimes, generated support code, and backend strategies exist to realize that behavior.

By defining semantics first and implementation strategies second, NXD seeks to prevent any backend target from becoming the language's de facto definition.

The migration from a Python frontend to a Rust frontend does not redefine NXD. It changes the compiler implementation while preserving the language's intended behavior.

Target Ecosystems

NXD initially targets:

Nim

Native systems development and broad backend support.

D

Native systems programming with high-performance execution.

Elixir

Distributed and fault-tolerant execution through the Erlang VM.

Each ecosystem was selected to explore how a single semantic model can be realized across dramatically different execution environments while preserving observable behavior.

Why Follow NXD?

NXD may be interesting to developers interested in:

Programming Language Design
Compiler Construction
Rust Compiler Development
Intermediate Representations
Language Server Protocol Implementation
Runtime Architecture
Static Type Systems
Systems Programming
Distributed Systems
Multi-Target Compilation
Security-Oriented Development
AI-Assisted Software Engineering
Vision

The long-term vision of NXD is a language whose meaning is defined by its specification rather than any specific compiler, runtime, backend, or implementation strategy.

Whether executed through transpilation, generated runtime support, native compilation, or a future standalone execution model, NXD remains defined by its semantics.

The unified Rust compiler and language server provide the current implementation foundation for advancing that vision while the archived Python frontend preserves the project's original reference architecture.

Status
Compiler: Rust-Native and Under Active Validation
Frontend: Rust Migration Complete
Backend: Rust
Language Server: Rust
Migration: Behavioral Parity Achieved
Archived Reference: Original Python Frontend Preserved
Stage: Core Compiler & Open Source
License: MIT License
Current Focus: Compiler Maturation, Semantic Validation, LSP Stabilization, Runtime Research, and Backend Conformance

### Main changes

- Replaced the Python-to-JSON-to-Rust pipeline with the active native Rust pipeline.
- Described JSON as an optional debugging artifact rather than an internal compiler requirement.
- Added the completed frontend migration, behavioral parity, and all-Rust LSP as milestones.
- Clearly preserved the Python implementation as an archived reference.
- Updated LSP wording so it no longer describes communication with a separate semantic compiler through frontend IR.
- Removed the stale passing/failing result breakdown because parity has been achieved but updated exact totals were not supplied.
- Updated the completed, in-progress, architecture, and status sections to reflect the repository flip without implying that the historical implementation was deleted.