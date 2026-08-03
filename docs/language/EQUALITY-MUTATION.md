EQUALITY-MUTATION.md

NXD Equality & Mutation Semantics Specification

NXD defines strict, explicit, and predictable rules for equality and mutation across all value categories: primitives, structs, collections, channels, processes, tasks, and bindings.

This chapter closes the remaining semantics gaps.


1. Equality Semantics

NXD defines two kinds of equality:

• Value equality — compares contents
• Identity equality — compares runtime identity


1.1 Primitive Equality (value)

Primitives use value equality:

• int — numeric equality
• float — IEEE‑754 equality (NaN EQ NaN is false)
• string — byte‑wise equality
• bool — true/false
• enum — tag equality


1.2 Struct Equality (value)

Structs use field‑wise value equality.

Two structs are equal if:

• they have the same type
• all fields are equal by value


Example:

TYPE POINT { X: int, Y: int }

POINT(1,2) EQ POINT(1,2)   # true
POINT(1,2) EQ POINT(2,1)   # false


1.3 List Equality (value)

Lists are equal if:

• same length
• each element equal in order


1.4 Map Equality (value)

Maps are equal if:

• same key set
• each key’s value equal


Order does not matter.

1.5 Channel Equality (identity)

Channels use identity equality:

• same underlying runtime channel → equal
• otherwise → not equal


1.6 Process Equality (identity)

Processes use identity equality:

• same process ID → equal
• otherwise → not equal


1.7 Task Equality (identity)

Tasks use identity equality:

• same task handle → equal
• otherwise → not equal


2. Mutation Semantics

NXD defines three binding forms:

• LET — mutable binding
• CONST — immutable binding
• IMMUTABLE (future extension) — immutable value graph


2.1 LET (mutable binding)

LET X SET 10
X SET 20   # legal


• Binding can be reassigned
• Value may be mutated (if mutable type)


2.2 CONST (immutable binding)

CONST PERSON
PERSON.NAME SET "x"


Rule:

CONST freezes the binding, not the value graph.

Meaning:

• PERSON SET ... is illegal
• PERSON.NAME SET ... is legal
• PERSON.ADDRESS.ZIP SET ... is legal


CONST does not make the object immutable — only the variable.

2.3 Value mutability

NXD core types:

• Structs — mutable
• Lists — mutable
• Maps — mutable
• Channels — mutable (stateful)
• Processes — mutable (stateful)
• Tasks — immutable (result only)


3. Collection Mutation Semantics

3.1 Lists

Lists are mutable, reference‑semantics:

LET A SET [1,2,3]
LET B SET A
B[0] SET 99

A EQ [99,2,3]   # true


3.2 Maps

Maps are mutable, reference‑semantics:

LET M SET { "x": 1 }
LET N SET M
N["x"] SET 42

M["x"] EQ 42   # true


3.3 Copying

• LET A SET B → copies reference
• CLONE(B) → deep copy


4. Trait Object Semantics

You asked:

LET X : SERIALIZABLE


Rule:

Traits are compile‑time only. They do not exist as runtime values.

Thus:

• LET X : SERIALIZABLE → illegal
• LET X : USER where USER IMPLEMENTS SERIALIZABLE → legal


Dynamic trait objects (runtime vtables) may be added later via:

BOX<SERIALIZABLE>


5. Diagnostic Specification (summary)

NXD uses structured diagnostic codes:

Errors (E‑codes)

• E1001 — Type mismatch
• E2001 — Constraint violation
• E3001 — Channel closed
• E3002 — Channel full
• E4001 — Capability revoked
• E4002 — Capability not delegable
• E5001 — Non‑exhaustive match
• E5002 — Invalid cast (AS)


Warnings (W‑codes)

• W1001 — Unused import
• W1002 — Unused variable
• W2001 — Shadowed binding


6. Backend Capability Matrix

Feature	Nim	Elixir	D	
RESULT	native	emulated	native	
OPTION	native	emulated	native/Nullable	
MOVE	ARC/ORC analysis	hint only	RAII/hint	
BORROW	hint	N/A (immutable heap)	hint	
CHANNEL	native	wrapped (GenServer/mailbox)	native	
UNSAFE	full	limited (NIF/ports)	full	
TRAITS	concepts	protocols	interfaces/templates	
INIT	static blocks	@on_load	static this()	
PROCESS	threads/tasks	BEAM processes	threads/fibers	
TASK	futures/promises	Task	std.concurrency/futures	


7. Summary

You now have:

• Value vs identity equality
• CONST vs LET semantics
• Collection mutability rules
• Trait object rules
• Diagnostic catalog
• Backend capability matrix
