---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC007",
  "title": "Shared State and Ownership Semantics",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC007 Shared State and Ownership Semantics

## Purpose

This specification defines NXD shared-state and ownership semantics across backends with different memory models.

Nim and D can represent shared mutable state directly through memory, references, locks, atomics, and runtime structures. Elixir and BEAM-style execution typically represent state through immutable values and process ownership. NXD must therefore define a unified language-level state model that backends implement through their native or generated mechanisms.

Shared mutable state is an implementation strategy. Ownership and permitted access are the semantic contract.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. What is Shared State in NXD?

Shared state is state that may be observed or affected from more than one logical execution context, module, process, task, or authority boundary.

Shared state MAY be represented as:

- Global state
- Module-local state
- Process-local state exposed through messaging
- Runtime-managed state
- Actor-owned state
- Synchronized mutable memory
- Capability-protected state

A backend's storage mechanism does not define NXD shared-state semantics.

### 1.1 Global State

Global state is accessible through a globally visible binding, service, runtime table, or module boundary according to the NXD language specification.

Global mutable state SHOULD be restricted, capability-controlled, or explicitly marked by the language.

### 1.2 Module-Local State

Module-local state is visible only within the module or module-defined authority boundary.

A backend MUST preserve module visibility rules even if the target language has a different module or visibility model.

### 1.3 Process-Local State

Process-local state is owned by one NXD process and accessed by other processes only through defined communication mechanisms.

For actor-style semantics, mutation of process-local state occurs through messages handled by the owning process.

## 2. Ownership Rules

Ownership determines who may control, mutate, transfer, or release a value.

### 2.1 Who Owns a Value?

Every ownership-sensitive value SHOULD have a defined owner at every semantically significant program point.

Owners MAY include:

- A local binding
- A structure field
- A module
- A process
- A runtime authority object
- A capability-protected resource

### 2.2 Who May Mutate a Value?

A value may be mutated only by an owner or by an authorized mutable access path.

NXD SHALL distinguish between:

- Owned mutation
- Borrowed mutable access
- Shared synchronized mutation
- Actor-mediated mutation
- Capability-authorized mutation

A backend MUST NOT allow mutation through an access path that NXD would reject.

### 2.3 How is Ownership Transferred?

Ownership transfer MUST follow SC005 MOVE semantics.

Ownership may be transferred through:

- Assignment
- Function call
- Return
- Message passing
- Capability handoff
- Runtime state transition

After ownership transfer, the previous owner MUST lose authority to use the value where the type is move-only or otherwise ownership-restricted.

## 3. State Models

NXD recognizes multiple implementation models, but defines one semantic contract.

### 3.1 Native Shared Mutable State

Nim and D backends MAY implement shared state through native memory, references, locks, atomics, channels, or runtime data structures.

Such implementations MUST preserve NXD access, synchronization, and ownership rules.

### 3.2 Actor-Owned State

An Elixir backend or actor-style runtime MAY implement shared state through process ownership.

In this model, state is not directly shared. Other processes interact with the state owner through messages.

Actor-owned state is conformant when it preserves the NXD-visible behavior of the shared-state abstraction.

### 3.3 Runtime-Assisted State

A backend MAY use generated runtime services to provide shared-state behavior where native target-language constructs are insufficient.

Runtime-assisted state MAY include:

- State servers
- Protected mutable cells
- Capability-guarded references
- Transaction-like wrappers
- Atomic operation libraries
- Synchronization adapters

## 4. Required Abstraction

NXD MUST define a unified state model independently of backend storage details.

Backends MUST implement the abstraction rather than exposing target-specific state behavior as NXD semantics.

### 4.1 Unified State Model

A shared-state abstraction SHOULD define:

- Owner
- Authorized readers
- Authorized writers
- Synchronization requirements
- Mutation ordering guarantees
- Visibility guarantees
- Failure behavior
- Capability requirements
- Transfer behavior

### 4.2 Backend Responsibilities

A backend MUST:

- Preserve ownership restrictions
- Preserve mutation restrictions
- Preserve visibility rules
- Preserve synchronization semantics where defined
- Reject unsupported state constructs rather than generating unsound behavior
- Document restricted-conformance behavior where allowed

## 5. Race Conditions

Where NXD defines deterministic state behavior, a backend MUST prevent target-level races from producing nondeterministic NXD-visible outcomes.

Where NXD permits nondeterminism, the specification SHALL define the permitted outcome space.

A backend MUST NOT treat incidental data races as permitted nondeterminism unless the NXD specification explicitly allows them.

## 6. Global State Consistency

If NXD permits global mutable state, the specification SHALL define:

- Initialization order
- Visibility across modules
- Visibility across processes or tasks
- Mutation ordering
- Synchronization requirements
- Shutdown or cleanup behavior

A backend MUST NOT rely on target-language global initialization behavior unless it matches the NXD specification.

## 7. Backend Mapping Table

| State Form | Nim | D | Elixir | Required NXD Behavior |
|---|---|---|---|---|
| Global state | module/global variable plus locks or runtime wrapper | module/global variable plus locks/atomics or runtime wrapper | application/process state wrapper | Defined initialization and mutation visibility |
| Module-local state | private module variable | private module/static state | module process or private state wrapper | Module visibility preserved |
| Process-local state | task/thread-local state or runtime process state | fiber/thread-local state or runtime process state | BEAM process state | Owned by one process |
| Shared counter | atomic or locked cell | atomic or locked cell | process-owned counter or runtime cell | Defined update and read semantics |
| Capability-protected state | authority object | sealed state handle | process-owned authority server | Access only through capability |

## 8. Conformance Tests

The SC006 conformance suite SHALL include tests for:

- Shared counter updates under concurrency
- Actor state isolation
- Rejection of unauthorized mutation
- Ownership transfer into shared state
- Ownership transfer out of shared state
- Mutation while borrowed rejection
- Global state initialization
- Global state consistency
- Module-local visibility
- Process-local isolation
- Capability-protected state access
- Race-condition boundaries
- Backend-specific unsound sharing prevention

## Normative Requirement

NXD SHALL define shared state through ownership, access, synchronization, and capability rules. A conformant backend MUST implement that abstraction rather than exposing incompatible native shared-state behavior as language semantics.
