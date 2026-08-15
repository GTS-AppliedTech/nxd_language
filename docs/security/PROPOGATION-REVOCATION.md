---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE008",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE008 capability propagation and revocation semantics

Here’s the clean spec for how capabilities exist, move, and die in NXD—so they stop being vibes and become law.


1. What a capability is

• Capability: a first‑class value that represents permission to perform a sensitive action.
• Examples:• CAP_FS_READ — read filesystem
• CAP_NET_CONNECT — open outbound connections
• CAP_PROC_SPAWN — spawn processes



Capabilities are not booleans or flags; they are typed values with rules.


2. Creation and scope

• Capabilities are created by:• the runtime at startup (root capabilities)
• secure factory functions (scoped capabilities)



LET CAP SET FS_READ_CAP()


• Each capability has:• type (what it allows)
• scope (where it applies: module, process, system)
• policy (copyable, delegable, revocable, serializable)


3. Propagation rules

3.1 Explicit propagation only

Capabilities do not automatically propagate.

FUNC READ_SECURE(FILE, CAP_FS_READ):
    ...

FUNC MAIN():
    LET CAP SET FS_READ_CAP()
    READ_SECURE("x", CAP)   # explicit


• When you SPAWN or TASK, you decide what capabilities to pass:


LET CAP SET FS_READ_CAP()

LET P SET SPAWN WORKER(CAP)
LET T SET TASK(WORKER, CAP)


No implicit inheritance from parent process/task.


3.2 Delegation

• Delegation = passing a capability value to another process/task.


SEND CAP TO CH


• Whether delegation is allowed depends on the capability’s policy:• delegable — can be passed freely
• non‑delegable — cannot leave its original holder



Attempting to delegate a non‑delegable capability → runtime error.


3.3 Copying

• Some capabilities are copyable, others are unique.


Policies:

• copyable: COPY(CAP) allowed; both references valid.
• unique: COPY(CAP) illegal; capability must move, not copy.


LET CAP2 SET COPY(CAP)   # legal only if copyable


3.4 Serialization through channels

• Capabilities may be sent through channels only if marked serializable.
• Non‑serializable capabilities cannot cross process boundaries.


Attempting to send a non‑serializable capability → runtime error.


4. Revocation semantics

4.1 Revocation operation

REVOKE(CAP)


• Marks the capability as invalid.
• After revocation:• any use of CAP fails (error or exception)
• all copies of that logical capability become invalid



Revocation is global for that capability instance, not just local.


4.2 Revocation propagation

• If a capability was delegated or copied:• revoking the original also revokes all derived instances.

• The runtime tracks capability identity, not just value.


4.3 Use after revocation

Using a revoked capability:

READ_SECURE("x", CAP)   # after REVOKE(CAP)


• Results in:• ERR("capability revoked"), or
• runtime exception (depending on API design)


5. Capability and processes/tasks

5.1 Process startup

• A process starts with:• capabilities explicitly passed at SPAWN
• possibly a default minimal set (e.g., logging)



5.2 Task startup

• Same rule: only capabilities explicitly passed at TASK are available.


5.3 No ambient authority

There is no ambient authority in NXD safe code.
All sensitive operations require explicit capability values.


6. Security and auditability

• Capability operations (create, delegate, copy, revoke, use) are:• represented in IR
• visible to audit agents
• loggable at runtime



This lets you trace:

• who had what
• when it was used
• when it was revoked
• how it flowed through channels and processes


7. Summary table

Aspect	Rule	
Propagation	Explicit only; no implicit inheritance	
Delegation	Allowed only if policy permits	
Copying	Policy‑controlled (copyable vs unique)	
Serialization	Only if capability is serializable	
Revocation	Global for that capability instance	
Use after revoke	Error/exception	
Ambient authority	None in safe NXD	

