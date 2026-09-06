---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO900",
  "title": "Changelog (repo)",
  "description": "Project Repo Changelog",
  "layer": "Root",
  "category": "change log",
  "keywords": [],
  "project_version": "1.1.0",
  "site_version": "1.6.10",
  "status": "active"
}
---

# **NXD Language Changelog**


### NXD Language v1.1.0
#### 2026-09-05

Release Focus: LSP Foundation, VS Code Integration, Ownership Semantics, Testing Expansion, and Compiler Stability

### Major Features Added

Language Server Protocol (LSP) Development Begun

NXD now includes the first generation of LSP support.

Implemented features include:

Syntax highlighting support
Diagnostic error reporting
Auto-completion / text prediction
Document parsing integration
VS Code language integration groundwork

This marks the transition from a compiler-only project toward a full developer tooling ecosystem.

#### Visual Studio Code Extension

Initial VS Code extension support has been added.

Features currently available:

NXD file recognition
Syntax highlighting
Language Server integration
Extension packaging and distribution support

This represents the first official IDE experience for NXD development.

#### Ownership System Progress

MOVE Statement Support

Added end-to-end compiler support for:

MOVE SOURCE TO TARGET


Implemented through:

Parser
AST generation
IR generation
JSON serialization
Rust deserialization
Semantic analyzer integration
Nim backend integration

#### CLONE Statement Support

Added end-to-end compiler support for:

CLONE SOURCE TO COPY


Implemented through:

Parser
AST generation
IR generation
JSON serialization
Rust deserialization
Semantic analyzer integration
Nim backend integration

Generated Nim output now correctly performs deep-copy semantics.

### Testing Improvements
Nim Specialty Backend Test Suite Added

Created a dedicated Nim specialty testing layer focused on backend-specific functionality and ownership behaviors.

Coverage includes:

Ownership semantics
MOVE operations
CLONE operations
Invalid ownership usage
Backend-specific edge cases

#### Testing Growth

Significant new specialty tests were added during this development cycle.

Recent testing efforts focused on:

Ownership semantics
Control flow validation
Backend validation
Compiler pipeline verification
IR transport verification

Current test coverage now includes positive tests, negative tests, and specialty backend tests.

#### Comment Handling Validation

A new testing methodology has been introduced.

Ownership and specialty tests now include:

Before-code comments
Inline comments
Between-statement comments
End-of-file comments
Mixed-case comments
Punctuation-heavy comments

This validates comment removal independently from language behavior and provides embedded test documentation.

### Compiler Improvements

#### Ownership Parsing Refactor

Ownership operators were promoted from expression handling into dedicated statement forms.

Previous structure:

ASTUnary(...)
ASTVar("TO")
ASTVar("TARGET")


Current structure:

ASTMove(
    source=ASTVar("SOURCE"),
    target=ASTVar("TARGET")
)


and

ASTClone(
    source=ASTVar("SOURCE"),
    target=ASTVar("COPY")
)


This significantly simplifies ownership analysis and backend implementation.

#### Comment Processing Fixes

Resolved a preprocessing issue affecting comment handling.

Previous behavior could incorrectly terminate processing after encountering:

//


The compiler now properly ignores comment content on the current line while continuing parsing normally.

### Project Progress
Compiler Stability

Numerous parser, AST, IR, semantic, and backend issues were identified and resolved throughout the v1.0.2 cycle.

Special focus areas included:

Ownership semantics
AST → IR lowering
Rust IR reconstruction
Nim backend generation
Semantic validation consistency

### Documentation & Testing Process

Testing documentation has been expanded to record:

Expected behavior
Actual behavior
Expected rejection stage
Compiler output
Generated IR
Generated backend code
Known defects

This improves reproducibility, debugging, and long-term project maintainability.

### Known Issues

#### Ownership Enforcement

MOVE and CLONE pipeline support is now implemented; however, ownership-state tracking is still under active development.

Example:

MOVE SOURCE TO TARGET
PRINTLN(SOURCE)


Currently passes semantic validation in some scenarios where future versions are expected to reject use-after-move behavior.

Additional ownership enforcement work is planned for upcoming releases.

Version: 1.1.0
 Status: Active Development
 Primary Milestone: First LSP implementation, first VS Code extension release, ownership semantics foundation, expanded backend test coverage, and continued compiler stabilization.

### Version 1.0.2
### August 26, 2026

#### Validation Program Expansion
Continued NXD → Nim backend validation effort.
Expanded positive validation coverage through ongoing compiler conformance testing.
Expanded negative validation coverage through intentional compiler rejection testing.
Added and documented additional failure-condition test cases designed to verify compiler rejection behavior.
Continued verification of parser, semantic, IR, and backend-stage behavior through structured validation workflows.

#### Test Documentation
Added comprehensive public-facing documentation for validation results.
Expanded test result coverage available through project documentation.
Improved organization of compiler validation records.
Standardized documentation structure for positive and negative validation reporting.

#### Compiler Development
Continued refinement and validation of the Nim backend implementation.
Continued investigation and tracking of remaining validation failures.
Ongoing review of compiler behavior against documented NXD language specifications.
Continued stabilization efforts surrounding compiler conformance and expected language behavior.

#### Project Status
Stage 2: Core Compiler & Open Source remains in progress.
Validation efforts continue to be a primary development focus.
Positive and negative validation suites continue to expand as compiler development progresses.


## **Version 1.0.1‑dev — August 16, 2026**  
*(Post‑1.0.0 milestone — Semantic Conformance + Compiler Infrastructure)*

### **Added — Semantic Conformance System (SC001–SC007)**  
A new top‑level specification layer defining how NXD programs must behave at the semantic level.  
This includes:

- SC001 — Symbol Resolution Rules  
- SC002 — Type Resolution & Primitive Compatibility  
- SC003 — Trait Conformance & IMPL Requirements  
- SC004 — Generic Instantiation Semantics  
- SC005 — Pattern Matching Exhaustiveness & Variant Correctness  
- SC006 — Ownership Semantics (MOVE / CLONE / BORROW)  
- SC007 — Cast Semantics (AS / IS) aligned with LG004  

This is now the authoritative reference for all compiler semantic checks.

---

### **Added — Full Semantic Subsystem (Compiler Implementation)**  
A complete semantic layer was implemented in the compiler core:

- `semantic/symbols.py` — hierarchical symbol table  
- `semantic/types.py` — type checking engine  
- `semantic/traits.py` — trait registry + conformance checks  
- `semantic/ownership.py` — MOVE/CLONE/BORROW validation  
- `semantic/casts.py` — AS/IS cast rules (LG004)  
- `semantic/patterns.py` — pattern validation scaffolding  
- `semantic/analyzer.py` — full semantic pass over IR  
- `semantic/errors.py` — structured semantic error types  

This is the largest single expansion of compiler capability since the initial parser.

---

### **Added — Rust Compiler Core Initialization**  
A new Rust crate was initialized for IR + backend + semantic:

- `Cargo.toml` created  
- `lib.rs` module tree established  
- IR nodes moved into Rust  
- Nim backend scaffolding connected  
- Semantic engine wired into Rust side  

This marks the beginning of the multi‑backend transpiler pipeline.

---

### **Improved — Lexer Stability & Token Specification (RO002)**  
Critical fixes applied to the Python lexer:

- Corrected STRING literal regex  
- Corrected LBRACK/RBRACK regex  
- Removed accidental newline breaks inside raw strings  
- Ensured full compliance with RO002 lexical rules  
- Verified uppercase identifier constraints  
- Verified lowercase literal constraints  

Lexer is now stable and ready for indentation tokenization.

---

### **Improved — Parser Infrastructure**  
Parser updated to align with LG003 grammar:

- TYPE bodies standardized to brace syntax  
- Typed parameters enabled (`X: int`)  
- Optional return types supported (`FUNC F(): int:`)  
- Lambda literal parsing (`fn(X) => EXPR`)  
- Map literal parsing (`{ "a": 1 }`)  
- Concurrency statements integrated (SPAWN/SEND/RECV/AWAIT)  
- Block parsing stabilized  
- Pattern parsing scaffolded  

Parser is now ready for full semantic integration.

---

### **Added — Project Packaging & Repo Structure**  
All required `__init__.py` files added across:

- `frontend/`  
- `semantic/`  
- `backend/`  
- `runtime/`  

This officially converts the compiler into a structured Python package.

---

### **Added — Scripts Directory Definition**  
A new `scripts/` folder was defined for:

- build automation  
- code generation tools  
- documentation sync utilities  
- test runners  
- release tooling  

This separates developer tooling from compiler runtime.

---

### **Documentation Updates**  
The language site received major updates:

- New **Semantic Conformance** section  
- Updated **Lexical Specification (RO002)**  
- Updated **Type System Specification (LG005)**  
- Updated **Type Conversion & Casting Semantics (LG004)**  
- Updated **Concurrency Model (RT002)**  
- Updated **Standard Library Layout (ES003)**  

Documentation now reflects the full compiler architecture.

---

## **Summary**  
NXD has progressed from a parsed language specification to an early-stage semantically validated compiler architecture.

Version 1.0.1-dev introduces formal semantic conformance, semantic analysis infrastructure, Rust compiler-core foundations, and backend pipeline scaffolding, establishing the framework required for future Nim, Elixir, and D transpilation targets.

---

