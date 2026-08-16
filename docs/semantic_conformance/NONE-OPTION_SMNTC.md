---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC006",
  "title": "none and Option Semantics",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC006 none and Option Semantics

## Purpose

This specification formally defines `none` and `Option<T>` semantics in NXD.

The purpose of this document is to remove ambiguity between absence, null, uninitialized memory, invalid pointers, bottom values, and target-language sentinel values. `none` is one of the largest semantic risk areas in a multi-backend language because Nim, D, and Elixir provide different native mechanisms for representing absence.

NXD therefore defines `none` as a language-level value belonging only to the `Option<T>` domain.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. What is `none`?

`none` is the NXD representation of absence within an `Option<T>` value.

`none` is not:

- A null pointer
- An arbitrary sentinel value
- An uninitialized value
- An invalid memory location
- A default value for all types
- A magic value that can inhabit any type
- A raw backend nil/null where NXD has not explicitly wrapped it as an option

The canonical NXD model is:

```text
Option<T> = Some(T) | None
```

In NXD source-level notation:

```nxd
let user_name: Option<String> = none
let count: Option<Int> = some(10)
```

A value of type `T` is distinct from a value of type `Option<T>`.

## 2. Type Rules

### 2.1 `none` Has Type `Option<T>`

`none` may only appear where an `Option<T>` value is expected or where type inference can resolve the expected option type.

Formal rule:

```text
Gamma |- none : Option<T>
```

This means that `none` is valid only when the compiler can determine the `T` parameter.

Valid examples:

```nxd
let port: Option<Int> = none
let nickname: Option<String> = none
```

Invalid examples:

```nxd
let port: Int = none
let nickname: String = none
```

### 2.2 `none` Is Not a Valid Value for Arbitrary Types

A backend MUST NOT treat `none` as a value of any non-option type merely because the target language permits null-like or nil-like values.

The following is invalid NXD unless an explicit `Option<T>` type is present:

```nxd
let config = none
```

If the surrounding context cannot infer `Option<T>`, the compiler SHALL produce a diagnostic requiring an explicit type.

### 2.3 `none` Is Not Implicitly Castable

`none` MUST NOT be implicitly converted into:

- A null pointer
- A nullable reference
- An empty string
- Zero
- False
- An empty collection
- A backend default initializer
- An error value

Any conversion from `Option<T>` to `T` MUST proceed through explicit pattern matching, unwrapping, defaulting, or result-producing extraction according to the NXD language specification.

### 2.4 `some(T)` Preserves Value Semantics

`some(value)` wraps a valid value of type `T` into `Option<T>`.

A backend MUST NOT erase the distinction between `some(null_equivalent)` and `none` unless the NXD language specification explicitly permits such representation for that backend and no observable semantic difference is lost.

## 3. Backend Mapping

Backend mappings are implementation strategies. They do not define the semantics of `none`.

| Backend | Permitted Strategy | Requirement |
|---|---|---|
| Nim | `Option[T]` with `none(T)` or equivalent tagged option | MUST preserve option typing and pattern behavior |
| D | `Nullable!T`, algebraic type, or generated tagged union | MUST distinguish `none` from raw null or uninitialized state |
| Elixir | Tagged option form such as `{:some, value}` and `:none` | MUST NOT expose raw `nil` as general NXD `none` |
| Native NXD Runtime | Runtime option representation | MUST preserve the formal `Option<T>` model |

### 3.1 Nim Mapping

A Nim backend SHOULD map NXD `Option<T>` to a representation that preserves the distinction between `some(value)` and `none`. The preferred mapping is a native or generated option type, not an uninitialized variable.

### 3.2 D Mapping

A D backend MAY use `Nullable!T` where it preserves NXD semantics. If `Nullable!T` cannot preserve required behavior for a particular `T`, the backend SHALL use a tagged union or generated option representation.

### 3.3 Elixir Mapping

An Elixir backend MAY use internal BEAM values for representation, but MUST expose NXD `Option<T>` behavior at the language boundary.

A raw Elixir `nil` MUST NOT be treated as a universal substitute for NXD `none`. The backend SHOULD prefer a tagged representation such as:

```elixir
:none
{:some, value}
```

or an equivalent generated form.

## 4. Forbidden Mappings

The following mappings are forbidden unless a future specialized specification explicitly permits them under documented constraints.

### 4.1 Nim Uninitialized Values

A Nim backend MUST NOT map `none` to an uninitialized variable or memory state.

### 4.2 D Null Pointers

A D backend MUST NOT map `none` to a raw pointer null for a non-pointer `Option<T>` value.

### 4.3 Elixir Raw Nil

An Elixir backend MUST NOT expose raw `nil` as the general representation of NXD `none` without a typed option boundary.

### 4.4 Implicit Boolean or Numeric Absence

No backend MAY map `none` to `false`, `0`, an empty string, or an empty collection.

## 5. Pattern Matching

Pattern matching on `Option<T>` MUST distinguish the `some(value)` and `none` cases.

Example:

```nxd
match maybe_user {
  some(user) => user.name
  none => "anonymous"
}
```

A conformant implementation MUST reject incomplete option matches unless the NXD pattern-matching rules permit non-exhaustive matching for the surrounding context.

## 6. Option Unwrap Behavior

Unwrapping an `Option<T>` MUST be explicit.

A safe unwrap operation MUST define behavior for the `none` case through one of the following mechanisms:

- Pattern matching
- A default value
- Conversion to `Result<T, E>`
- Propagation through an option-aware operator
- A checked unwrap that produces a defined failure

An unchecked unwrap of `none`, if permitted by the language, MUST have explicitly defined failure behavior.

## 7. Error Propagation

`none` and `Result` failure are distinct semantic domains.

A backend MUST NOT collapse `none` into an error unless the NXD source construct explicitly converts an `Option<T>` into a `Result<T, E>`.

Example conceptual conversion:

```nxd
let value: Result<User, Error> = maybe_user.ok_or(UserMissing)
```

## 8. Conformance Tests

The SC002 conformance suite SHALL include tests for:

- `typeof(none)` with explicit `Option<T>` annotation
- Rejection of `none` assigned to non-option types
- Rejection of ambiguous `none` without inferable type
- Pattern matching on `some` and `none`
- Exhaustiveness checking for option matches
- Safe unwrap behavior
- Unchecked unwrap failure behavior where applicable
- Conversion from `Option<T>` to `Result<T, E>`
- Backend-specific prevention of raw null/nil leakage
- Negative compilation tests for implicit casts

## Normative Requirement

`none` SHALL be considered a valid NXD value only within the `Option<T>` semantic domain. A backend MUST preserve the distinction between absence, failure, null-like backend values, and uninitialized state.
