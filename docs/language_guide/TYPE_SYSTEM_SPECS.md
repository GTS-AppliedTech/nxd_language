---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "LG005",
  "title": "",
  "description": "",
  "layer": "language guide",
  "category": "language guide",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# LG005 type system specification

1. Primitive types

Set:

• int — machine integer (backend‑mapped: Nim int, D int, Elixir integer)
• float — double precision (Nim float64, D double, Elixir float)
• bool — true / false
• string — UTF‑8 string
• none — absence of value (Nim nil/option, D null, Elixir nil)


2. Structs

Form:

TYPE PERSON { NAME: string, AGE: int }


Semantics:

• Product type: all fields present.
• Fields are typed, no implicit any.
• Backend:• Nim: object
• D: struct
• Elixir: defstruct or map


3. Enums

Form:

TYPE COLOR ENUM { RED, GREEN, BLUE }


Semantics:

• Closed set of named variants.
• No payloads (payloads → sum types).


Backend:

• Nim: enum
• D: enum
• Elixir: atoms (:red, :green, :blue)


4. Sum types (tagged unions)

Form:

TYPE RESULT UNION { OK(string), ERR(int) }


Semantics:

• Exactly one variant active.
• Pattern matching is the primary consumption mechanism.
• Backend:• Nim: object with case field or Result[T] pattern.
• D: Algebraic!(...) or tagged union.
• Elixir: tagged tuples ({:ok, v}, {:error, e}).


5. Option types

Canonical:

TYPE OPTION UNION { SOME(any), NONE }


Semantics:

• Represents presence/absence.
• Preferred over none for typed absence.
• Backend:• Nim: Option[T] or ref T.
• D: Nullable!T or Result!T with none.
• Elixir: {:some, v} / :none or nil.


6. Result types

You already started this:

TYPE RESULT UNION { OK(any), ERR(string) }


Semantics:

• Primary error channel.
• Encouraged over exceptions for recoverable errors.
• Backend:• Nim: Result[T].
• D: Result!T.
• Elixir: {:ok, v} / {:error, msg}.


7. Traits / interfaces / protocols

Form:

TRAIT SERIALIZABLE { FUNC TO_STRING(X): string }


Semantics:

• Behavioral contracts.
• No state.
• Backend:• Nim: concept.
• D: interface or template constraints.
• Elixir: protocol.


8. Generics

Form:

TYPE BOX<T> { VALUE: T }

FUNC ID<T>(X: T): T:
    RETURN X


Semantics:

• Parametric polymorphism.
• Monomorphization at backend level (Nim/D).
• Protocol‑style behavior in Elixir.


9. Type inference

Rules (initial):

• Local inference only:• LET X SET 10 → X: int
• LET S SET "hi" → S: string

• Function return types:• Explicit preferred.
• Inference allowed when unambiguous.


10. Ownership & mutability

Mutability:

• LET → mutable binding.
• CONST → immutable binding.


Ownership (first pass):

• MOVE → transfer ownership (semantic hint; backend‑specific).
• CLONE → deep copy.
• BORROW → temporary reference.
