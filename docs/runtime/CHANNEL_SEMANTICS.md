---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "RT001",
  "title": "",
  "description": "",
  "layer": "runtime",
  "category": "runtime",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# RT001 Channel Semantics Specification

(Creation, closing, boundedness, send/receive rules, error behavior, lifecycle)

NXD channels are the core primitive for inter‑process communication.
They provide typed, FIFO, message‑passing semantics across all backends (Nim, Elixir, D).

This chapter defines all rules governing channel behavior.


1. Channel identity and type

A channel is a first‑class typed value:

LET CH SET MAKE_CHANNEL<int>()


Properties:

• Typed — channels carry values of exactly one type.
• First‑class — channels can be passed, stored, sent, returned.
• Reference‑semantics — multiple processes may hold references to the same channel.


2. Channel creation

Syntax

LET CH SET MAKE_CHANNEL<T>()
LET CH SET MAKE_CHANNEL<T>(CAPACITY N)


Semantics

• Creates a new channel with type T.
• Default capacity is bounded, implementation‑defined (recommended: 64).
• Explicit capacity overrides default.


Backend mapping

• Nim → Channel[T]
• Elixir → mailbox abstraction or GenServer wrapper
• D → std.concurrency or fiber‑safe queue


3. Bounded vs unbounded channels

Bounded channels (default)

• Have a fixed capacity.
• SEND blocks or errors when full (see §5).


Unbounded channels

Created explicitly:

LET CH SET MAKE_CHANNEL<T>(UNBOUNDED)


Rules:

• Allowed but discouraged in safe NXD.
• Must be explicitly declared.
• Runtime may impose soft limits to prevent memory exhaustion.


4. FIFO ordering guarantee

Rule

Messages sent to a channel are received in strict FIFO order.

This is the only ordering guarantee NXD provides.

Implications

• No reordering across processes.
• No priority semantics.
• No fairness guarantees beyond FIFO per channel.


5. SEND semantics

Syntax

SEND V TO CH


Behavior

1. If channel is open and not full → enqueue message.
2. If channel is open but full:• bounded channel → runtime error (ERR("channel full"))
• unbounded channel → grow buffer

3. If channel is closed → runtime error (ERR("channel closed")).


Rule

Sending to a closed channel is always an error.

No implicit drop

NXD never silently drops messages.


6. RECV semantics

Syntax

LET V SET RECV CH


Behavior

1. If channel is open and non‑empty → dequeue message.
2. If channel is open and empty → block until message arrives.
3. If channel is closed:• If buffer has messages → dequeue remaining messages.
• If buffer empty → runtime error (ERR("channel closed")).



Rule

Receiving from a closed and empty channel is an error.


7. Channel closing

Syntax

CLOSE CH


Who can close?

• Any process holding a reference to the channel.
• Closing is idempotent:• closing an already closed channel → no effect.



Effects of closing

• No further sends allowed.
• Receivers may drain remaining messages.
• Once empty, further receives error.


Rule

Closing a channel does not kill processes. It only affects communication.


8. Channel lifecycle

States

1. Open
2. Closing (optional internal state)
3. Closed


Transitions

Open → Closed
Closed → Closed (idempotent)


Channels never reopen.


9. Error behavior summary

Operation	Open	Full	Closed	
SEND	enqueue	error (bounded)	error	
RECV	dequeue/block	N/A	drain or error	
CLOSE	close	close	no‑op	


10. Serialization of channels

Rule

Channels themselves are not serializable.

You cannot send a channel through another channel.

Reasons:

• Prevents accidental topology creation.
• Prevents capability leakage.
• Prevents backend mismatch (Elixir PIDs vs Nim objects vs D Tids).


Exception (future extension)

• Capability‑restricted “channel handles” may be allowed.


11. Channel and capability interaction

Channels may carry capabilities only if:

• the capability is serializable, and
• the channel type matches the capability type.


Example:

LET CH SET MAKE_CHANNEL<CAP_FS_READ>()
SEND CAP TO CH


If capability is non‑serializable → runtime error.


12. Channel fairness and scheduling

Rule

NXD does not guarantee fairness or starvation‑free scheduling.

Only FIFO per channel is guaranteed.

No guarantees about:

• which process receives first
• which process wakes first
• task/process priority
• starvation prevention


These are backend/runtime concerns.


13. Channel safety and auditability

All channel operations are:

• represented in IR
• visible to audit agents
• logged at runtime (optional policy)
• checked for capability violations (if carrying capabilities)


This allows:

• tracing message flow
• detecting misuse
• enforcing security policies
• verifying isolation guarantees


14. Examples

Example 1 — Basic send/receive

LET CH SET MAKE_CHANNEL<int>()

SPAWN:
    SEND 10 TO CH

LET X SET RECV CH   # X = 10


Example 2 — Closing

CLOSE CH

SEND 5 TO CH   # error


Example 3 — Draining after close

SEND 1 TO CH
SEND 2 TO CH
CLOSE CH

RECV CH   # 1
RECV CH   # 2
RECV CH   # error


Example 4 — Bounded channel full

LET CH SET MAKE_CHANNEL<int>(CAPACITY 1)

SEND 1 TO CH
SEND 2 TO CH   # error: channel full


15. Summary Table

Feature	Rule	
Ordering	FIFO per channel	
Closing	Allowed; idempotent	
Send after close	Error	
Receive after close	Drain then error	
Bounded	Default	
Unbounded	Allowed explicitly	
Serialization	Channels not serializable	
Capability flow	Allowed only if capability is serializable	
Fairness	Not guaranteed	
Starvation	Not prevented	
Reopening	Impossible	
