---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC005",
  "title": "Move Borrow Clone Semantics",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC005 Move Borrow Clone Semantics

## Purpose

This specification formalizes NXD ownership operations: MOVE, BORROW, and CLONE.

Ownership is one of the hardest semantic areas in NXD because Nim, D, and Elixir provide fundamentally different memory and value models. NXD ownership semantics are therefore defined by the language specification and represented in the IR. They are not defined by the backend memory model.

The backend's responsibility is to preserve the NXD-visible ownership behavior, even when the target language implements memory management differently.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. MOVE Semantics

MOVE transfers ownership of a value from one binding, location, or owner to another.

After a successful MOVE, the original binding enters a consumed state unless the type is explicitly defined as copyable or the operation is otherwise exempted by the NXD specification.

### 1.1 Binding Invalidation

A moved-from binding MUST NOT be used as though it still owns the moved value.

Conceptual model:

```text
Owned -> MOVE -> Consumed
Consumed -> use -> compile-time error or defined diagnostic
```

Example:

```nxd
let file = open_file("config.nxd")
let moved = move file
read(file) // invalid: file has been consumed
```

A conformant implementation SHALL reject invalid access after consumption where such access is statically detectable.

### 1.2 IR Rule: MOVE Creates a Consumed State

The IR MUST explicitly preserve ownership transition information sufficient for the backend to enforce or emulate the consumed state.

At minimum, applicable IR SHOULD identify:

- The source binding
- The target binding
- The ownership state before the move
- The ownership state after the move
- Whether the value type permits implicit copy
- Whether the move crosses an effect, process, or capability boundary

### 1.3 Backend Obligations for MOVE

Backends MUST preserve access invalidation after MOVE.

Backends MAY implement MOVE using:

- Native move semantics
- Reference-count transitions
- Generated state flags
- Static single-assignment transformation
- Compiler diagnostics
- Runtime checks where static enforcement is unavailable

A backend MUST NOT allow a moved value to be used in a way that violates NXD ownership semantics merely because the target language would allow such use.

## 2. BORROW Semantics

BORROW creates a non-owning access path to a value without transferring ownership.

Borrowing permits controlled access while preserving the original ownership relationship.

### 2.1 Non-Owning Reference

A borrow MUST NOT extend the lifetime of the borrowed value beyond the lifetime permitted by the NXD specification.

The borrower does not become the owner.

Conceptual model:

```text
Owned -> BORROW -> Borrowed view
Borrowed view expires -> Owned remains valid
```

### 2.2 Mutability Rules

NXD SHALL distinguish between immutable and mutable borrows.

An immutable borrow permits read-only access.

A mutable borrow permits mutation only when the language rules establish exclusive mutable access.

The compiler SHOULD prevent simultaneous access patterns that violate the borrow rules, including:

- Mutating through one path while an immutable borrow is active
- Creating multiple mutable borrows to the same value where exclusivity is required
- Moving a value while it is actively borrowed
- Allowing a borrow to outlive the owned value

### 2.3 Lifetime Rules

A borrow MUST NOT outlive the value being borrowed.

The IR SHOULD represent borrow lifetime boundaries explicitly enough for a backend to enforce them through static analysis or generated runtime checks.

### 2.4 Backend Approximations

Backends MAY approximate borrow semantics through:

- Static analysis
- Generated temporary variables
- Scoped references
- Read-only wrappers
- Copy-on-write behavior where explicitly permitted
- Runtime borrow tracking where needed

An approximation is valid only if it preserves NXD-visible behavior.

## 3. CLONE Semantics

CLONE creates a distinct value according to the clone rules defined for the type.

CLONE does not transfer ownership away from the source value.

Conceptual model:

```text
Owned value A -> CLONE -> Owned value A + Owned value B
```

### 3.1 Deep Copy

By default, CLONE SHOULD produce a semantically independent value unless the type definition specifies shared internals or reference-preserving clone behavior.

For security-sensitive, capability-bearing, or ownership-sensitive values, CLONE MUST follow the type's explicit clone policy.

### 3.2 Structural Clone Rules

For structural values, clone behavior SHOULD be defined recursively over fields:

- Primitive fields are copied according to their value semantics
- Owned fields are cloned according to their type-specific clone policy
- Borrowed fields MUST NOT be cloned into invalid outliving references
- Capability fields MUST NOT be cloned unless explicitly allowed

### 3.3 Backend Implementations

Backends MAY implement CLONE through:

- Native copy constructors
- Generated deep-copy functions
- Serialization/deserialization where explicitly permitted
- Reference-counted sharing where observationally equivalent
- Runtime clone hooks

A backend MUST NOT treat CLONE as a shallow copy when the NXD type requires a deep or authority-preserving clone.

## 4. Backend Mapping Table

| Operation | Nim | D | Elixir | Required NXD Behavior |
|---|---|---|---|---|
| MOVE | ARC/ORC plus static analysis or generated consumed-state checks | Native move facilities, scope rules, or generated state tracking | Semantic hint plus generated invalid-use prevention where possible | Source binding becomes consumed |
| BORROW | References, views, or generated wrappers | References, scope rules, or generated wrappers | Read-only semantic view, copied data, or process-owned access pattern | Borrow does not transfer ownership |
| CLONE | Copy/deep-copy routine or generated clone | Copy constructor, postblit/copy, or generated clone | New immutable value or generated structural copy | Source remains valid and clone policy is preserved |

## 5. Aliasing Rules

NXD ownership semantics MUST prevent aliasing patterns that violate defined mutation, capability, or lifetime rules.

A backend MUST NOT allow unsafe aliasing merely because the target language makes aliasing easy or implicit.

## 6. Capability Interaction

Capability-bearing values are subject to the capability rules in SC005.

A capability-bearing value MUST NOT be cloned, copied, borrowed mutably, or transferred across process boundaries unless the capability's construction and transfer policy explicitly permits the operation.

## 7. Conformance Tests

The SC003 conformance suite SHALL include tests for:

- MOVE followed by use
- MOVE into function parameter
- MOVE across process or task boundary
- BORROW read access
- BORROW plus mutation rejection
- Mutable borrow exclusivity
- Borrow lifetime expiration
- Move while borrowed
- CLONE identity separation
- CLONE deep-copy behavior
- CLONE of nested structures
- Capability-bearing clone rejection
- Backend-specific aliasing prevention
- Negative compilation tests for invalid ownership use

## Normative Requirement

MOVE, BORROW, and CLONE SHALL be treated as semantic operations, not backend implementation suggestions. A conformant backend MUST preserve NXD ownership behavior independently of the target language's memory-management model.
