{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "TS001",
  "title": "TS001 Test Suite Philosophy and Methodology",
  "description": "Test Suite Overview",
  "layer": "Test Suite",
  "category": "",
  "keywords": [],
  "doc_version": "1.0",
  "status": "active"
}


# TS001 Test Suite Philosophy and Methodology

### Overview

The NXD test suite is used to validate language semantics, compiler behavior, IR generation, semantic validation, and backend transpilation. Testing is performed incrementally, allowing issues to be isolated to specific language features and compiler stages.

The NXD project treats testing as a first-class development activity. Conformance testing and transpilation validation are explicitly identified as development milestones for the compiler.

### Test Suite Goals

The NXD test suite exists to verify:

Parser correctness
Semantic validation correctness
IR generation correctness
Backend code generation correctness
NXD → Nim transpilation correctness
Future NXD → D transpilation correctness
Future NXD → Elixir transpilation correctness
Regression prevention

A passing test demonstrates that a specific language construct can successfully move through the compiler pipeline.

### Testing Philosophy

NXD uses an incremental testing strategy.

Each successive test introduces a small amount of new functionality while reusing functionality already validated by previous tests.

Example:

```nxd
MODULE TEST

FUNC MAIN():
    PRINTLN("hello")
```

A later test might become:

```nxd
MODULE TEST

FUNC MAIN():
    LET X SET 10
    PRINTLN(X)
```

The second test validates:

Variable binding
Identifier resolution
Expression handling

while continuing to validate functionality already proven by the first test.

This approach makes root-cause identification significantly easier because newly introduced syntax becomes the primary suspect when a test fails.

### Positive Tests

Positive tests contain code that is expected to compile successfully.

#### Purpose

Positive tests verify that supported language features:

Parse correctly
Pass semantic validation
Generate valid IR
Produce valid backend output

#### Expected Result

The test should:

Parse successfully
Pass semantic validation
Generate IR
Generate backend code
Produce output consistent with NXD semantics

#### Example

```nxd
MODULE TEST

FUNC MAIN():
    LET NAME SET "gabriel"
    PRINTLN(NAME)
```

**Expected outcome:**

PASS

### Negative Tests

Negative tests contain code that is expected to fail.

#### Purpose

Negative tests verify that invalid programs are rejected.

These tests ensure the compiler does not silently accept incorrect code.

#### Expected Result

The test should:

Produce a compiler error
Produce a semantic error
Reject invalid syntax
Reject invalid type usage

#### Example

```nxd
MODULE TEST

FUNC MAIN():
    LET X SET UNKNOWN_IDENTIFIER
```

**Expected outcome:**

FAIL


The compiler should report the error rather than generating backend code.

### Compiler Stages Being Tested

A test may fail in several different locations within the compiler pipeline.

#### Parser Stage

**Validates:**

Syntax structure
Tokenization
Grammar rules

**Example failures:**

Missing parentheses
Invalid keywords
Malformed statements

#### Semantic Validation Stage

**Validates:**

Identifier resolution
Scope rules
Type rules
Ownership rules
Language semantics

**Example failures:**

Undefined identifiers
Type mismatches
Invalid ownership operations

#### IR Generation Stage

**Validates:**

Lowering from AST to IR
Semantic representation
Compiler intermediate structures

A test may successfully parse and semantically validate but still fail during IR generation.

#### Backend Generation Stage

**Validates:**

NXD → Nim generation
Future NXD → D generation
Future NXD → Elixir generation

NXD's architecture defines the IR as the canonical semantic representation used before backend emission.

### Reading Test Results

When reviewing a failing test, identify the earliest failing stage.

**Example**

Parser: PASS
Semantic: FAIL
IR: NOT RUN
Backend: NOT RUN


The problem is semantic validation.

Investigating backend generation would be a waste of time because execution never reached that stage.

### Root Cause vs Secondary Symptoms

A single bug may cause multiple test failures.

Example:

Test 9: FAIL
Test 10: FAIL
Test 11: FAIL
Test 12: FAIL


This does not necessarily indicate four separate defects.

Often a single parser or semantic issue causes downstream failures across numerous tests.

The recommended workflow is:

Group failures by common behavior.
Identify the earliest failing feature.
Fix the root cause.
Re-run the entire suite.

### Recommended Testing Workflow

#### Step 1

Run all positive tests.

Document:

Passes
Failures

#### Step 2

Group failures into categories.

Examples:

Parser failures
Semantic failures
IR failures
Backend failures

#### Step 3

Identify the likely root cause.

Prioritize:

Parser problems
Semantic problems
IR problems
Backend problems

#### Step 4

Implement fixes.

#### Step 5

Re-run the full suite.

Never assume a fix only affects one test.

A successful fix may resolve an entire failure group.

### Regression Testing

All previously passing tests must be re-run after any compiler modification.

A new feature is not considered complete until:

Existing tests still pass
No new failures are introduced

Regression testing prevents accidental breakage of previously validated functionality.

### Conformance Validation

The long-term purpose of the test suite is not merely to validate generated code.

The primary goal is to validate NXD semantics.

Backend implementations must preserve semantic behavior across supported targets. NXD's backend requirements specify preservation of type semantics, ownership semantics, error semantics, evaluation order, side effects, concurrency guarantees, and other language guarantees.

Therefore:

NXD Source
    ↓
IR Generation
    ↓
Semantic Validation
    ↓
Backend Generation
    ↓
Backend Output


Every stage must preserve the semantic meaning of the original NXD source.

### Success Criteria

A test is considered fully successful only when:

Parsing succeeds
Semantic validation succeeds
IR generation succeeds
Backend generation succeeds
Generated output accurately reflects NXD semantics

Passing backend compilation alone is not sufficient.

The output must also be semantically correct.

### Summary

The NXD test suite is designed to:

Validate language semantics
Verify compiler correctness
Prevent regressions
Identify root causes efficiently
Ensure backend conformance
Support long-term language evolution

The most effective approach is to focus on root causes, not individual failures. One fix often resolves multiple tests, making systematic analysis far more effective than addressing failures one at a time.