{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "TS002",
  "title": "TS002 Test Suite User Guide",
  "description": "How to use the test suite",
  "layer": "Test Suite",
  "category": "",
  "keywords": [],
  "doc_version": "1.0",
  "status": "active"
}

# TS002 Test Suite User Guide
### Purpose

The NXD Test Suite User Guide explains the structure of the NXD testing framework and provides guidance on selecting the appropriate testing level for a given task.

This document is intended for:

Compiler developers
QA testers
Contributors
Auditors

*For detailed command syntax, see* **TS003.**

*For test numbering rules, see* **TS004.**

### Test Suite Overview

The NXD test suite is divided into four testing levels.

Each testing level validates a different portion of the compiler pipeline.

The suite supports the following backend targets:

**Nim**
**D**
**Elixir**

The testing architecture allows developers to isolate failures within individual compiler subsystems while also supporting complete end-to-end validation.

#### Test Level Summary
Level	Purpose	OutputLevel 1	Frontend Validation	JSON
Level 2	Python → Rust Handoff Validation	JSON + Backend File
Level 3	Full Pipeline Validation	JSON + Backend File
Level 4	Bulk Pipeline Validation	Multiple JSON + Backend Files

### Level 1 Testing
#### Purpose

Level 1 validates the Python frontend.

The test validates:

Lexer
Parser
AST generation
AST serialization

The test ends before Rust processing begins.

Success Criteria

A serialized JSON file is successfully generated.

Example:

serialized_ast.json

Recommended Usage

Use Level 1 when debugging:

Lexer issues
Parser issues
AST issues
Serialization issues

This level produces minimal noise and is intended for frontend diagnostics.

### Level 2 Testing
#### Purpose

Level 2 validates the Python-to-Rust handoff.

The test validates:

Serialized JSON generation
Rust handoff
AST transfer
Backend lowering

NXD semantic validation has not yet occurred.

Success Criteria

Produces:

serialized_ast.json

backend_output.*

Recommended Usage

Use Level 2 when debugging:

Rust integration
AST transfer issues
Lowering issues
Backend generation prior to semantic validation

### Level 3 Testing
#### Purpose

Level 3 executes the complete NXD compilation pipeline.

Pipeline:

NXD Source
  ↓
Lexer
  ↓
Parser
  ↓
AST
  ↓
JSON Serialization
  ↓
Rust Handoff
  ↓
NXD Semantics
  ↓
Backend Generation

Level 3 is the primary test level used for validating individual tests.

#### Level 3 Positive Testing

Positive tests contain valid NXD code.

Expected Result:

PASS


Produces:

serialized_ast.json

backend_output.*

#### Level 3 Negative Testing

Negative tests contain intentionally invalid NXD code.

Expected Result:

FAIL

The compiler should reject the test.

Negative Test Failure Condition

If a negative test successfully reaches backend generation and produces:

serialized_ast.json

backend_output.*


the negative test is considered **failed** because invalid code was accepted.

### Level 4 Testing
#### Purpose

Level 4 executes bulk batches of Level 3 tests.

Level 4 provides rapid pass/fail status determination across many tests.

#### Recommended Usage

Use Level 4 when:

Running multiple tests
Checking overall compiler status
Determining failure locations before detailed investigation
Batch Size Recommendation

Recommended:

10 tests or fewer per batch

Larger batches increase analysis complexity and troubleshooting time.

#### Positive Testing Workflow

Recommended process:

Run Level 4 Positive
  ↓
Identify Failing Tests
  ↓
Run Level 3 Positive
  ↓
Debug Individual Failures

#### Negative Testing Workflow

Recommended process:

Run Level 4 Negative
  ↓
Identify Unexpected Passes
  ↓
Run Level 3 Negative
  ↓
Investigate Acceptance Path

Choosing the Correct Test Level
Scenario	Recommended LevelParser Failure	Level 1
AST Failure	Level 1
Rust Handoff Failure	Level 2
Lowering Failure	Level 2
Semantic Failure	Level 3
Backend Generation Failure	Level 3
Bulk Compiler Validation	Level 4