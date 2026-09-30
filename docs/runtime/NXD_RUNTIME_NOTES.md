{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RT006",
  "title": "RT006 NXD RUNTIME NOTES",
  "description": "NXD standalone runtime exploration/implementation",
  "layer": "runtime",
  "category": "runtime",
  "keywords": [nxd runtime, ASYNC, AWAIT, SEND, RECV, SPAWN],
  "doc_version": "1.0",
  "status": "active"
}
---

# RT006 NXD RUNTIME NOTES

## ASYNC FEATURE NOTES

### AWAIT

AWAIT evaluates an awaitable expression and suspends the enclosing ASYNC FUNC until the observed operation reaches a terminal state.

*Scalar Await:*
For a single awaitable of type Task[T], successful completion causes AWAIT to evaluate to a value of type T.

*Aggregate Await:*
For a collection of awaitables, AWAIT observes all supplied operations and produces their outcomes in completion order. Spawn order and input order do not determine output order.

AWAIT distinguishes successful completion, result failure, raised error, process failure, cancellation, and timeout.

An implementation may use backend-native async, task, process, fiber, thread, join, future, promise, or message-receive facilities, but it must preserve NXD-visible typing, terminal-state behavior, operation identity, and completion ordering.

##### Awaitable Definition

An awaitable expression represents an operation whose terminal state may be observed through AWAIT.

*An awaitable must provide:*

- Completion state
- Success value or outcome
- Failure information where applicable

##### Awaitable Invariant

Every awaitable has an identity, progresses toward a terminal state,
and may be observed through AWAIT.

AWAIT observes the terminal state of an awaitable without changing
the identity of the observed operation.

Completion order, cancellation, timeout, failure, and success do not
alter awaitable identity.

##### Initial Awaitable Types

- Task[T]
- ProcessHandle[T]
- ReceiveOperation[T]
- TimeoutOperation[T]
- AwaitGroup[T]

##### Initial Non-Awaitable Types

The following are not awaitable by default:

- Primitive values
- Array values
- Struct values
- Enum values
- Channel[T]
- Process definitions

AWAIT may only observe awaitable operations.

### Task[T]

Task[T] represents a managed asynchronous operation that will
eventually resolve to a value of type T.

A Task[T\]:

- Has an identity.
- Progresses toward a terminal state.
- Is awaitable.
- Preserves identity through observation.
- Resolves to T on successful completion.

AWAIT Task[T] -> T

##### Validated Awaitable Semantics

Task[T]
---------
AWAIT Task[T] -> T

ProcessHandle[T]
----------------
AWAIT ProcessHandle[T] -> T

Awaitables may be:
- assigned to variables
- referenced by name
- awaited through identity-preserving handles