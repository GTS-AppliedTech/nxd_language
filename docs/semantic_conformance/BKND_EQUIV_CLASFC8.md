---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC001",
  "title": "Backend Equivalence Classification",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC001 Backend Equivalence Classification

## Purpose

This specification defines how NXD classifies backend equivalence and semantic portability.

The purpose of SC007 is to document which NXD operations are portable across backends, which require adaptation, which require runtime support, which are backend-specific, and which are unsupported by a particular backend.

This document separates semantic portability from semantic evidence. Portability classes describe the kind of backend support required. Evidence levels, defined in SC001, describe how strongly a semantic claim has been validated.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. Classification Levels

NXD SHALL use portability classes P1 through P7 for backend equivalence classification.

These classes replace letter-based A through E categories to avoid confusion with the E0 through E5 semantic evidence scale.

| Class | Name | Meaning |
|---|---|---|
| P1 | Fully Portable | Defined behavior is representable across all applicable conformant backends without semantic adaptation beyond ordinary lowering. |
| P2 | Adaptively Portable | Backend implementation differs, but preserves NXD semantics through target-native or generated mechanisms. |
| P3 | Constrained Portable | Portable only under documented constraints, restrictions, or subset rules. |
| P4 | Runtime-Assisted | Requires NXD runtime services, support libraries, generated wrappers, or compatibility infrastructure. |
| P5 | Backend-Specific | Valid NXD, but dependent on capabilities unavailable on all targets. |
| P6 | Native/Standalone | Requires a native NXD implementation or runtime to preserve full semantics. |
| P7 | Unsupported | Backend cannot provide the required semantics. |

A portability class is not an evidence level. A construct may be P2 with E1 evidence, or P4 with E5 evidence.

## 2. Relationship to Evidence Levels

Evidence levels are defined in SC001:

| Evidence Level | Meaning |
|---|---|
| E0 | Not evaluated |
| E1 | Structurally translatable |
| E2 | Observationally equivalent for the specified example |
| E3 | Semantically equivalent under defined constraints |
| E4 | Conformance-tested across applicable target outputs |
| E5 | Runtime-verified by an automated test suite |

A complete classification SHOULD include both a portability class and an evidence level.

Example:

```text
SPAWN: P2/P4, E4
```

This means SPAWN is adaptively portable or runtime-assisted, and has conformance-test evidence at E4.

## 3. Operation Classification Table

The following table is an initial working classification. It SHALL be revised as backend implementations and conformance tests mature.

| Operation or Construct | Initial Class | Notes |
|---|---:|---|
| Primitive arithmetic | P1 | Fully portable when overflow behavior is specified. |
| Boolean logic | P1 | Portable under defined truth-value semantics. |
| Basic records/structs | P1/P2 | Representation may differ, behavior should preserve field semantics. |
| Pattern matching | P2 | Requires backend adaptation where target lacks equivalent pattern semantics. |
| `Result<T, E>` | P2 | Requires consistent error/failure representation. |
| `Option<T>` | P2 | Requires typed option representation. |
| `none` | P2/P4 | Requires strict prevention of null/nil/uninitialized collapse. |
| MOVE | P3/P4 | Requires static analysis, runtime tracking, or restricted subset. |
| BORROW | P3/P4/P7 | Difficult on backends without borrow support; may be unsupported in some modes. |
| CLONE | P2/P3 | Portable when type clone policy is defined and implemented. |
| SPAWN | P2/P4 | Requires backend-specific concurrency mapping or runtime support. |
| SEND | P2/P4/P5 | Depends on mailbox and ownership transfer support. |
| RECV | P2/P4/P5 | Depends on mailbox, blocking, selective receive, and timeout semantics. |
| AWAIT | P2/P4 | Requires completion-state equivalence. |
| Mailbox selective receive | P4/P5/P7 | Native on BEAM-like backends, assisted or unsupported elsewhere. |
| Capabilities | P4/P5/P6 | Requires protected authority model. |
| Capability revocation | P4/P6 | Requires shared revocation state or native runtime authority. |
| Shared mutable state | P3/P4/P5 | Requires backend constraints or runtime support. |
| Actor-owned state | P2/P4/P5 | Natural on actor runtimes, adapted elsewhere. |
| Runtime reflection | P5/P7 | Backend-specific unless NXD defines a portable reflection model. |
| Native FFI | P5 | Valid but target-dependent. |
| Native NXD runtime services | P6 | Requires standalone NXD runtime or equivalent support. |

## 4. Backend Notes

### 4.1 Nim Equivalence

The Nim backend is expected to provide strong support for native compiled output, structured typing, option-like representations, and ARC/ORC-assisted ownership strategies.

Nim may require generated analysis, wrappers, or runtime support for full conformance in:

- MOVE consumed-state enforcement
- BORROW lifetime enforcement
- Mailbox semantics
- Selective receive
- Capability unforgeability
- Capability revocation
- Shared-state synchronization

### 4.2 D Equivalence

The D backend is expected to provide strong support for compiled output, systems-level control, RAII-oriented resource handling, references, and generated abstractions.

D may require generated support for:

- Full option semantics
- Borrow restriction enforcement
- Mailbox and actor abstractions
- Capability sealing and revocation
- Runtime-assisted shared-state consistency

### 4.3 Elixir Equivalence

The Elixir backend is expected to provide strong support for process-oriented concurrency, message passing, actor-owned state, and supervision patterns.

Elixir may require adaptation or runtime support for:

- Static ownership consumed-state enforcement
- Borrow-like semantics
- Non-raw option semantics
- Capability unforgeability outside ordinary atoms/maps
- Native mutable shared-state abstractions
- Value identity expectations for clone behavior

## 5. Conformance Requirements

### 5.1 What Must Be Fixed

If an operation is classified as P7 for a backend, the backend MUST reject the construct or operate in a documented restricted-conformance mode.

If an operation is classified as P3, the compiler MUST enforce the documented constraints.

If an operation requires P4 support, the required runtime or generated support MUST be present before the backend may claim conformance for that construct.

### 5.2 What Must Be Documented

Each backend SHALL document:

- Supported portability classes
- Unsupported constructs
- Restricted-conformance modes
- Runtime-assisted features
- Known semantic limitations
- Evidence level for each claimed feature

### 5.3 What Must Be Restricted

A backend MUST restrict or reject constructs that cannot preserve NXD semantics.

A backend MUST NOT silently lower a construct into target code that is syntactically valid but semantically incorrect.

## 6. Classification Format

Backend support tables SHOULD use the following format:

```text
<construct>: <portability-class>, <evidence-level>
```

Examples:

```text
Option<T>: P2, E4
none: P2/P4, E3
SPAWN: P2/P4, E4
BORROW: P3/P7, E1
Capability revocation: P4/P6, E0
```

When multiple classes are listed, the backend documentation MUST explain the conditions under which each class applies.

## 7. Conformance Tests

The SC007 conformance suite SHALL include tests and documentation checks for:

- Correct class assignment
- Evidence-level assignment
- Backend support matrix completeness
- Rejection of unsupported P7 constructs
- Enforcement of P3 constraints
- Availability of P4 runtime support
- Documentation of P5 backend-specific features
- Separation of portability claims from evidence claims
- Prevention of conformance claims based solely on successful compilation

## Normative Requirement

NXD backend equivalence SHALL be classified using portability classes P1 through P7 and evidence levels E0 through E5. A backend MUST NOT claim semantic equivalence for a construct unless both its portability class and supporting evidence level are documented.
