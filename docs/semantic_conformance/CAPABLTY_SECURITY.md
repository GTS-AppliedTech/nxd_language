---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC002",
  "title": "Capability Security Semantics",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC002 Capability Security Semantics

## Purpose

This specification defines the semantic model for NXD capabilities.

The purpose of this document is to distinguish true capability security from API shapes that merely resemble capabilities. In NXD, a capability is authority. It is not just a permission string, identifier, role name, marker object, or convention.

Capability semantics are part of the language contract and MUST be preserved by all conformant implementations.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. What is a Capability?

A capability is an authority-bearing value that grants access to a specific operation, resource, or effect.

A capability MUST be treated as:

- An unforgeable token or authority object
- A value whose possession grants defined authority
- A value whose construction is controlled
- A value whose transfer, borrowing, cloning, and revocation are governed by explicit rules

A capability MUST NOT be treated as:

- A string permission name
- A role label
- A user identifier
- A public enum value standing in for authority
- A boolean flag
- A backend-only convention

### 1.1 Unforgeable Token

A capability is unforgeable when ordinary NXD code cannot create a valid authority-bearing value except through an authorized construction path.

A backend MUST NOT allow a user program to synthesize a valid capability by constructing a matching shape, record, struct, map, atom, integer, string, or pointer.

### 1.2 Authority Object

A capability carries authority directly or indirectly through protected runtime state.

A backend MUST preserve the distinction between possessing authority and merely naming authority.

### 1.3 Non-Copyable Unless Explicitly Allowed

By default, capabilities SHOULD be non-copyable and non-cloneable unless the capability type explicitly permits copying or cloning.

If a capability may be cloned, the clone semantics MUST specify whether the clone shares revocation state, authority scope, audit identity, lifetime, or transfer limits.

## 2. Capability Construction Rules

Only privileged code may create capabilities.

The NXD specification SHALL define privileged construction contexts, which may include:

- Runtime internals
- Compiler-generated trusted code
- Module-level authority factories
- Explicitly trusted system libraries
- Host integration boundaries

### 2.1 Authorized Construction

A capability constructor MUST be inaccessible to ordinary code unless explicitly exposed by the capability's authority model.

### 2.2 Anti-Forgery Requirement

A backend MUST NOT represent a capability in a way that allows ordinary code to forge one by constructing an equivalent value.

### 2.3 Authority Narrowing

Capability derivation MAY create narrower capabilities from broader capabilities when explicitly permitted.

Example conceptual model:

```text
FileSystemCapability -> ReadOnlyDirectoryCapability -> ReadFileCapability
```

A derived capability MUST NOT grant broader authority than the source capability.

## 3. Transfer, Borrow, and Clone Rules

Capabilities interact with SC003 ownership semantics.

### 3.1 Transfer

Transferring a capability MAY transfer authority from one owner to another. If the capability is move-only, the source binding MUST become consumed after transfer.

### 3.2 Borrow

Borrowing a capability MAY permit temporary authority use without transferring ownership. Borrowed capabilities MUST NOT outlive their owner or escape the permitted scope.

### 3.3 Clone

Cloning a capability is forbidden unless explicitly allowed by the capability type.

If cloning is allowed, the specification MUST define whether revocation applies to all clones or only the cloned handle.

## 4. Revocation Semantics

Capability revocation invalidates authority according to the capability's revocation policy.

### 4.1 Revocation Must Invalidate All Copies Where Required

If a capability type is revocable and cloneable, revocation MUST identify whether it invalidates:

- The current handle only
- All handles sharing the same authority
- A subtree of derived capabilities
- All derived capabilities
- A runtime-managed authority object

For security-sensitive capabilities, revocation SHOULD invalidate all authority-equivalent handles unless the specification explicitly defines a narrower revocation model.

### 4.2 Backend Implementation Strategies

Backends MAY implement revocation through:

- Runtime authority tables
- Reference-counted authority objects
- Capability IDs backed by protected tables
- Sealed structs with hidden constructors
- Process-owned authority servers
- Linear or affine ownership rules
- Generated validation checks

The representation MUST NOT allow revoked authority to remain usable through stale copies.

## 5. Backend Mapping

| Backend | Possible Mapping | Required Behavior |
|---|---|---|
| Nim | ref-counted authority objects, hidden constructors, runtime tables | Prevent forgery and preserve revocation |
| D | RAII, sealed structs, private constructors, runtime authority tables | Prevent raw reconstruction and unsafe copying |
| Elixir | process-owned capabilities, opaque tokens backed by authority servers | Prevent raw atom/map forgery and preserve authority checks |
| Native NXD Runtime | Native capability table or authority object model | Preferred long-term semantic baseline |

## 6. Capability Leakage

A backend MUST NOT leak capability authority through:

- Debug representation
- Serialization
- Public fields
- Reflection
- Pattern matching over hidden state
- Unrestricted copying
- Target-language escape hatches
- Untyped foreign-function boundaries

Where a capability crosses an external boundary, the boundary MUST either preserve the capability security model or reject the operation.

## 7. Privilege Escalation

A backend MUST NOT broaden capability authority during lowering, optimization, runtime wrapping, serialization, message passing, or error handling.

A capability for one resource MUST NOT become a capability for a broader resource unless the NXD specification explicitly permits authority derivation.

## 8. Conformance Tests

The SC005 conformance suite SHALL include tests for:

- Authorized capability construction
- Rejection of forged capabilities
- Copy rejection for non-copyable capabilities
- Clone rejection for non-cloneable capabilities
- Explicitly allowed clone behavior
- Revocation of original handle
- Revocation of cloned or derived handles
- Authority narrowing
- Privilege escalation prevention
- Capability leakage through serialization or debugging
- Capability transfer through MOVE
- Capability borrow lifetime behavior
- Capability message-passing restrictions
- Backend-specific construction escape prevention

## Normative Requirement

A capability SHALL be treated as authority. A conformant backend MUST preserve unforgeability, transfer restrictions, clone restrictions, revocation behavior, and authority boundaries for all NXD capability-bearing operations.
