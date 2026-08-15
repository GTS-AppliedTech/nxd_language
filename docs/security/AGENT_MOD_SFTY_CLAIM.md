---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE001",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE001 AGENT MODIFICATION SAFTEY CLAIM

NXD does not claim that all agent‑generated behavior is automatically correct.
Instead, NXD claims that declared structural invariants are preserved or made statically detectable under agent modification.

Because NXD has:

- explicit types
- closed unions
- typed channels
- explicit modules
- actor‑style message passing
- /explicit error values
- no implicit shared memory

an agent‑generated edit that passes:

- parsing
- type checking
- channel checking
- module resolution
- match exhaustiveness checking
- policy analysis

cannot silently violate the system’s declared structural invariants without producing parser failures, type errors, analyzer findings, or policy‑gate violations.

Invariant violations surface through enforcement mechanisms rather than hidden runtime corruption.