# nxd-rust-frontend

Status: Archived

This repository previously served as the development and validation environment for migrating the NXD compiler frontend from Python to Rust.

The migration is now complete.

The completed Rust frontend, Rust compiler backend, and Rust Language Server Protocol implementation have been transferred into the primary `nxd_language` repository.

This repository now serves as an archive of the former `nxd_language` implementation and repository structure.

## Migration Result

The NXD compiler migration successfully completed the following:

- Rust scanner
- Rust parser
- Rust AST
- Rust IR lowering
- Native Rust IR construction
- Rust semantic integration
- Rust backend integration
- Rust diagnostic integration
- Rust Language Server Protocol implementation
- Frontend behavioral parity
- Validation-suite parity
- Removal of the active Python-to-JSON-to-Rust handoff

The active NXD compiler is now implemented as a unified Rust pipeline:


NXD Source
→ Rust Scanner
→ Rust Parser
→ Rust AST
→ Rust Lowering
→ IR Root
→ NXD Semantics
→ Backend Transpilation


JSON output may still be generated for debugging, inspection, testing, and compiler analysis, but JSON serialization and rehydration are no longer required as the internal handoff between the frontend and semantic backend.

Repository Purpose

This repository is no longer the active development location for the NXD Rust frontend.

It now preserves the former nxd_language repository contents as an archived reference.

The archived implementation provides:

A historical record of the previous compiler architecture
The former Python frontend
The previous Python-to-JSON-to-Rust compiler pipeline
Earlier Rust compiler and backend components
Previous AST, IR, semantic, and diagnostic structures
A reference point for regression analysis
A comparison point for future compiler validation
Historical context for the completed Rust migration

The contents of this repository should not be assumed to represent the current active NXD compiler architecture.

Migration Background

Before the migration, NXD used a Python frontend and a Rust compiler backend connected through a JSON IR handoff:

Python Scanner
→ Python Parser
→ Python AST
→ Python Lowering
→ IR JSON
→ Rust Loader
→ IR Root
→ NXD Semantics
→ Backend Transpilation


This architecture enabled the initial development of the NXD language, but it also required compiler structures to be maintained across Python and Rust.

The serialized handoff introduced additional:

Data structure duplication
Serialization and deserialization
Frontend and backend synchronization
Cross-language debugging
IR schema maintenance
Diagnostic propagation complexity

The nxd-rust-frontend repository was created as an isolated migration environment where the frontend could be rewritten and validated without disrupting the working implementation in nxd_language.

Migration Goals

The migration was developed around the following goals:

Reimplement the NXD scanner in Rust
Reimplement the NXD parser in Rust
Reimplement the NXD AST in Rust
Reimplement AST-to-IR lowering in Rust
Preserve existing NXD language behavior
Preserve compiler semantics
Preserve diagnostic behavior
Maintain compatibility with the existing semantic analyzer
Maintain compatibility with the existing backend
Validate the Rust implementation against the NXD test corpus
Remove the active JSON rehydration boundary
Preserve JSON as an optional debugging artifact
Migrate the NXD Language Server Protocol implementation to Rust
Establish behavioral parity with the Python frontend
Transfer the completed implementation into nxd_language

These goals have been completed.

Validation Strategy

During migration, the Rust implementation was tested against the existing NXD validation corpus.

Validation focused on preserving:

Scanner behavior
Token production
Parser behavior
AST construction
IR lowering
IR structure
Source span propagation
Warning generation
Parser diagnostics
Semantic diagnostics
Backend integration
Positive test behavior
Negative test behavior
Specialty backend behavior
Language Server Protocol behavior

The former Python implementation served as the behavioral reference while each compiler stage was migrated.

After parity was achieved, the completed Rust implementation became the active compiler in the primary nxd_language repository.

Language Server Migration

The NXD Language Server Protocol implementation was also migrated to Rust.

The active editor diagnostic pipeline now follows the unified Rust architecture:

Editor
→ Rust Language Server
→ Rust Compiler Frontend
→ IR Root
→ NXD Semantics
→ Diagnostics
→ Editor Response


This allows the command-line compiler and editor tooling to use the same frontend, semantic analyzer, source spans, and diagnostic infrastructure.

Current Development

Active NXD compiler development now occurs in the nxd_language repository.

That repository contains the authoritative versions of the:

Rust scanner
Rust parser
Rust AST
Rust lowering pipeline
Rust IR
Semantic analyzer
Diagnostic infrastructure
Compiler orchestration
Backend implementations
Rust language server
Validation suites

This archived repository should not receive active compiler features unless required for historical preservation, archival corrections, or documentation maintenance.

Archive Status

The code preserved here represents an earlier stage of NXD development.

It may contain:

Superseded compiler structures
Previous file locations
Earlier dependency definitions
Deprecated integration boundaries
Files intentionally omitted from the active compiler
Files that were reduced, consolidated, or relocated during migration
Historical Rust components that should not be mixed with the current implementation
Python components retained for reference

The repository is preserved for historical and technical reference and is not the authoritative source for current NXD compiler behavior.

Status Summary
Rust Frontend Migration: Complete
Frontend Behavioral Parity: Achieved
Rust Backend Integration: Complete
Native Rust IR Handoff: Complete
Rust LSP Migration: Complete
Primary Repository Transfer: Complete
Active Development Repository: nxd_language
Repository Role: Archived Reference

Notice

For the current NXD compiler, language server, validation infrastructure, and active development history, refer to the primary nxd_language repository.

This repository remains available as a record of the architecture and implementation that existed before the completed all-Rust compiler migration.


