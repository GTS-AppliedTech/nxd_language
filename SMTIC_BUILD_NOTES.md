# SEMANTIC BUILD NOTES

### ASYNC / CONCURRENCY RUNTIME

CORE OPERATIONS
===============
AWAIT
SPAWN
SEND
RECV
TASK
CHANNEL
SYNC


AWAIT
=====
Context:
    ASYNC FUNC only

Target:
    Awaitable[T]

Return:
    T

Notes:
    Waits for completion of an awaitable operation.
    May wait indefinitely unless bounded by valid surrounding semantics.


SPAWN
=====
Context:
    ASYNC FUNC

Target:
    ASYNC function

Produces:
    ProcessHandle[T]


SEND
====
Context:
    ASYNC FUNC

Target:
    ProcessHandle[T]

Input:
    T

Return:
    none


RECV
====
Context:
    ASYNC FUNC

Target:
    Named Var/Const containing ProcessHandle[T]

Produces:
    ReceiveOperation[T]


TASK
====
Context:
    ASYNC FUNC

Target:
    Valid asynchronous operation/composition

Examples:
    TASK(SEND(...))
    TASK(RECV(...))
    TASK(SPAWN(...))
    TASK(AWAIT(...))
    TASK(PING(SEND(...)))
    TASK(PING(RECV(...)))

Produces:
    Task[T]

Rules:
    Nested operations are validated inside-out.
    Each nested operation owns its own diagnostic.
    A TASK wrapper does not change the validity rules of the
    operation contained inside it.

Diagnostic:
    S3005-12 Invalid TASK target


CHANNEL
=======
Context:
    TBD during CHANNEL semantic implementation

Purpose:
    Typed communication primitive used by async/concurrent operations.

Produces:
    Channel[T] — proposed, not yet implemented

Rules:
    CHANNEL owns validity of channel construction/declaration.
    SEND/RECV continue to own validity of their respective
    target/source relationships.

Diagnostic:
    S3005-11 Invalid CHANNEL declaration


SYNC
====
Context:
    ASYNC or synchronous context

Purpose:
    Explicit synchronization boundary.

Target:
    Synchronizable semantic value/operation.
    Exact accepted types TBD during SYNC implementation.

Rules:
    SYNC is not ASYNC-exclusive.
    S3005-06 does not apply merely because SYNC occurs outside
    an ASYNC FUNC.

Diagnostics:
    S3005-10 Invalid SYNC target within concurrency semantics.

    Future:
    S3006 reserved candidate for non-ASYNC synchronization
    violations once those semantics are formally defined.


OPTIONAL GUARDS / CONTROLLERS
============================

PING
====
Context:
    ASYNC FUNC

Direct Target:
    SEND
    RECV

Return:
    bool

Rules:
    PING validates only its immediate target.
    It does not search through wrappers for SEND or RECV.

Valid:
    PING(SEND(...))
    PING(RECV(...))
    TASK(PING(SEND(...)))

Invalid:
    PING(TASK(SEND(...)))
    PING(SPAWN(...))
    PING(AWAIT(...))

Diagnostic:
    S3005-07 Invalid PING target


ESC
===
Context:
    ASYNC FUNC

Current Target Types:
    ProcessHandle[T]
    ReceiveOperation[T]

Purpose:
    Attempt operation and abort/cancel when it cannot proceed
    according to defined ESC semantics.

Return:
    TBD

Compatibility with TASK, CHANNEL, and SYNC:
    TBD after their semantic boundaries are established.

Diagnostic:
    S3005-08 Invalid ESC target


TIMEOUT
=======
Context:
    ASYNC FUNC

Current Target Types:
    ProcessHandle[T]
    ReceiveOperation[T]

Additional Input:
    int duration

Purpose:
    Gives an eligible operation a bounded lifetime.

Return:
    TBD

Compatibility with TASK, CHANNEL, and SYNC:
    TBD after their semantic boundaries are established.

Diagnostic:
    S3005-09 Invalid TIMEOUT target


SEMANTIC TYPES
==============

Existing:
    Task[T]
    ProcessHandle[T]
    ReceiveOperation[T]

Planned / TBD:
    Channel[T]
    Synchronizable representation, if one is required


COMPOSITION RULE
================

Async/concurrency expressions are validated inside-out.

Each operation validates its immediate target according to its
own semantic rules.

An outer wrapper does not make an otherwise invalid inner
relationship valid.

Example:

    TASK(PING(SEND(...)))

Validation:
    SEND
      -> PING directly targets SEND
      -> TASK wraps the validated composition

Valid.

Example:

    PING(TASK(SEND(...)))

Validation:
    PING directly targets TASK

Invalid:
    S3005-07


DIAGNOSTIC OWNERSHIP
====================

S3005-01    AWAIT outside ASYNC
S3005-02    Invalid AWAIT target
S3005-03    Invalid SPAWN target
S3005-04    Invalid SEND target
S3005-05    Invalid RECV source
S3005-06    Operation requires valid ASYNC context
S3005-07    Invalid PING target
S3005-08    Invalid ESC target
S3005-09    Invalid TIMEOUT target
S3005-10    Invalid SYNC target
S3005-11    Invalid CHANNEL declaration
S3005-12    Invalid TASK target

Future candidate:
=================
S3006 reserved candidate for general (SYNC) synchronization violations
outside the ASYNC-specific diagnostic family.
Exact semantics intentionally TBD.