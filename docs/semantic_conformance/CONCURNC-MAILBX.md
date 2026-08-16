---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SC003",
  "title": "Concurrency and Mailbox Semantics",
  "description": "",
  "layer": "Semantic Conformance",
  "category": "Semantics",
  "keywords": [Semantics],
  "doc_version": "1.0",
  "status": "active"
}
---

# SC003 Concurrency and Mailbox Semantics

## Purpose

This specification defines NXD concurrency semantics, including SPAWN, SEND, RECV, AWAIT, process abstraction, mailbox behavior, failure propagation, and cancellation.

Concurrency is one of the largest semantic gaps across the current NXD backends because Nim, D, and Elixir expose different runtime models. NXD therefore defines concurrency at the language level. Backend primitives are implementation strategies, not semantic definitions.

## Normative Language

The terms MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, and MAY are used in their normative sense.

## 1. Process Model

An NXD process is a language-level unit of concurrent execution.

An NXD process is not necessarily:

- An operating-system process
- An operating-system thread
- A green thread
- A fiber
- A BEAM process
- A native async task

A backend MAY map an NXD process to any suitable target mechanism provided it preserves the guarantees required by this specification.

### 1.1 Process Identity

Each spawned NXD process SHALL have a process identity suitable for SEND, RECV, AWAIT, and cancellation where those operations are supported.

The representation of process identity is backend-specific, but the NXD-visible behavior MUST remain consistent.

### 1.2 Lightweight vs OS Thread vs BEAM Process

Backend mappings MAY include:

| Backend | Possible Mapping | Notes |
|---|---|---|
| Nim | async task, thread, channel worker, or runtime process wrapper | Mapping depends on runtime configuration |
| D | fiber, thread, task, or generated runtime worker | Mapping depends on concurrency library support |
| Elixir | BEAM process | Natural mapping for actor-like NXD process semantics |
| Native NXD Runtime | Native NXD scheduler process | Preferred long-term semantic baseline |

No backend mapping is automatically conformant merely because it compiles. It must preserve the NXD process semantics.

## 2. Mailbox Semantics

A mailbox is the message-receiving queue associated with an NXD process.

### 2.1 FIFO Ordering

Unless a future specification defines selective receive or priority rules, NXD mailboxes SHOULD preserve FIFO delivery for messages from the same sender to the same receiver.

If global FIFO across multiple senders is not guaranteed, the specification SHALL identify the permitted nondeterministic ordering domain.

### 2.2 Selective Receive

If NXD supports selective receive, the semantics MUST define:

- Whether earlier unmatched messages remain in the mailbox
- Whether matching scans preserve message order
- Whether selection is type-based, pattern-based, tag-based, or predicate-based
- Whether selective receive can starve unmatched messages
- How timeouts interact with selection

A backend MUST NOT silently substitute non-selective receive where selective receive is required.

### 2.3 Typed Messages

NXD message channels or mailboxes SHOULD preserve declared message types.

A backend MUST reject or safely handle messages that do not conform to the declared mailbox type rules.

### 2.4 Atomic Dequeue

RECV MUST remove a message atomically with respect to the receiving process's mailbox semantics.

A conformant implementation MUST NOT allow the same message to be received twice unless the specification explicitly defines broadcast, replay, or peeking behavior.

## 3. SEND Semantics

SEND transfers a message to the target process or mailbox according to the NXD message-passing rules.

### 3.1 Delivery Guarantees

NXD SHALL classify each SEND operation under one of the following delivery models:

- Best-effort delivery
- Local reliable enqueue
- Acknowledged delivery
- Supervised delivery
- Runtime-defined delivery

If the source construct does not specify a delivery mode, the default mode SHALL be defined by the language specification.

### 3.2 Ordering Guarantees

For a single sender sending multiple messages to the same receiver, SEND SHOULD preserve send order unless NXD explicitly permits nondeterminism.

For multiple senders, the interleaving MAY be nondeterministic unless the program uses a defined synchronization mechanism.

### 3.3 Ownership Transfer Through SEND

If a message contains owned values, SEND MUST follow the MOVE, BORROW, and CLONE rules defined in SC003.

The sender MUST NOT retain ownership of moved message payloads unless the payload is explicitly cloneable or shared under defined rules.

## 4. RECV Semantics

RECV retrieves a message from the current process's mailbox or an explicitly referenced mailbox according to NXD mailbox rules.

### 4.1 Blocking Rules

A blocking RECV waits until a matching message is available, cancellation occurs, failure propagates, or a timeout expires.

A non-blocking RECV returns immediately with an option-like or result-like value as defined by the language specification.

### 4.2 Matching Rules

Message matching MAY be based on:

- Message type
- Message tag
- Pattern structure
- Sender identity
- Capability authority
- User-defined predicate using permitted side-effect rules

Matching MUST NOT violate type safety or capability restrictions.

### 4.3 Timeout Rules

Timeout behavior MUST be explicit.

A timeout MUST NOT be reported as a successful message receive. It SHOULD produce a distinct absence, result, or cancellation state according to the NXD error model.

## 5. SPAWN Semantics

SPAWN creates a new NXD process.

### 5.1 Failure Propagation

The NXD specification SHALL define whether spawned process failure is:

- Isolated from the parent
- Linked to the parent
- Supervised
- Awaited explicitly
- Propagated through a result-like join operation

A backend MUST preserve the selected failure propagation behavior.

### 5.2 Supervision Model

If NXD defines supervision, the semantics SHALL specify:

- Supervisor ownership of child processes
- Restart behavior
- Escalation behavior
- Shutdown ordering
- Cancellation behavior
- Failure reporting

Backends MAY implement supervision through target-native primitives or generated runtime services.

### 5.3 Cancellation

Cancellation MUST have defined behavior.

The specification SHOULD define:

- Whether cancellation is cooperative or preemptive
- Whether cleanup handlers run
- Whether mailbox messages are preserved or discarded
- Whether owned resources are released
- Whether AWAIT observes cancellation as a distinct state

## 6. AWAIT Semantics

AWAIT observes completion of an asynchronous or concurrent operation.

AWAIT MUST distinguish, where applicable:

- Successful completion
- Result failure
- Raised error
- Process failure
- Cancellation
- Timeout

A backend MAY implement AWAIT using target-native async or join primitives, but MUST preserve NXD-visible completion behavior.

## 7. Backend Mapping Table

| Operation | Nim | D | Elixir | Required NXD Behavior |
|---|---|---|---|---|
| SPAWN | async task, thread, worker, or runtime process | fiber, thread, task, or generated worker | BEAM process | Creates NXD process identity and concurrency boundary |
| SEND | channel send, queue enqueue, runtime mailbox | channel send, queue enqueue, runtime mailbox | `send` or generated wrapper | Preserves message typing, ordering, and ownership rules |
| RECV | channel receive or mailbox runtime | channel receive or mailbox runtime | `receive` or generated wrapper | Preserves blocking, matching, timeout, and atomic dequeue semantics |
| AWAIT | await/join/future wait | fiber join/task wait/future wait | monitor, receive, Task.await, or runtime wrapper | Preserves completion and failure states |

## 8. Conformance Tests

The SC004 conformance suite SHALL include tests for:

- SPAWN creating an addressable process
- SEND followed by RECV
- Message ordering from one sender
- Interleaving from multiple senders within permitted nondeterminism
- Typed message acceptance and rejection
- Blocking RECV
- Non-blocking RECV
- RECV timeout
- Selective receive if supported
- Atomic dequeue
- Process failure propagation
- AWAIT success
- AWAIT failure
- Cancellation
- Race-condition boundaries
- Ownership transfer through message passing
- Capability restrictions in messages

## Normative Requirement

NXD concurrency SHALL be defined by language-level process, mailbox, SEND, RECV, SPAWN, and AWAIT semantics. A backend MUST preserve those semantics even when its native concurrency model differs.
