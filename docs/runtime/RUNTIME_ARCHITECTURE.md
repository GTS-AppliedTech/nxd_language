RUNTIME_ARCHITECTURE.md


NXD runtime architecture


1. Separation of concerns

NXD is split into three major layers:

• Compiler• Parses, type‑checks, lowers to IR, and emits backend code.

• Runtime• Provides concurrency, scheduling, channels, tasks, and core services.

• Standard library• Collections, IO, math, crypto, utilities, built on top of the runtime.



The runtime is semantic: it defines behavior that each backend must implement.


2. Runtime core components

The NXD runtime defines these core subsystems:

• Scheduler• Manages processes and tasks.
• Abstracts over:• Nim: async event loop / threads
• Elixir: BEAM scheduler
• D: threads/fibers/event loop


• Process model• Semantic processes with:• local state
• inbox (messages)
• lifecycle (start/stop)

• Backend:• Nim: tasks/threads
• Elixir: processes
• D: threads/fibers


• Task system• Managed units of work that return values.
• Supports TASK and AWAIT.
• Backend:• Nim: Future[T]
• Elixir: Task
• D: futures/promises or custom wrapper


• Channel system• Typed channels for message passing.
• Supports MAKE_CHANNEL, SEND, RECV.
• Backend:• Nim: Channel[T]
• Elixir: mailboxes/GenServer abstractions
• D: std.concurrency / Tid


3. Runtime services

On top of the core, the runtime exposes:

• Timing• SLEEP, AFTER, TIMEOUT.

• Networking (future spec)• sockets, HTTP, streams.

• Security hooks• capability checks
• sandbox boundaries
• unsafe region tracking.

• Logging• structured logs for processes/tasks/errors.



These are runtime services, not just library functions—they may integrate with backend event loops and schedulers.


4. Relationship to the standard library

The standard library is built on top of the runtime:

• CONCURRENT module → uses scheduler, processes, tasks, channels.
• RESULT / OPTION → integrate with task and process error semantics.
• IO → uses runtime IO services.
• TIME → uses runtime timing services.
• SECURITY → uses runtime security hooks.


The rule of thumb:

Stdlib expresses behavior; runtime enforces behavior.


5. Compiler vs runtime boundary

The compiler is responsible for:

• turning NXD source into IR
• enforcing type and error rules
• generating backend code that calls into the runtime layer


The runtime is responsible for:

• actually scheduling work
• delivering messages
• managing tasks and channels
• enforcing concurrency and safety guarantees at execution time


Backends must provide a runtime shim that implements NXD’s runtime API in Nim/Elixir/D.


6. Backend runtime shims

Each backend has a thin runtime layer:

• Nim runtime shim• wraps asyncdispatch, channels, threads.
• exposes NXD‑style APIs: spawn, makeChannel, send, recv, task, await.

• Elixir runtime shim• wraps processes, send/receive, Task, supervision trees.
• exposes NXD‑style APIs with Elixir semantics.

• D runtime shim• wraps std.concurrency, threads, fibers.
• exposes NXD‑style APIs for processes, tasks, channels.



The NXD runtime spec defines what must exist; each shim defines how it’s implemented.


7. Error, memory, and security integration

The runtime is where your other specs meet:

• Error handling• task failures become RESULT ERR.
• process crashes can emit error messages.

• Memory management• unsafe regions are tracked at runtime.
• shared memory (if allowed) is mediated by runtime APIs.

• Security• capabilities and sandboxing enforced at runtime boundaries.
• audit agents can observe runtime events via IR‑linked metadata.


Short summary

NXD’s runtime is:

• a semantic layer defining processes, tasks, channels, scheduling, and core services,
• implemented per‑backend via runtime shims in Nim, Elixir, and D,
• and tightly integrated with your type, concurrency, error, memory, and security specs.
