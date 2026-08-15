---
{
  "@context": "https://nxdlang.org/schema",
  "doc_id": "SE002",
  "title": "",
  "description": "",
  "layer": "security",
  "category": "security",
  "keywords": [],
  "doc_version": "",
  "status": "active"
}
---


# SE002 NXD Channel Safety Claim 

NXD enforces channel safety through typed message‑passing primitives.
Every channel is parameterized by a concrete type (CHANNEL<T>).

A typed channel constitutes a communication contract between producers and consumers.

Because NXD provides:

typed channels

- no untyped message passing
- no raw byte‑level send/receive
- no dynamic message casting
- no implicit serialization/deserialization

Incorrect message routing is statically detectable and cannot silently violate the established communication contract.

Therefore, NXD guarantees that communication invariants (channel → type mappings) remain preserved, and violations surface as type errors or analyzer findings rather than runtime failures.