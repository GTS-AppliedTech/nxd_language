---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "",
  "title": "",
  "description": "",
  "layer": "",
  "category": "",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---

# BE001 Backend Mapping Philosophy

The backend mapping philosophy defines how NXD’s single semantic model is preserved across three radically different target languages: Nim, Elixir, and D.
It ensures that NXD behaves consistently regardless of backend, while still leveraging each backend’s strengths.


1. NXD defines semantic constructs, not implementation constructs

NXD does not expose Nim’s threads, Elixir’s BEAM processes, or D’s fibers directly.

Instead, NXD defines:

• semantic processes
• semantic tasks
• semantic channels
• semantic types
• semantic errors
• semantic ownership rules


These constructs exist at the NXD level, not the backend level.

Backends implement these semantics using their own primitives.


2. NXD is actor‑leaning but backend‑agnostic

NXD’s concurrency model is intentionally closer to Elixir’s actor model:

• isolated processes
• message passing
• channels
• no implicit shared mutable state


But NXD does not require a BEAM runtime.

Backend mappings:

• Nim → async tasks, threads, channels
• Elixir → BEAM processes, mailboxes, Task
• D → threads, fibers, std.concurrency


The user sees one concurrency model.
The compiler chooses the best implementation per backend.


3. NXD IR is the “semantic truth”

NXD → AST → IR → Backend

The IR is:

• backend‑neutral
• typed
• normalized
• sugar‑free
• concurrency‑aware
• ownership‑aware


Backends never interpret NXD syntax directly—they interpret IR.

This guarantees:

• consistent semantics
• consistent concurrency
• consistent error handling
• consistent type behavior
• consistent memory rules


Across all backends.


4. Message passing is the preferred coordination model

NXD’s concurrency philosophy:

• Processes do not share mutable state by default.
• Channels are first‑class.
• Message passing is the preferred coordination mechanism.


Shared memory is allowed only via explicit constructs such as:

• UNSAFE blocks
• SHARED variables


This gives NXD:

• Elixir‑like safety
• Nim/D‑like performance
• A unified mental model for developers


5. NXD types are semantic types

NXD defines:

• primitives
• structs
• enums
• unions
• option
• result
• traits
• generics


These are NXD types, not backend types.

Backend mappings:

• Nim → object, enum, Result[T], Option[T], concept
• Elixir → defstruct, atoms, tagged tuples, protocols
• D → struct, enum, Algebraic!T, Nullable!T, interface, templates


The user writes one type system.
Backends implement it differently but consistently.


6. Unified error model

NXD defines:

• RESULT (ok/err)
• OPTION (some/none)
• TRY/CATCH/FINALLY


Backends map them as:

• Nim → Result[T], Option[T], try/except
• Elixir → {:ok, v}, {:error, msg}, try/rescue
• D → Result!T, Nullable!T, try/catch


The user sees one error handling philosophy, not three.


7. Unified memory model

NXD defines:

• stack vs heap semantics
• ownership (MOVE, CLONE, BORROW)
• lifetimes
• unsafe blocks


Backends map them as:

• Nim → ARC/ORC, GC, ref, ptr, deepCopy
• Elixir → immutable data, BEAM GC
• D → GC, RAII, pointers, .dup


NXD memory rules are semantic, not mechanical.


8. Unified concurrency model

NXD defines:

• SPAWN
• SEND
• RECV
• TASK
• AWAIT
• CHANNEL


Backends map them as:

• Nim → async tasks, threads, channels
• Elixir → BEAM processes, send, receive, Task.async
• D → threads, fibers, std.concurrency


One model → three implementations.


9. Backend quirks are hidden from the user

NXD hides:

• Nim’s GC modes
• Elixir’s scheduler
• D’s fiber stack sizes
• Nim’s asyncCheck
• Elixir’s GenServer
• D’s Tid quirks


The user writes NXD, not backend‑specific code.


10. NXD is semantically portable, not binary portable

NXD programs:

• behave the same
• produce the same results
• follow the same concurrency rules
• follow the same type rules
• follow the same error rules


Across all backends.

But they are not required to produce identical binaries or performance profiles.

NXD is a semantic language, not a bytecode language.


 One‑sentence summary

NXD defines one unified semantic model—types, concurrency, memory, errors—and each backend implements that model using its own native strengths.

