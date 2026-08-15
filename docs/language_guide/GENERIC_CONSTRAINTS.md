---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "LG002",
  "title": "",
  "description": "",
  "layer": "language guide",
  "category": "language guide",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# LG002 Generic Constraints Specification

(Traits, constraint resolution, multiple constraints, backend mapping, semantics)

NXD generics are built on nominal trait constraints.
This ensures consistent semantics across Nim, Elixir, and D while preserving strong type guarantees and predictable lowering.


1. Constraint kinds

NXD supports three kinds of constraints:

1. Trait constraints

The most common form:

T : SERIALIZABLE
T : ORDERED
T : HASHABLE


These require the type to explicitly implement the trait.

2. Type constraints

Restrict a type parameter to a specific type:

T : int
T : string


Useful for overload resolution or specialization.

3. Composite constraints

Multiple constraints on the same type parameter:

T : SERIALIZABLE, ORDERED


All constraints must be satisfied.


2. Constraint syntax

On types

TYPE BOX<T : SERIALIZABLE>


On functions

FUNC SORT<T : ORDERED>(LIST<T>): LIST<T>


Multiple constraints

FUNC F<T : SERIALIZABLE, ORDERED>(X: T): T


Where‑clauses (optional extension)

FUNC F<T>(X: T) WHERE T : SERIALIZABLE, ORDERED:
    ...


3. Trait definition

Traits define behavioral contracts.

TRAIT ORDERED {
    FUNC COMPARE(A, B): int
}


Traits may contain:

• function signatures
• associated types (future extension)
• default implementations (future extension)


Traits may not contain:

• fields
• state
• constructors


Traits are purely behavioral.


4. Trait implementation

Types implement traits explicitly:

TYPE POINT IMPLEMENTS SERIALIZABLE:
    X: float
    Y: float

FUNC TO_STRING(P: POINT): string:
    RETURN "(" ADD P.X ADD "," ADD P.Y ADD ")"


A type may implement multiple traits.


5. Constraint resolution rules

Rule 1 — All constraints must be satisfied

If a generic function declares:

FUNC F<T : A, B>(X: T)


Then T must implement both A and B.

Rule 2 — Constraint failure is a compile‑time error

Example:

FUNC SORT<T : ORDERED>(LIST<T>): LIST<T>

LET L SET [1, 2, 3]
SORT(L)   # error: int does not implement ORDERED


Rule 3 — Constraint resolution happens before lowering

NXD resolves constraints at the IR level, not backend level.

Rule 4 — Constraints apply to:

• type parameters
• function parameters
• return types
• struct fields (future extension)


6. Nominal vs structural constraints

NXD uses nominal constraints

A type must explicitly declare:

TYPE X IMPLEMENTS TRAIT


Why nominal?

Because your backends differ:

• Nim → structural concepts
• Elixir → nominal protocols
• D → structural templates + nominal interfaces


Nominal constraints give you:

• consistent semantics
• predictable lowering
• clear auditability
• backend‑agnostic behavior


7. Backend mapping

Nim

Traits map to concepts or typeclass‑like constraints.

concept Serializable
  toString(x: T): string


Elixir

Traits map to protocols.

defprotocol Serializable do
  def to_string(x)
end


D

Traits map to:

• interface
• template constraints
• static if blocks


interface Serializable {
    string toString();
}


8. Constraint inheritance

If a trait extends another trait:

TRAIT A { ... }
TRAIT B IMPLEMENTS A { ... }


Then:

• any type implementing B automatically satisfies A
• constraint resolution treats B : A as satisfied


9. Constraint specialization

NXD allows specialized implementations:

FUNC HASH<T : HASHABLE>(X: T): int
FUNC HASH(X: int): int   # specialization


Resolution rules:

1. Check trait constraints first
2. Check type‑specific overloads second
3. Ambiguity → compile‑time error


10. Constraint visibility

Traits follow module visibility rules:

• public traits can be used anywhere
• internal traits only within the package
• private traits only within the module


11. Constraint error messages

NXD must provide clear diagnostics:

Type POINT does not implement trait ORDERED
Required by SORT<T : ORDERED>


12. Examples

Example 1 — Simple constraint

TRAIT SERIALIZABLE {
    FUNC TO_STRING(X): string
}

FUNC PRINT<T : SERIALIZABLE>(X: T):
    PRINTLN(TO_STRING(X))


Example 2 — Multiple constraints

FUNC SAVE<T : SERIALIZABLE, HASHABLE>(X: T):
    LET S SET TO_STRING(X)
    LET H SET HASH(X)
    WRITE(S, H)


Example 3 — Trait inheritance

TRAIT A { FUNC F(X): int }
TRAIT B IMPLEMENTS A { FUNC G(X): int }

TYPE T IMPLEMENTS B:
    ...

# T satisfies both A and B


13. Summary Table

Feature	Rule	
Constraint type	Nominal traits	
Multiple constraints	Allowed	
Resolution	Compile‑time	
Failure	Compile‑time error	
Backend mapping	Concepts / Protocols / Interfaces	
Trait inheritance	Supported	
Specialization	Supported	
Structural typing	Not used	
