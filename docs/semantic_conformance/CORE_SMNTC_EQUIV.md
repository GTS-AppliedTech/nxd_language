---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC004",
  "title": "Core Semantic Equivalance",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC004 Core Semantic Equivalence

## Purpose

This specification defines semantic equivalence within the NXD ecosystem and establishes the requirements by which an implementation may claim conformance with the NXD language specification.

NXD is designed to support multiple implementation strategies, including the current Nim, D, and Elixir backends, as well as potential future standalone NXD runtime implementations. These implementations may differ substantially in memory management, execution model, scheduling, runtime architecture, generated representation, garbage collection, threading, process models, and platform-specific mechanisms.

Such differences are permitted provided that they do not alter the observable semantics guaranteed by NXD. The purpose of semantic conformance is therefore not to require identical implementations. It is to require equivalent language behavior.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

A conformant implementation MUST satisfy all applicable normative requirements defined by the NXD specification.

## 1. Definition of Semantic Equivalence

Two NXD implementations are semantically equivalent when, for the same valid NXD program and equivalent execution conditions, they preserve all applicable language-level guarantees defined by the NXD specification.

Semantic equivalence does not require:

- Identical generated source code
- Identical IR serialization
- Identical memory layout
- Identical runtime architecture
- Identical scheduling
- Identical optimization strategies
- Identical internal error mechanisms

Semantic equivalence requires preservation of specified observable behavior and language guarantees.

### 1.1 Observable Behavior

Observable behavior includes, where applicable:

- Return values
- Program output
- Generated files
- Persistent state changes
- Network transmissions
- Externally visible messages
- Process exit status
- Defined error results
- Defined exception behavior
- Capability-authorized operations
- Ownership outcomes
- Other externally observable effects explicitly defined by NXD

An implementation MUST NOT introduce an observable difference where the NXD specification defines the behavior as deterministic.

### 1.2 Type Behavior

The implementation MUST preserve:

- Type validity
- Type compatibility
- Inference rules
- Generic constraints
- Compile-time restrictions
- Pattern compatibility
- Defined conversion behavior

A backend MUST NOT accept an NXD program that the language specification defines as invalid solely because the target language permits the corresponding construct. Likewise, a backend MUST NOT reject a valid NXD construct solely because the target language lacks a direct syntactic equivalent, provided that the construct can be implemented through another permitted mechanism.

### 1.3 Error Behavior

Error-producing operations MUST preserve the NXD-defined distinction between:

- Ordinary values
- Option or absence
- Result-based failures
- Raised exceptions
- Process failures
- Cancellation
- Other explicitly defined failure states

A backend MAY implement these mechanisms using target-specific facilities, but MUST preserve their NXD-visible behavior.

### 1.4 Concurrency Behavior

Concurrency behavior MUST preserve all guarantees explicitly defined by NXD. These may include process isolation, message delivery, mailbox semantics, ordering guarantees, synchronization, task completion, failure propagation, and cancellation.

NXD does not require backend implementations to use identical concurrency primitives. An NXD process is a language-level abstraction, not necessarily an operating-system thread, runtime process, fiber, or BEAM process.

### 1.5 Evaluation Order

Expressions and statements MUST be evaluated according to NXD's defined evaluation rules. Where evaluation order is specified, backend implementations MUST preserve that order.

Where NXD explicitly permits nondeterminism, implementations MAY produce different valid outcomes within the defined nondeterministic domain. Permitted nondeterminism is not semantic nonconformance.

### 1.6 Side Effects

Side effects explicitly defined by NXD MUST occur according to the language specification. Backends MAY use different internal mechanisms, but MUST preserve the externally observable result.

## 2. The Language Specification and IR

The NXD Language Specification is the ultimate semantic authority. The NXD Intermediate Representation provides the canonical compiler-level representation through which those semantics are normalized and preserved.

```text
source semantics -> normalized IR semantics -> backend realization
```

A backend MUST NOT reinterpret source-level meaning independently of the language specification.

### 2.1 AST to IR Normalization Rules

Frontend implementations MUST normalize semantically equivalent source constructs into equivalent IR representations.

The IR SHOULD make semantically significant properties explicit rather than relying on backend inference. These may include:

- Resolved types
- Ownership state
- Control flow
- Error and effect boundaries
- Capability requirements
- Concurrency operations
- Side-effect boundaries

### 2.2 IR Invariants

The IR MUST preserve all information necessary for a conformant backend to reproduce NXD semantics. At minimum, applicable IR representations SHOULD explicitly represent:

#### Type State

- Resolved types
- Generic substitutions
- Conversions
- Constraints

#### Ownership State

- Ownership acquisition
- Ownership transfer
- Borrow lifetime
- Clone operations
- Consumed state

#### Control Flow

- Branches
- Loops
- Function returns
- Termination
- Exceptional control flow

#### Effects

- Mutation
- I/O
- Capability use
- External calls
- Concurrency operations

#### Failure State

- Result
- Option
- Raised errors
- Propagation
- Process-level failure where applicable

### 2.3 IR Evaluation Model

The IR evaluation model MUST define the ordering, ownership, error, and effect behavior necessary for all conformant backends to preserve language-level semantics.

## 3. Backend Obligations

### 3.1 What a Backend MUST Preserve

All conformant backends MUST preserve:

- Type semantics
- Ownership semantics
- Option semantics
- Result semantics
- Error semantics
- Capability restrictions
- Defined evaluation order
- Defined side effects
- Concurrency guarantees
- Other language guarantees applicable to the construct

### 3.2 What a Backend MAY Approximate

Backends MAY differ in:

- Memory layout
- Allocation strategy
- Runtime representation
- Garbage collection
- Reference counting
- Stack and heap organization
- Scheduling implementation
- Synchronization mechanisms
- Generated source structure
- Optimization strategy

Differences are permitted when they are not observable through NXD-defined semantics.

### 3.3 What a Backend MAY NOT Change

A backend MUST NOT change:

- Program meaning
- Ownership outcomes
- Capability authority
- Result semantics
- Option semantics
- Defined error behavior
- Defined side-effect behavior
- Any deterministic behavior required by the specification

### 3.4 Backend Adaptation

A backend MAY use native target-language constructs, generated support code, runtime services, compatibility libraries, compiler intrinsics, target-specific optimizations, or a combination of these mechanisms.

The mechanism used to implement an NXD feature does not determine its semantic definition. This permits a construct to be implemented differently across Nim, D, Elixir, or a future native NXD runtime while maintaining semantic conformance.

## 4. Non-Portable and Underspecified Behavior

Not every property of an execution environment is necessarily controlled by NXD. Examples may include:

- OS scheduling
- External network timing
- Filesystem latency
- Hardware-specific behavior
- Timer precision
- Resource exhaustion
- External service availability
- Physical device behavior

Where NXD does not define a particular behavior, implementations MAY differ. However, implementations MUST NOT treat an undefined implementation detail as permission to violate a behavior that NXD explicitly defines.

### 4.1 Nondeterministic Operations

Where NXD intentionally permits nondeterministic behavior, the specification SHALL define the permitted outcome space. A backend is conformant when its result falls within that defined space.

### 4.2 Unsupported Backend Features

A backend MUST NOT silently generate semantically incorrect code when it cannot implement an NXD construct.

If a construct cannot be supported while preserving its required semantics, the implementation SHALL:

1. Reject the construct
2. Identify the unsupported semantic requirement
3. Provide an appropriate diagnostic

A backend MAY provide a documented restricted-conformance mode where explicitly permitted by the specification.

## 5. Conformance Test Categories

The NXD semantic conformance suite SHALL include, where applicable:

- Type-system tests
- Evaluation-order tests
- Ownership tests
- MOVE, BORROW, and CLONE tests
- Option tests
- Result tests
- Error propagation tests
- Exception tests
- Capability tests
- Pattern-matching tests
- Concurrency tests
- Mailbox tests
- Shared-state tests
- Side-effect tests
- Boundary-condition tests
- Negative compilation tests

Tests SHOULD include both positive cases and negative cases.

A backend that produces correct output for valid programs but incorrectly accepts invalid programs is not fully conformant.

## 6. Semantic Audit Classification

Semantic evidence SHALL be classified using the NXD semantic audit scale:

| Level | Classification |
|---|---|
| E0 | Not evaluated |
| E1 | Structurally translatable |
| E2 | Observationally equivalent for the specified example |
| E3 | Semantically equivalent under defined constraints |
| E4 | Conformance-tested across applicable target outputs |
| E5 | Runtime-verified by an automated test suite |

An E-level represents the strength of available evidence, not an inherent quality ranking of the language construct.

A construct MUST NOT be described as fully conformant solely because it has achieved E1 or E2 evidence.

## 7. Conformance and Portability

Semantic conformance and portability SHALL be treated as related but distinct concepts.

A construct MAY be:

- Semantically valid NXD
- Fully implementable by the NXD runtime
- Partially supported by one or more external backends
- Unavailable on another backend

Such a construct does not become invalid NXD merely because a particular backend cannot implement it. This distinction permits NXD to evolve beyond the capabilities shared by its current target ecosystems.

## 8. Relationship to Other Semantic Conformance Specifications

SC001 establishes the general conformance model. The following specifications define specialized semantic domains:

- SC002 none and Option Semantics
- SC003 Move, Borrow, Clone Semantics
- SC004 Concurrency and Mailbox Semantics
- SC005 Capability Security Semantics
- SC006 Shared State and Ownership Semantics
- SC007 Backend Equivalence Classification

These specifications SHALL be interpreted consistently with SC001.

Where a specialized specification defines a more restrictive semantic requirement, the specialized requirement applies to that domain.

## Normative Requirement

A backend SHALL be considered semantically conformant only when it preserves all applicable semantic guarantees defined by the NXD Language Specification and represented by the NXD IR.

A backend MUST NOT claim conformance based solely on syntactic translation or successful compilation. Compilation success is not evidence of semantic equivalence. Runtime success for a single example is not, by itself, evidence of general semantic equivalence. Semantic claims SHALL be supported by the applicable NXD semantic audit level.
