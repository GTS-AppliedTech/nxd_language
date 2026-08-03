OPERATE_SEMANTICS.md


NXD Operational Semantics

(Evaluation, Matching, Borrowing, Imports, Initialization, Runtime Ordering)

This chapter defines the precise execution rules of NXD—how expressions evaluate, how patterns match, how modules load, how borrows behave, and what guarantees the runtime provides.


1. Expression Evaluation Order

NXD enforces a strict left‑to‑right evaluation order for all expressions.

Rule

All subexpressions in NXD are evaluated left‑to‑right. This order is guaranteed and preserved in IR and all backends.

Example

LET X SET F() ADD G()


Evaluation steps:

1. Evaluate F()
2. Evaluate G()
3. Apply ADD


This applies to:

• binary operators
• function arguments
• pipeline expressions
• match guards
• list/map literals


Rationale

This ensures deterministic behavior across Nim, Elixir, and D, which otherwise differ in evaluation order.


2. Pattern Matching Exhaustiveness

Rule

Pattern matching over sum types (UNION, RESULT, OPTION) must be exhaustive. Non‑exhaustive matches are compile‑time errors.

Example (legal)

MATCH R:
    CASE OK(V): ...
    CASE ERR(E): ...


Example (illegal)

MATCH R:
    CASE OK(V): ...


Compile‑time error: missing ERR.

Non‑sum types

For non‑sum types (e.g., int, string):

• Non‑exhaustive matches are allowed.
• If no case matches and no OTHERWISE exists → runtime exception.


Rule

Non‑sum matches may omit cases; missing matches without OTHERWISE raise a runtime exception.


3. Runtime Ordering & Scheduling Guarantees

NXD defines semantic guarantees, not real‑time guarantees.

3.1 Channel Ordering

Messages sent to a channel are received in FIFO order.

This is the only strict ordering guarantee.

3.2 Process Scheduling

NXD guarantees cooperative fairness, not strict fairness.

• No starvation guarantees
• No priority guarantees
• No ordering guarantees between independent processes
• No guarantee that earlier SPAWN completes earlier


Rule

NXD guarantees FIFO per channel but does not guarantee global fairness, starvation prevention, or task prioritization.

3.3 Task Completion Ordering

AWAIT returns results in completion order, not spawn order.


4. Borrowing Semantics

NXD currently defines immutable borrows only.

Rule

BORROW creates an immutable borrow. Multiple immutable borrows of the same value are allowed.

Example (legal)

LET A SET BORROW X
LET B SET BORROW X


Mutable borrows

NXD reserves MUT_BORROW for a future extension.

Current spec:

• No mutable borrow restrictions
• No aliasing rules
• No exclusivity rules


Rationale

Strict Rust‑style borrow checking is incompatible with Elixir’s immutable heap and Nim/D’s GC/ARC/ORC unless implemented as a linting layer.


5. Cyclic Imports

Rule

Cyclic imports are allowed unless they create unsatisfiable initialization dependencies.

Legal

A imports B  
B imports A  


As long as:

• No cyclic type initialization
• No cyclic INIT dependency
• No top‑level executable code requiring the other module to be initialized first


Illegal

# A.nxd
MODULE A
IMPORT B
INIT:
    CALL B.START()

# B.nxd
MODULE B
IMPORT A
INIT:
    CALL A.START()


This is a compile‑time error.


6. INIT Block Execution Order

Rule

INIT blocks execute once at startup in topological dependency order.

Algorithm

1. Build module import graph
2. Topologically sort
3. Execute each module’s INIT after all its dependencies’ INIT blocks
4. If a cycle exists → compile‑time error


Example

MODULE A
INIT:
    PRINTLN("A")

MODULE B
IMPORT A
INIT:
    PRINTLN("B")


Execution order:

1. INIT A
2. INIT B


7. Summary Table

Topic	Rule	
Expression evaluation	Left‑to‑right, always	
Sum type matching	Must be exhaustive (compile‑time error if not)	
Non‑sum matching	Non‑exhaustive allowed; missing match → runtime exception	
Channel ordering	FIFO guaranteed	
Process scheduling	No fairness/starvation guarantees	
Task ordering	Completion order, not spawn order	
Borrowing	Unlimited immutable borrows allowed	
Cyclic imports	Allowed unless INIT cycle exists	
INIT execution	Dependency‑ordered, topological	


