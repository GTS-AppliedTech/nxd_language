# SEMANTIC BUILD NOTES

### ASYNC / CONCURRENCY RUNTIME

#### ASYNC EXECUTION OPERATIONS
==========================
AWAIT
SPAWN
SEND
RECV
TASK

### ASYNC CONTEXT BOUNDARY
==========================

S3005-06 applies only to operations whose semantics require
active asynchronous execution.

ASYNC-context operations:
    SPAWN
    SEND
    RECV
    TASK
    PING
    ESC
    TIMEOUT

Special case:
    AWAIT is ASYNC-only but owns the more specific S3005-01
    diagnostic when used outside ASYNC.

Concurrency operations not inherently ASYNC-exclusive:
    CHANNEL
    SYNC

Rules:
    Membership in the concurrency subsystem does not make an
    operation ASYNC-only.

    CHANNEL may establish communication infrastructure outside
    active asynchronous execution.

    SYNC may establish a synchronization/reconciliation boundary
    in either synchronous or asynchronous execution.

    S3005-06 applies only where the operation itself requires
    ASYNC execution.

#### CONCURRENCY BOUNDARY OPERATIONS
===============================
CHANNEL
SYNC

##### AWAIT
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


##### SPAWN
=====
Context:
    ASYNC FUNC

Target:
    ASYNC function

Produces:
    ProcessHandle[T]


##### SEND
====
Context:
    ASYNC FUNC

Target:
    ProcessHandle[T]

Input:
    T

Return:
    none


##### RECV
====
Context:
    ASYNC FUNC

Target:
    Named Var/Const containing ProcessHandle[T]

Produces:
    ReceiveOperation[T]


##### TASK
====
Context:
    ASYNC FUNC only

Target:
    Exactly one valid asynchronous operation/composition

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
    TASK requires exactly one target.
    TASK targets must represent valid asynchronous work/composition.
    Nested operations are validated inside-out.
    Each nested operation owns its own diagnostic.
    A TASK wrapper does not make an invalid inner relationship valid.

Parser:
    TASK is parsed as an expression call.
    Arity errors are intentionally forwarded to semantic validation.

Diagnostic:
    S3005-12 Invalid TASK target

Current Tests:
    1 positive
    1 negative

##### CHANNEL
=======
Context:
    ASYNC or synchronous context

Purpose:
    Declares/creates typed communication infrastructure used by
    asynchronous and concurrent operations.

Produces:
    Channel[T] — proposed, not yet implemented

Rules:
    CHANNEL declaration itself does not require ASYNC context.

    A CHANNEL may be declared, stored, or passed independently
    of active asynchronous execution.

    Operations that actively communicate through a CHANNEL remain
    subject to their own ASYNC-context and target/source rules.

    CHANNEL owns validity of channel construction/declaration.

    SEND/RECV continue to own validity of their respective
    target/source relationships.

    S3005-06 does not apply merely because CHANNEL is declared
    outside an ASYNC FUNC.

Diagnostic:
    S3005-11 Invalid CHANNEL declaration

Future: 
    S3007 reserved for general channel violations outside 
    the ASYNC-specific diagnostic family. 


##### SYNC
====
Context:
    ASYNC or synchronous context

Purpose:
    Explicit synchronization/reconciliation boundary.

Rules:
    SYNC is not equivalent to AWAIT.

    AWAIT is completion-oriented and unwraps an awaitable result.

    SYNC is state/synchronization-oriented and does not become
    invalid merely because it occurs outside an ASYNC FUNC.

    S3005-06 does not apply merely because SYNC occurs outside
    an ASYNC FUNC.

    Current ASYNC/concurrency target validation is owned by
    S3005-10.

    Broader non-ASYNC synchronization semantics remain TBD.

Diagnostic:
    S3005-10 Invalid SYNC target

Current Tests:
    2 positive
    2 negative

Future:
    S3006 reserved for general synchronization violations outside
    the ASYNC-specific diagnostic family.


#### OPTIONAL GUARDS / CONTROLLERS
============================

##### PING
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


##### ESC
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


##### TIMEOUT
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


### SEMANTIC TYPES
==============

Existing:
    Task[T]
    ProcessHandle[T]
    ReceiveOperation[T]

Planned:
    Channel[T]

Future / TBD:
    Additional synchronization state/result representation,
    only if required by final SYNC semantics.

### COMPOSITION RULE
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


### DIAGNOSTIC OWNERSHIP
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

### Future candidates:
=================
S3006 reserved candidate for general (SYNC) synchronization violations
outside the ASYNC-specific diagnostic family.
Exact semantics intentionally TBD.
S3007 reserved candidate for general CHANNEL violations outside the
ASYNC-specific diagnostic family.
Exact semantics intentionally TBD.