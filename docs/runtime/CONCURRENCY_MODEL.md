---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RT002",
  "title": "",
  "description": "",
  "layer": "runtime",
  "category": "runtime",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# RT002 CONCURRANCY MODEL

1. Core concepts

NXD defines three primary concurrency entities:

• Process:• The fundamental unit of execution in NXD.
• Has its own stack, message inbox, and local state.
• Maps to:• Nim: thread or async task
• Elixir: BEAM process
• D: thread or fiber


• Task:• A managed unit of work that returns a value.
• Often created to run a function concurrently and await its result.
• Maps to:• Nim: Future[T] / async proc
• Elixir: Task
• D: future/promise or custom wrapper over threads/fibers


• Channel:• A typed communication primitive for message passing.
• Used to send values between processes/tasks.
• Maps to:• Nim: Channel[T]
• Elixir: process mailbox (or custom GenServer abstraction)
• D: std.concurrency channels / Tid messaging


2. Concurrency primitives in NXD

At the language level, NXD exposes:

• SPAWN — create a new process/task:


SPAWN WORK()


• SEND — send a message to a channel or process:


SEND MSG TO CH


• RECV — receive a message from a channel:


LET V SET RECV CH


• TASK — create a managed task:


LET T SET TASK(FN)


• AWAIT — wait for a task’s result:


LET RESULT SET AWAIT T


These are semantic primitives—the compiler maps them to backend‑specific constructs.


3. Memory sharing vs message passing

Design stance:

• Default model: message passing via channels and process mailboxes.
• Shared memory: allowed but discouraged; treated as “advanced/unsafe” in the spec.


Rules (first pass):

• Processes do not share mutable state by default.
• Shared memory requires explicit opt‑in (e.g., unsafe or SHARED constructs in a future spec).
• Channels are first‑class and preferred for coordination.


This lets NXD feel closer to Elixir conceptually, while still mapping cleanly to Nim and D.


4. Backend mapping philosophy

User sees one model:

• Processes
• Tasks
• Channels
• SPAWN / SEND / RECV / AWAIT


Compiler maps to three:

• Nim:• SPAWN → async proc or thread
• TASK → Future[T]
• AWAIT → waitFor
• CHANNEL → Channel[T]

• Elixir:• SPAWN → spawn(fn -> ... end)
• TASK → Task.async/await
• SEND / RECV → send/receive
• CHANNEL → process mailbox or GenServer abstraction

• D:• SPAWN → Thread or Fiber
• TASK → future/promise abstraction
• SEND / RECV → std.concurrency.send/receive
• CHANNEL → Tid or custom channel type


5. What can share memory?

First‑pass rule set:

• Processes:• Conceptually isolated; no implicit shared mutable state.
• Any shared state must be explicitly modeled (e.g., via SHARED or unsafe in a future spec).

• Tasks:• Tasks spawned within the same process may share local immutable data.
• Mutable shared data is allowed only via explicit constructs.

• Channels:• Channels carry values, not references to shared mutable state (unless explicitly allowed by backend).



This gives you a security‑friendly default while still allowing low‑level control later.


6. BEAM vs OS threads semantics

NXD’s concurrency model is actor‑leaning, but backend‑agnostic:

• Conceptually, a process in NXD behaves like an Elixir process:• isolated state
• communicates via messages
• supervised via higher‑level constructs (future spec)

• Implementation‑wise:• Nim/D may use OS threads, fibers, or async tasks.
• Elixir uses BEAM processes directly.



The spec should emphasize:

“NXD defines semantic processes; backends choose the most appropriate implementation (actor, thread, fiber, async task) while preserving message‑passing semantics.”
