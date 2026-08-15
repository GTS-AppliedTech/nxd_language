---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RT003",
  "title": "",
  "description": "",
  "layer": "runtime",
  "category": "runtime",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# RT003 execution model

Here’s the spec‑level Execution Model—how an NXD program actually runs, from startup to shutdown.


1. Program startup

Entry point:

MODULE app.main

FUNC MAIN():
    ...


• The compiler designates MAIN in app.main (or configured module) as the entry function.
• Program startup sequence:


1. Load all modules reachable from MAIN.
2. Resolve imports and build the module graph.
3. Execute INIT blocks in dependency order.
4. Call MAIN().


2. Module loading and initialization

Loading:

• Modules are loaded on demand based on imports.
• Each module is loaded at most once.


Initialization:

• Each module may define:


INIT:
    ...


• INIT executes:• once per module
• after all imported modules’ INIT have run

• Cyclic INIT dependencies → compile‑time error.


3. Process and task lifecycle

Process creation:

SPAWN FN()


• Creates a new semantic process.
• Process has:• local state
• inbox (messages)
• lifecycle (running → terminated)



Task creation:

LET T SET TASK(FN)
LET R SET AWAIT T


• TASK wraps work in a managed unit that returns a value.
• AWAIT blocks until the task completes and returns:• OK(value) or ERR(error) depending on your error model.



Lifecycle:

• Processes and tasks terminate when:• their function returns, or
• an uncaught exception occurs.


4. Message passing and channels

Channel creation:

LET CH SET MAKE_CHANNEL()


Sending:

SEND MSG TO CH


Receiving:

LET V SET RECV CH


Guarantees:

• Per channel, messages are delivered in FIFO order.
• No global ordering guarantees across channels or processes.


5. Error propagation

Within a process/task:

• RESULT and OPTION are used for recoverable/optional cases.
• THROW unwinds the stack to the nearest TRY/CATCH.


Across tasks:

• If a task fails:• AWAIT T returns ERR("...") (or raises, depending on your chosen mapping).



Across processes:

• Process‑level failures may:• terminate the process
• optionally emit error messages to supervising processes (future spec)


6. Program shutdown

A program terminates when:

• MAIN() returns and:• all non‑daemon processes have completed, and
• all tasks have either completed or been cancelled.



Shutdown sequence (conceptual):

1. Stop accepting new work.
2. Allow in‑flight tasks/processes to complete or be cancelled.
3. Release resources (channels, files, network, etc.).
4. Exit with:• success code, or
• error code if an uncaught exception reached the top level.


In short: NXD’s execution model is:

• INIT graph → MAIN → processes/tasks → channels/messages → shutdown,
with strict left‑to‑right evaluation, FIFO per channel, and explicit error semantics layered on top.
