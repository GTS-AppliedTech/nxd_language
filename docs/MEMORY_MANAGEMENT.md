---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RO003",
  "title": "",
  "description": "",
  "layer": "Root",
  "category": "Memory",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# RO003 MEMORY MANAGEMENT

1. Design stance

NXD sits on top of three different memory worlds:

• Nim → GC + manual + ARC/ORC
• D → GC + manual + RAII
• Elixir → BEAM GC, immutable data


So NXD’s memory model is:

• Semantically simple for the user
• Backend‑aware for the compiler
• Security‑conscious by design


2. Allocation model

Stack allocation

• Local variables (LET, CONST) are conceptually stack‑allocated.
• Backend:• Nim/D: real stack locals.
• Elixir: bindings in process heap (but treated as “local” in NXD spec).



Heap allocation

• Structs, collections, channels, tasks, and large objects are conceptually heap‑allocated.
• Backend:• Nim: GC/ARC/ORC or ref types.
• D: GC or new.
• Elixir: BEAM heap.


3. Lifetimes

NXD defines semantic lifetimes, not explicit annotations (first pass):

• Local lifetime: within a function/block.
• Process lifetime: tied to a process/actor.
• Task lifetime: tied to a task until completion.
• Channel lifetime: until explicitly closed or dropped.


Backends enforce actual lifetimes via their own mechanisms; NXD’s spec focuses on:

• “No use after free” at the semantic level.
• “No dangling references” in safe NXD code.


4. Ownership & borrowing

You already introduced:

• MOVE — transfer ownership.
• CLONE — deep copy.
• BORROW — temporary reference.


First‑pass semantics:

• MOVE:• After MOVE X, the original binding is considered logically invalid in NXD.
• Backend:• Nim/D: normal assignment, plus optional linting/static analysis.
• Elixir: no real move; treated as semantic hint.


• CLONE:• Creates a deep copy of the value.
• Backend:• Nim: deepCopy or custom.
• D: .dup or custom.
• Elixir: data is already immutable; clone is a no‑op or copy semantics.


• BORROW:• Temporary, non‑owning access.
• Backend:• Nim/D: ref or pointer semantics.
• Elixir: just another binding.




For now, these are semantic + linting tools, not hard Rust‑style rules—unless you decide later to enforce them.


5. GC and resource cleanup

GC interaction

• NXD does not define its own GC.
• It relies on:• Nim’s GC/ARC/ORC.
• D’s GC.
• Elixir’s BEAM GC.



Resource cleanup

• NXD encourages RAII‑style patterns via:


FUNC WITH_RESOURCE(R, FN):
    # acquire
    LET RES SET OPEN(R)
    # use
    LET RESULT SET FN(RES)
    # release
    CLOSE(RES)
    RETURN RESULT


• Backend:• Nim/D: destructors / defer / scope guards.
• Elixir: try/finally or supervision trees.


6. Unsafe and low‑level access

NXD should reserve an explicit unsafe block for later:

UNSAFE:
    RAW_MEMORY_ACCESS()


Semantics:

• Inside UNSAFE, the compiler relaxes memory safety guarantees.
• Backends:• Nim/D: pointer arithmetic, manual malloc/free, FFI.
• Elixir: NIFs or ports.



This gives you a clear line between safe NXD and systems‑level NXD, which is crucial for your security goals.


7. Security‑relevant guarantees (first pass)

• No implicit shared mutable state between processes.
• No manual free in safe NXD code.
• Ownership operations (MOVE, CLONE, BORROW) are visible in IR for audit agents.
• Unsafe blocks are explicitly marked and traceable.

